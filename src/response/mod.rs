use std::{fmt, usize};

pub use http::*;
use serde::{
    de::{Unexpected, Visitor},
    Deserializer,
};
pub use ws::*;

pub mod http;
pub mod ws;

struct F64Visitor;

impl<'de> Visitor<'de> for F64Visitor {
    type Value = f64;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representation of a f64")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse::<f64>()
            .map_err(|_| E::invalid_value(Unexpected::Str(v), &"a string representation as f64"))
    }
}

fn string_as_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(F64Visitor)
}

struct UsizeVisitor;

impl<'de> Visitor<'de> for UsizeVisitor {
    type Value = usize;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representation of a usize")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse::<usize>()
            .map_err(|_| E::invalid_value(Unexpected::Str(v), &"a string representation as usize"))
    }
}

fn string_as_usize<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(UsizeVisitor)
}
