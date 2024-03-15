use crate::app::{App, AppResult, Coordinates};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use x11rb::protocol::randr::ConnectionExt;

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
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
                for monitor in app.monitors.iter_mut() {
                    monitor.new_coordinates = None
                }
            }
        }

        KeyCode::Char('?') => {
            app.help.show_help = true;
        }

        KeyCode::Enter => {
            if let Some(secondary_monitor) = app.monitors.iter().find(|m| !m.primary) {
                if let Some(primary_monitor) = app.monitors.iter().find(|m| m.primary) {
                    if let Some(secondary_monitor_new_coordinates) =
                        &secondary_monitor.new_coordinates
                    {
                        let primary_monitor_new_coordinates =
                            primary_monitor.new_coordinates.as_ref().unwrap();

                        let mut primary_crtc = primary_monitor.crtc_info.clone();
                        let mut secondary_crtc = secondary_monitor.crtc_info.clone();

                        primary_crtc.x = primary_monitor_new_coordinates.x;
                        primary_crtc.y = primary_monitor_new_coordinates.y;

                        secondary_crtc.x = secondary_monitor_new_coordinates.x;
                        secondary_crtc.y = secondary_monitor_new_coordinates.y;

                        app.conn
                            .randr_set_crtc_config(
                                primary_monitor.crtc,
                                0,
                                0,
                                primary_crtc.x,
                                primary_crtc.y,
                                primary_crtc.mode,
                                primary_crtc.rotation,
                                &primary_monitor.crtc_info.outputs,
                            )?
                            .reply()?;

                        app.conn
                            .randr_set_crtc_config(
                                secondary_monitor.crtc,
                                0,
                                0,
                                secondary_crtc.x,
                                secondary_crtc.y,
                                secondary_crtc.mode,
                                secondary_crtc.rotation,
                                &secondary_monitor.crtc_info.outputs,
                            )?
                            .reply()?;
                    }
                }
            }
        }

        KeyCode::Char('l') => {
            let primary_monitor = app.monitors.iter().enumerate().find(|(_, m)| m.primary);
            let secondary_monitor = app.monitors.iter().enumerate().find(|(_, m)| !m.primary);

            if let Some((secondary_monitor_index, _)) = secondary_monitor {
                if let Some((primary_monitor_index, _)) = primary_monitor {
                    app.monitors[primary_monitor_index].new_coordinates =
                        Some(Coordinates { x: 0, y: 0 });

                    app.monitors[secondary_monitor_index].new_coordinates = Some(Coordinates {
                        x: app.monitors[primary_monitor_index].width as i16,
                        y: 0,
                    });
                }
            }
        }

        KeyCode::Char('k') => {
            let primary_monitor = app.monitors.iter().enumerate().find(|(_, m)| m.primary);
            let secondary_monitor = app.monitors.iter().enumerate().find(|(_, m)| !m.primary);

            if let Some((secondary_monitor_index, _)) = secondary_monitor {
                if let Some((primary_monitor_index, _)) = primary_monitor {
                    app.monitors[primary_monitor_index].new_coordinates = Some(Coordinates {
                        x: 0,
                        y: app.monitors[secondary_monitor_index].height as i16,
                    });

                    app.monitors[secondary_monitor_index].new_coordinates =
                        Some(Coordinates { x: 0, y: 0 });
                }
            }
        }

        KeyCode::Char('j') => {
            let primary_monitor = app.monitors.iter().enumerate().find(|(_, m)| m.primary);
            let secondary_monitor = app.monitors.iter().enumerate().find(|(_, m)| !m.primary);
            if let Some((secondary_monitor_index, _)) = secondary_monitor {
                if let Some((primary_monitor_index, _)) = primary_monitor {
                    app.monitors[primary_monitor_index].new_coordinates =
                        Some(Coordinates { x: 0, y: 0 });

                    app.monitors[secondary_monitor_index].new_coordinates = Some(Coordinates {
                        x: 0,
                        y: app.monitors[primary_monitor_index].height as i16,
                    });
                }
            }
        }

        KeyCode::Char('h') => {
            let primary_monitor = app.monitors.iter().enumerate().find(|(_, m)| m.primary);
            let secondary_monitor = app.monitors.iter().enumerate().find(|(_, m)| !m.primary);
            if let Some((secondary_monitor_index, _)) = secondary_monitor {
                if let Some((primary_monitor_index, _)) = primary_monitor {
                    app.monitors[primary_monitor_index].new_coordinates = Some(Coordinates {
                        x: app.monitors[secondary_monitor_index].width as i16,
                        y: 0,
                    });

                    app.monitors[secondary_monitor_index].new_coordinates =
                        Some(Coordinates { x: 0, y: 0 });
                }
            }
        }

        _ => {}
    }
    Ok(())
}
