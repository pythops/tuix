use std::rc::Rc;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    if !app.monitors.is_empty() {
        let nb_blocks = app.monitors.len() * 2 + 1;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints({
                (0..nb_blocks)
                    .map(|_| Constraint::Ratio(1, nb_blocks as u32))
                    .collect::<Vec<Constraint>>()
            })
            .split(frame.size());

        let chunks = chunks
            .iter()
            .map(|chunk| {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints({
                        (0..nb_blocks)
                            .map(|_| Constraint::Ratio(1, nb_blocks as u32))
                            .collect::<Vec<Constraint>>()
                    })
                    .split(*chunk)
            })
            .collect::<Vec<Rc<[Rect]>>>();

        let chunks: Rc<[Rect]> = chunks
            .into_iter()
            .flat_map(|rect| rect.iter().cloned().collect::<Vec<Rect>>())
            .collect();

        let primary_monitor = app.monitors.iter().find(|monitor| monitor.primary);

        for monitor in app.monitors.iter() {
            let text = vec![
                Line::from(monitor.name.clone()),
                Line::from(format!("{}x{}", monitor.width, monitor.height)),
            ];

            let paragraph = Paragraph::new(text)
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_style({
                            if let Some(selected_monitor) = &app.selected_monitor {
                                if selected_monitor == &monitor.name {
                                    Style::default().green()
                                } else {
                                    Style::default()
                                }
                            } else {
                                Style::default()
                            }
                        })
                        .border_type({
                            if let Some(selected_monitor) = &app.selected_monitor {
                                if selected_monitor == &monitor.name {
                                    BorderType::Thick
                                } else {
                                    BorderType::default()
                                }
                            } else {
                                BorderType::default()
                            }
                        }),
                )
                .style(Style::new().white())
                .centered()
                .wrap(Wrap { trim: true });

            if monitor.primary {
                let paragraph = paragraph.style(Style::default().blue());
                frame.render_widget(paragraph, chunks[chunks.len() / 2]);
            } else {
                let (monitor_x, monitor_y) = {
                    if let Some(coordinates) = &monitor.new_coordinates {
                        (coordinates.x, coordinates.y)
                    } else {
                        (monitor.crtc_info.x, monitor.crtc_info.y)
                    }
                };

                let (primary_x, primary_y) = {
                    let primary = primary_monitor.unwrap();
                    if let Some(coordinates) = &primary.new_coordinates {
                        (coordinates.x, coordinates.y)
                    } else {
                        (primary.crtc_info.x, primary.crtc_info.y)
                    }
                };

                if monitor_x > primary_x {
                    let position = chunks.len() / 2 + 1;
                    frame.render_widget(paragraph.clone(), chunks[position]);
                }

                if monitor_x < primary_x {
                    let position = chunks.len() / 2 - 1;
                    frame.render_widget(paragraph.clone(), chunks[position]);
                }

                if monitor_y < primary_y {
                    let position = chunks.len() / 2 - nb_blocks;
                    frame.render_widget(paragraph.clone(), chunks[position]);
                }

                if monitor_y > primary_y {
                    let position = chunks.len() / 2 + nb_blocks;
                    frame.render_widget(paragraph.clone(), chunks[position]);
                }
            }
        }
    }

    if app.help.show_help {
        app.help.render(frame);
    }
}
