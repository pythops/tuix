use std::error;

use crate::{
    help::Help,
    notification::Notification,
    xrandr::{Screen, Xrandr},
};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Default)]
pub struct App {
    pub running: bool,
    pub notifications: Vec<Notification>,
    pub screens: Vec<Screen>,
    pub help: Help,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            ..Default::default()
        }
    }

    pub fn get_all_screens(&self) -> Vec<Screen> {
        Xrandr::get_screens()
    }

    pub fn tick(&mut self) {
        self.notifications.retain(|n| n.ttl > 0);
        self.notifications.iter_mut().for_each(|n| n.ttl -= 1);

        let screens = self.get_all_screens();

        let names = {
            let mut names: Vec<String> = Vec::new();

            for screen in self.screens.iter() {
                if !screens.iter().any(|m| m.name == screen.name) {
                    names.push(screen.name.clone());
                }
            }

            names
        };

        // Remove unplugged monitors
        for name in names {
            self.screens.retain(|c| c.name != name);
        }

        for screen in screens {
            if let Some(s) = self.screens.iter_mut().find(|c| c.name == screen.name) {
                s.is_primary = screen.is_primary;
                s.resolution = screen.resolution;
                s.position = screen.position;
            } else {
                self.screens.push(screen);
            }
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
