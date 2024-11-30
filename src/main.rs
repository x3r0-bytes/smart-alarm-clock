use iced;
use reqwest::{self, Client, Error};
use chrono::{self, Local, DateTime};
use tokio;

// "41.11207176071251", "-111.91263498602"
use std::fmt;

#[tokio::main]
async fn main() {
    let lat = "41.11207176071251";
    let long = "-111.91263498602";
    time().await; 
    match weather_api(lat, long).await {
        Ok(response) => println!("Response: {}", response),
        Err(err) => eprintln!("Error fetching weather: {}", err),
    }
}

// Define a custom error type for better error handling
#[derive(Debug)]
enum WeatherError {
    HttpError(reqwest::StatusCode),
    RequestError(reqwest::Error),
}

impl fmt::Display for WeatherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WeatherError::HttpError(status) => write!(f, "HTTP error: {}", status),
            WeatherError::RequestError(err) => write!(f, "Request error: {}", err),
        }
    }
}

impl From<reqwest::Error> for WeatherError {
    fn from(err: reqwest::Error) -> Self {
        WeatherError::RequestError(err)
    }
}

async fn weather_api(lat: &str, long: &str) -> Result<String, WeatherError> {
    let client = reqwest::Client::new();
    let url_points = format!("https://api.weather.gov/points/{},{}", lat, long);
    let response = client
        .get(&url_points)
        .header("User-Agent", "Weather-clock/1.0 (email@email.com)")
        .send()
        .await?;

    if response.status().is_success() {
        let url_points_response = response.text().await?;
        Ok(url_points_response)
    } else {
        Err(WeatherError::HttpError(response.status()))
    }
}

async fn time() {
    let time_loc: DateTime<Local> = Local::now();
    print!("The Current time is: {}", time_loc); 
}
