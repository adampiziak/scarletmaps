use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use serde::de::{self, Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct Config {
    pub route: Vec<ConfigRoute>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigRoute {
    pub tag: String,
    pub title: String,
    #[serde(deserialize_with = "map_or_seq")]
    stop: Vec<ConfigRouteStop>,
    #[serde(deserialize_with = "from_str")]
    lat_max: f64,
    #[serde(deserialize_with = "from_str")]
    lat_min: f64,
    #[serde(deserialize_with = "from_str")]
    lon_max: f64,
    #[serde(deserialize_with = "from_str")]
    lon_min: f64,
}

#[derive(Deserialize)]
pub struct ConfigRouteStop {
    tag: String,
    title: String,
    #[serde(deserialize_with = "from_str")]
    lon: f64,
    #[serde(deserialize_with = "from_str")]
    lat: f64
}


fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

fn map_or_seq<'de, D>(deserializer: D) -> Result<Vec<ConfigRouteStop>, D::Error>
    where D: Deserializer<'de>
{
    struct Stops;

    impl<'de> de::Visitor<'de> for Stops {
        type Value = Vec<ConfigRouteStop>;
        fn visit_map<A>(self, map: A) -> Result<Vec<ConfigRouteStop>, A::Error>
            where A: de::MapAccess<'de>
        {
            Ok(vec![Deserialize::deserialize(de::value::MapAccessDeserializer::new(map)).unwrap()])
        }

        fn visit_seq<A>(self, visitor: A) -> Result<Vec<ConfigRouteStop>, A::Error>
            where A: de::SeqAccess<'de>
        {
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
        }

        // ERROR
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("stop or list of stops")
        }
    }

    deserializer.deserialize_any(Stops)
}
