use std::{cmp::Ordering, rc::Rc};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

const SCREEN_NUMBER: usize = 3;

pub fn render(app: &mut App, frame: &mut Frame) {
    if !app.screens.is_empty() {
        let nb_blocks = SCREEN_NUMBER;

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

        for screen in app.screens.iter() {
            let text = vec![
                Line::from(screen.name.clone()),
                Line::from(format!("{}x{}", screen.resolution.0, screen.resolution.1)),
            ];

            let paragraph = Paragraph::new(text)
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_style(Style::default())
                        .border_type(BorderType::default()),
                )
                .style(Style::new().white())
                .centered()
                .wrap(Wrap { trim: true });

            if screen.is_primary {
                let paragraph = paragraph.style(Style::default().blue());
                frame.render_widget(paragraph, chunks[chunks.len() / 2]);
            } else {
                let (screen_x, screen_y) = {
                    if let Some(position) = &screen.new_position {
                        (position.0, position.1)
                    } else {
                        (screen.position.0, screen.position.1)
                    }
                };

                let (primary_x, primary_y) = {
                    let primary = app.screens.iter().find(|screen| screen.is_primary).unwrap();
                    if let Some(postion) = &primary.new_position {
                        (postion.0, postion.1)
                    } else {
                        (primary.position.0, primary.position.1)
                    }
                };

                match screen_x.cmp(&primary_x) {
                    Ordering::Greater => {
                        let position = chunks.len() / 2 + 1;
                        frame.render_widget(paragraph.clone(), chunks[position]);
                    }
                    Ordering::Less => {
                        let position = chunks.len() / 2 - 1;
                        frame.render_widget(paragraph.clone(), chunks[position]);
                    }
                    Ordering::Equal => {
                        if screen_y < primary_y {
                            let position = chunks.len() / 2 - nb_blocks;
                            frame.render_widget(paragraph.clone(), chunks[position]);
                        }

                        if screen_y > primary_y {
                            let position = chunks.len() / 2 + nb_blocks;
                            frame.render_widget(paragraph.clone(), chunks[position]);
                        }
                    }
                }
            }
        }
    }

    if app.help.show_help {
        app.help.render(frame);
    }

    // Notifications
    for (index, notification) in app.notifications.iter().enumerate() {
        notification.render(index, frame);
    }
}
