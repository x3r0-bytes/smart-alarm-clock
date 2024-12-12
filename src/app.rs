use iced::{Theme, Command};
use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
}

pub struct WeatherForcast5Days {
    pub name: String,
    pub isdaytime: bool,
    pub temperature: i16,
    pub temp_unit: String,
    pub prob_of_precipt: u8,
    pub wind_speed: i16,
    pub wind_dir: String,
    pub short_forcast: String,
    pub det_forcast: String,
    pub icon: String,
}

pub struct Clock {
    pub current_date: String,
    pub current_time: String,
}

impl Clock {
    pub fn new() -> (Self, Command<Message>) {
        let time_loc: DateTime<Local> = Local::now();
        (
            Self {
                current_time: time_loc.format("%I:%M:%S %p").to_string(),
                current_date: time_loc.format("%m-%d-%Y").to_string(),
            },
            Command::none()
        )
    }
}