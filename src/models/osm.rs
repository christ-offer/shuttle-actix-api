use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Direction {
    pub lat: String,
    pub lon: String,
}
