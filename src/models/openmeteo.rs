use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailyUnits {
    pub time: String,
    pub weathercode: String,
    pub temperature_2m_max: String,
    pub temperature_2m_min: String,
    pub temperature_2m_mean: String,
    pub apparent_temperature_max: String,
    pub apparent_temperature_min: String,
    pub apparent_temperature_mean: String,
    pub sunrise: String,
    pub sunset: String,
    pub shortwave_radiation_sum: String,
    pub precipitation_sum: String,
    pub precipitation_hours: String,
    pub windspeed_10m_max: String,
    pub windgusts_10m_max: String,
    snowfall_sum: String,
    pub winddirection_10m_dominant: String,
    pub et0_fao_evapotranspiration: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Daily {
    pub time: Vec<Option<String>>,
    pub weathercode: Vec<Option<f64>>,
    pub temperature_2m_max: Vec<Option<f64>>,
    pub temperature_2m_min: Vec<Option<f64>>,
    pub temperature_2m_mean: Vec<Option<f64>>,
    pub apparent_temperature_max: Vec<Option<f64>>,
    pub apparent_temperature_min: Vec<Option<f64>>,
    pub apparent_temperature_mean: Vec<Option<f64>>,
    pub sunrise: Vec<Option<String>>,
    pub sunset: Vec<Option<String>>,
    pub shortwave_radiation_sum: Vec<Option<f64>>,
    pub precipitation_sum: Vec<Option<f64>>,
    pub precipitation_hours: Vec<Option<f64>>,
    pub windspeed_10m_max: Vec<Option<f64>>,
    pub windgusts_10m_max: Vec<Option<f64>>,
    pub snowfall_sum: Vec<Option<f64>>,
    pub winddirection_10m_dominant: Vec<Option<f64>>,
    pub et0_fao_evapotranspiration: Vec<Option<f64>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenMeteo {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: f64,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f64,
    pub daily_units: DailyUnits,
    pub daily: Daily,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatMap {
    pub min: MinMaxMean,
    pub max: MinMaxMean,
    pub mean: MinMaxMean,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MinMaxMean {
    pub time: String,
    pub value: f64,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stats {
    pub temperature_2m_max: StatMap,
    pub temperature_2m_min: StatMap,
    pub temperature_2m_mean: StatMap,
    pub apparent_temperature_max: StatMap,
    pub apparent_temperature_min: StatMap,
    pub apparent_temperature_mean: StatMap,
    pub shortwave_radiation_sum: StatMap,
    pub precipitation_sum: StatMap,
    pub precipitation_hours: StatMap,
    pub windspeed_10m_max: StatMap,
    pub windgusts_10m_max: StatMap,
    pub et0_fao_evapotranspiration: StatMap,
    pub snowfall_sum: StatMap,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReturnOpenMeteo {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: f64,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f64,
    pub daily: Daily,
    pub daily_units: DailyUnits,
    //pub daily_sorted: Daily,
    pub stats: Stats,
}

#[derive(Deserialize)]
pub struct OpenMeteoRequest {
    pub city: String,
    pub start_date: String,
    pub end_date: String,
}
