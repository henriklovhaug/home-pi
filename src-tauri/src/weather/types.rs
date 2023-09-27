use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub hourly: Hourly,
    pub daily: Daily,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hourly {
    pub time: Vec<String>,
    #[serde(rename = "temperature_2m")]
    pub temperature_2m: Vec<f64>,
    pub rain: Vec<f64>,
    pub cloudcover: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daily {
    pub time: Vec<String>,
    pub sunrise: Vec<String>,
    pub sunset: Vec<String>,
    #[serde(rename = "uv_index_max")]
    pub uv_index_max: Vec<f64>,
}

#[derive(Debug, Serialize)]
pub struct WeatherHour {
    pub hour: DateTime<Utc>,
    pub temperature: f64,
    pub rain: f64,
    pub cloudcover: i64,
}

impl WeatherHour {
    pub fn new(hour: String, temperature: f64, rain: f64, cloudcover: i64) -> Self {
        // Parse "2021-06-06T12:00:00"
        let hour = DateTime::parse_from_str(&hour, "%Y-%m-%dT%H:%M")
            .map_err(|e| format!("Error parsing date: {}", e)).unwrap()
            .with_timezone(&Utc);
        Self {
            hour,
            temperature,
            rain,
            cloudcover,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Weather {
    pub hourly: Vec<WeatherHour>,
    pub uv: f64,
}

impl Weather {
    pub fn new(hourly: Vec<WeatherHour>, uv: f64) -> Self {
        Self { hourly, uv }
    }
}
