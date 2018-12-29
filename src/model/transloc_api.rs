use serde::de::{self, Deserialize, Deserializer};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct Routes {
    pub data: Agency
}

#[derive(Deserialize)]
pub struct Agency {
    #[serde(rename="1323")]
    pub routes: Vec<Route>,
}

#[derive(Deserialize)]
pub struct Route {
    #[serde(deserialize_with = "from_str")]
    pub route_id: i32,
    pub long_name: String,
    pub stops: Vec<String>
}

#[derive(Deserialize)]
pub struct Stops {
    pub data: Vec<Stop>
}


#[derive(Deserialize)]
pub struct Stop {
    #[serde(deserialize_with = "from_str")]
    pub stop_id: i32,
    pub name: String,
    pub routes: Vec<String>
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

