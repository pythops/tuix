use std::sync::mpsc::Sender;

use crate::{
    app::{App, AppResult},
    event::Event,
    notification::Notification,
    xrandr::Xrandr,
};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_events(
    key_event: KeyEvent,
    app: &mut App,
    sender: Sender<Event>,
) -> AppResult<()> {
    match key_event.code {
        KeyCode::Char('q') => {
            app.quit();
        }

        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }

        KeyCode::Esc => {
            if app.help.show_help {
                app.help.show_help = false;
            } else {
                app.screens.iter_mut().for_each(|screen| {
                    screen.new_position = None;
                    screen.location = None;
                });
            }
        }

        KeyCode::Char('?') => {
            app.help.show_help = true;
        }

        KeyCode::Enter => {
            let primary_screen_name = {
                let screen = app.screens.iter().find(|screen| screen.is_primary);
                screen.unwrap().name.as_ref()
            };

            if let Some(screen) = app.screens.iter().find(|screen| !screen.is_primary) {
                if let Some(location) = &screen.location {
                    if let Err(e) =
                        Xrandr::arrange(location, primary_screen_name, screen.name.as_ref())
                    {
                        Notification::send(
                            e.to_string(),
                            crate::notification::NotificationLevel::Error,
                            sender,
                        )?;
                    }
                }
            }
        }

        KeyCode::Char('l') => {
            let mut screens_iter = app.screens.iter_mut();

            let primary_screen = screens_iter.find(|screen| screen.is_primary).unwrap();

            if let Some(screen) = screens_iter.find(|screen| !screen.is_primary) {
                screen.new_position = Some((primary_screen.resolution.0, 0));
                primary_screen.new_position = Some((0, 0));
                screen.location = Some(crate::xrandr::Location::RIGHT);
            }
        }

        KeyCode::Char('k') => {
            let mut screens_iter = app.screens.iter_mut();
            let primary_screen = screens_iter.find(|screen| screen.is_primary).unwrap();

            if let Some(screen) = screens_iter.find(|screen| !screen.is_primary) {
                screen.new_position = Some((0, 0));
                primary_screen.new_position = Some((0, screen.resolution.1));
                screen.location = Some(crate::xrandr::Location::UP);
            }
        }

        KeyCode::Char('j') => {
            let mut screens_iter = app.screens.iter_mut();
            let primary_screen = screens_iter.find(|screen| screen.is_primary).unwrap();

            if let Some(screen) = screens_iter.find(|screen| !screen.is_primary) {
                screen.new_position = Some((0, primary_screen.resolution.1));
                primary_screen.new_position = Some((0, 0));
                screen.location = Some(crate::xrandr::Location::DOWN);
            }
        }

        KeyCode::Char('h') => {
            let mut screens_iter = app.screens.iter_mut();
            let primary_screen = screens_iter.find(|screen| screen.is_primary).unwrap();

            if let Some(screen) = screens_iter.find(|screen| !screen.is_primary) {
                screen.new_position = Some((0, 0));
                primary_screen.new_position = Some((screen.resolution.0, 0));
                screen.location = Some(crate::xrandr::Location::LEFT);
            }
        }

        _ => {}
    }
    Ok(())
}
