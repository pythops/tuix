use std::process::Command;

use regex::Regex;

use crate::app::AppResult;
pub struct Xrandr;

#[derive(Debug, Default)]
pub struct Screen {
    pub name: String,
    pub is_primary: bool,
    pub resolution: (u16, u16),
    pub position: (u16, u16),
    pub new_position: Option<(u16, u16)>,
    pub location: Option<Location>,
}

#[derive(Debug)]
pub enum Location {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

impl Xrandr {
    pub fn get_screens() -> Vec<Screen> {
        let output = Command::new("xrandr").output().unwrap();

        let stdout = String::from_utf8_lossy(&output.stdout);

        let re_connected =
            Regex::new(r"^(\S+)\sconnected\s(primary\s)?(\d+)x(\d+)\+(\d+)\+(\d+)?").unwrap();

        let mut screens: Vec<Screen> = Vec::new();

        for line in stdout.lines() {
            if let Some(cap) = re_connected.captures(line) {
                let name = cap[1].to_owned();
                let is_primary = cap.get(2).map_or(false, |_| true);
                let width: u16 = cap[3].parse().unwrap();
                let height: u16 = cap[4].parse().unwrap();
                let x: u16 = cap[5].parse().unwrap();
                let y: u16 = cap[6].parse().unwrap();

                screens.push(Screen {
                    name,
                    is_primary,
                    resolution: (width, height),
                    position: (x, y),
                    ..Default::default()
                })
            }
        }

        screens
    }

    pub fn arrange(
        position: &Location,
        primary_screen_name: &str,
        screen_name: &str,
    ) -> AppResult<()> {
        let postion_arg_name = match position {
            Location::UP => "--above",
            Location::DOWN => "--below",
            Location::LEFT => "--left-of",
            Location::RIGHT => "--right-of",
        };

        Command::new("xrandr")
            .args([
                "--auto",
                "--output",
                primary_screen_name,
                "--auto",
                "--output",
                screen_name,
                postion_arg_name,
                primary_screen_name,
            ])
            .output()?;

        Ok(())
    }
}
