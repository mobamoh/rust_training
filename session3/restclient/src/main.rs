use anyhow::Ok;
use serde::Deserialize;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Current Weather in Midelt!");
    let url = "https://api.open-meteo.com/v1/forecast?latitude=32.6852&longitude=-4.7451&current_weather=true";

    let response = reqwest::get(url).await?;
    let weather: Weather = response.json().await?;
    println!("{weather:#?}");
    Ok(())
}

#[derive(Deserialize, Debug)]
struct Weather {
    latitude: f64,
    longitude: f64,
    current_weather: CurrentWeather,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
}
