use std::error;

use x11rb::{
    connection::Connection, protocol::randr::ConnectionExt, rust_connection::RustConnection,
};

use x11rb::protocol::randr::{self, Crtc, GetCrtcInfoReply};

use crate::help::Help;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Position {
    Left,
    Right,
    Above,
    Below,
    Center,
}

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub conn: RustConnection,
    pub root_window: u32,
    pub monitors: Vec<Monitor>,
    pub selected_monitor: Option<String>,
    pub help: Help,
}

#[derive(Debug)]
pub struct Monitor {
    pub name: String,
    pub height: u16,
    pub width: u16,
    pub primary: bool,
    pub new_coordinates: Option<Coordinates>,
    pub crtc_info: GetCrtcInfoReply,
    pub crtc: Crtc,
}

#[derive(Debug, Default, Clone)]
pub struct Coordinates {
    pub x: i16,
    pub y: i16,
}

impl App {
    pub fn new() -> AppResult<Self> {
        let (conn, screen_num) = RustConnection::connect(None)?;
        let root_window = conn.setup().roots[screen_num].root;

        Ok(Self {
            running: true,
            conn,
            root_window,
            monitors: Vec::new(),
            selected_monitor: None,
            help: Help::new(),
        })
    }

    pub fn get_all_connected_monitors(&self) -> AppResult<Vec<Monitor>> {
        let mut monitors: Vec<Monitor> = Vec::new();

        let resources = self
            .conn
            .randr_get_screen_resources_current(self.root_window)?
            .reply()?;

        let discovered_monitors = self
            .conn
            .randr_get_monitors(self.root_window, false)?
            .reply()?
            .monitors;

        for output in resources.outputs {
            let info = self.conn.randr_get_output_info(output, 0)?.reply()?;

            if info.connection == randr::Connection::CONNECTED {
                let name = String::from_utf8_lossy(&info.name);
                if let Some(monitor) = discovered_monitors
                    .iter()
                    .find(|&monitor| monitor.outputs.contains(&output))
                {
                    let crtc_info = self
                        .conn
                        .randr_get_crtc_info(info.crtc, resources.config_timestamp)?
                        .reply()?;

                    monitors.push(Monitor {
                        name: name.to_string(),
                        height: monitor.height,
                        width: monitor.width,
                        primary: monitor.primary,
                        new_coordinates: None,
                        crtc_info,
                        crtc: info.crtc,
                    })
                }
            }
        }
        Ok(monitors)
    }

    pub fn tick(&mut self) -> AppResult<()> {
        let refreshed_monitores = self.get_all_connected_monitors()?;

        let names = {
            let mut names: Vec<String> = Vec::new();

            for monitor in self.monitors.iter() {
                if !refreshed_monitores.iter().any(|m| m.name == monitor.name) {
                    names.push(monitor.name.clone());
                }
            }

            names
        };

        // Remove unplugged monitors
        for name in names {
            self.monitors.retain(|c| c.name != name);
        }

        for refreshed_monitor in refreshed_monitores {
            if let Some(monitor) = self
                .monitors
                .iter_mut()
                .find(|c| c.name == refreshed_monitor.name)
            {
                monitor.crtc_info = refreshed_monitor.crtc_info
            } else {
                self.monitors.push(refreshed_monitor);
            }
        }

        if self.selected_monitor.is_none() {
            if let Some(monitor) = self.monitors.iter().find(|monitor| !monitor.primary) {
                self.selected_monitor = Some(monitor.name.clone());
            }
        }

        Ok(())
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
