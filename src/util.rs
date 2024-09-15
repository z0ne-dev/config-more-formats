use config::{Map, Value, ValueKind};
use std::error::Error;
use std::fmt;

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum Val {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Self>),
    Object(Map<String, Self>),
}

// Copied over and adjusted from config crate
#[allow(dead_code)]
pub fn extract_root_table(
    value: Value,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    match value.kind {
        ValueKind::Table(map) => Ok(map),
        ValueKind::Nil => Err(Unexpected::Unit),
        ValueKind::Array(_value) => Err(Unexpected::Seq),
        ValueKind::Boolean(value) => Err(Unexpected::Bool(value)),
        ValueKind::I64(value) => Err(Unexpected::I64(value)),
        ValueKind::I128(value) => Err(Unexpected::I128(value)),
        ValueKind::U64(value) => Err(Unexpected::U64(value)),
        ValueKind::U128(value) => Err(Unexpected::U128(value)),
        ValueKind::Float(value) => Err(Unexpected::Float(value)),
        ValueKind::String(value) => Err(Unexpected::Str(value)),
    }
        .map_err(|err| Box::new(err) as Box<dyn Error + Send + Sync>)
}

#[derive(Debug)]
pub enum Unexpected {
    Bool(bool),
    I64(i64),
    I128(i128),
    U64(u64),
    U128(u128),
    Float(f64),
    Str(String),
    Unit,
    Seq,
}

impl fmt::Display for Unexpected {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Unexpected::Bool(b) => write!(f, "unexpected boolean `{}`", b),
            Unexpected::I64(i) => write!(f, "unexpected 64-bit integer `{}`", i),
            Unexpected::I128(i) => write!(f, "unexpected 128-bit integer `{}`", i),
            Unexpected::U64(i) => write!(f, "unexpected 64-bit unsigned integer `{}`", i),
            Unexpected::U128(i) => write!(f, "unexpected 128-bit unsigned integer `{}`", i),
            Unexpected::Float(v) => write!(f, "unexpected floating point `{}`", v),
            Unexpected::Str(ref s) => write!(f, "unexpected string {:?}", s),
            Unexpected::Unit => write!(f, "unexpected unit value"),
            Unexpected::Seq => write!(f, "unexpected sequence"),
        }
    }
}

impl Error for Unexpected {}

// Copied over and adjusted from config crate
#[allow(dead_code)]
pub fn from_value(uri: Option<&String>, value: Val) -> Value {
    let vk = match value {
        Val::Null => ValueKind::Nil,
        Val::String(v) => ValueKind::String(v),
        Val::Integer(v) => ValueKind::I64(v),
        Val::Float(v) => ValueKind::Float(v),
        Val::Boolean(v) => ValueKind::Boolean(v),
        Val::Object(table) => {
            let m = table
                .into_iter()
                .map(|(k, v)| (k, from_value(uri, v)))
                .collect();

            ValueKind::Table(m)
        }

        Val::Array(array) => {
            let l = array
                .into_iter()
                .map(|v| from_value(uri, v))
                .collect();

            ValueKind::Array(l)
        }
    };

    Value::new(uri, vk)
}