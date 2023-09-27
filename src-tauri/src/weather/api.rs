use itertools::izip;

use crate::weather::types::WeatherHour;

use super::types::{Root, Weather};

const API_URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=43.5858&longitude=7.1083&hourly=temperature_2m,rain,cloudcover&daily=sunrise,sunset,uv_index_max&timezone=Europe%2FBerlin&forecast_days=1";

async fn weather_api() -> Result<Root, Box<dyn std::error::Error>> {
    let resp = reqwest::get(API_URL).await?.json::<Root>().await?;
    println!("Debug: {:?}", resp);
    Ok(resp)
}

pub async fn get_weather_backend() -> Result<Weather, String> {
    let weather_api = weather_api()
        .await
        .map_err(|e| format!("Error getting data: {}", e).to_string())?;

    println!("Debug: {:?}", weather_api);

    let weather_hourly = izip!(
        &weather_api.hourly.time,
        &weather_api.hourly.temperature_2m,
        &weather_api.hourly.rain,
        &weather_api.hourly.cloudcover
    )
    .map(|(time, temp, rain, cloud)| WeatherHour::new(time.to_owned(), *temp, *rain, *cloud))
    .collect::<Vec<WeatherHour>>();

    Ok(Weather::new(
        weather_hourly,
        weather_api.daily.uv_index_max[0],
    ))
}
