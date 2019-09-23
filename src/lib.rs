#![allow(unused)]
//! DynamoDB Data Utilities
//! 
//! API Example:
//! ```
//! use dynamodb_data::*;
//! use std::collections::HashMap;
//! 
//! let payload: HashMap<String, rusoto_dynamodb::AttributeValue> = fields!{
//!     id => ::uuid::Uuid::new_v4(),
//!     name => "user name",
//!     counter => 0
//! };
//! let get_item_query = rusoto_dynamodb::GetItemInput {
//!     key: fields!{
//!         id => ::uuid::Uuid::new_v4()
//!     },
//!     ..Default::default()
//! };
//! ```
use std::collections::HashMap;
use serde;
use serde::{Serialize, Deserialize};
use serde_json::{self, Value};
use rusoto_core;
use rusoto_core::Region;
use rusoto_dynamodb;
use rusoto_dynamodb::{
    DynamoDb,
    DynamoDbClient,
    QueryInput,
    AttributeValue,
};


///////////////////////////////////////////////////////////////////////////////
// EXTERNAL API - FUNCTIONS
///////////////////////////////////////////////////////////////////////////////

/// Converts any serializable value to a `rusoto_dynamodb::AttributeValue`.
/// ```
/// let msg: rusoto_dynamodb::AttributeValue = dynamodb_data::to_attribute_value("Hello World")
///     .expect("serde issue");
/// ```
pub fn to_attribute_value<A: Serialize>(value: A) -> Result<AttributeValue, serde_json::Error> {
    match serde_json::to_value(value) {
        Ok(x) => Ok(json_to_attribute_value(x)),
        Err(e) => Err(e),
    }
}

/// Converts any serializable value from a `rusoto_dynamodb::AttributeValue`.
/// ```
/// let msg: rusoto_dynamodb::AttributeValue = dynamodb_data::to_attribute_value("Hello World").expect("serde issue");
/// let msg: String = dynamodb_data::from_attribute_value(msg).expect("serde issue");
/// ```
pub fn from_attribute_value<A: serde::de::DeserializeOwned>(value: AttributeValue) -> Result<A, serde_json::Error> {
    let value: Value = attribute_value_to_json(value);
    serde_json::from_value(value)
}

/// Must be something that serializes to a JSON Object.
/// ```
/// use std::collections::HashMap;
/// use std::iter::FromIterator;
/// let mut object: HashMap<String, u32> = HashMap::from_iter(vec![
///     (String::from("red"), 1),
///     (String::from("green"), 2),
///     (String::from("blue"), 3)
/// ]);
/// let msg: HashMap<String, rusoto_dynamodb::AttributeValue> = dynamodb_data::to_fields(object).expect("to_fields issue");
/// ```
pub fn to_fields<A: Serialize>(value: A) -> Result<HashMap<String, AttributeValue>, serde_json::Error> {
    match serde_json::to_value(value) {
        Ok(x) => Ok(json_to_attribute_value_hashmap(x)),
        Err(e) => Err(e),
    }
}

/// Must be something that serializes from a JSON Object.
/// ```
/// use std::collections::HashMap;
/// use std::iter::FromIterator;
/// let mut object: HashMap<String, u32> = HashMap::from_iter(vec![
///     (String::from("red"), 1),
///     (String::from("green"), 2),
///     (String::from("blue"), 3)
/// ]);
/// let msg: HashMap<String, rusoto_dynamodb::AttributeValue> = dynamodb_data::to_fields(object).expect("to_fields issue");
/// let msg: HashMap<String, u32> = dynamodb_data::from_fields(msg).expect("from_fields failed");
/// ```
pub fn from_fields<A: serde::de::DeserializeOwned>(value: HashMap<String, AttributeValue>) -> Result<A, serde_json::Error> {
    let value: serde_json::Map<String, Value> = attribute_value_hashmap_to_json_map(value);
    serde_json::from_value(Value::Object(value))
}


///////////////////////////////////////////////////////////////////////////////
// EXTERNAL API - MACROS
///////////////////////////////////////////////////////////////////////////////

/// Internal
#[doc(hidden)]
#[macro_export]
macro_rules! i_field_key {
    ($k:ident) => {
        stringify!($k).to_owned()
    };
    ($k:expr) => {{
        let value: String = $k.to_owned();
        value
    }};
}

/// Converts the given fields to `HashMap<String, AttributeValue>`, automatically
/// serializing the keys to `AttributeValue` VIA `serde_json`.
/// 
/// Example 1:
/// ```
/// use dynamodb_data::*;
/// use std::collections::HashMap;
/// 
/// let payload: HashMap<String, rusoto_dynamodb::AttributeValue> = fields!{
///     id => ::uuid::Uuid::new_v4(),
///     name => "user name",
///     counter => 0
/// };
/// 
/// // Str keys are also supported
/// let payload: HashMap<String, rusoto_dynamodb::AttributeValue> = fields!{
///     "id" => ::uuid::Uuid::new_v4(),
///     // `name` is a DynamoDB reserved word.
///     ":name" => "user name",
///     "counter" => 0,
///     // Others can still be identifiers though
///     some_other_field => 0
/// };
/// ```
/// 
/// Example 2:
/// ```
/// use dynamodb_data::*;
/// let get_item_query = rusoto_dynamodb::GetItemInput {
///     key: fields!{
///         id => ::uuid::Uuid::new_v4()
///     },
///     ..Default::default()
/// };
/// ```
#[macro_export]
macro_rules! fields {
    ($($k:tt => $v:expr),* $(,)?) => {{
        use std::collections::hash_map::HashMap;
        use rusoto_dynamodb::AttributeValue;
        use $crate::*;

        let results: HashMap<String, AttributeValue> = {
            let mut m = HashMap::new();
            $(
                m.insert(
                    i_field_key!($k),
                    to_attribute_value($v).expect("object! serialization failure")
                );
            )*
            m
        };
        results
    }};
}



/// ```
/// use dynamodb_data::*;
/// rusoto_dynamodb::PutItemInput {
///     expression_attribute_names: Some(names!{
///         ":id" => "id"
///     }),
///     ..Default::default()
/// };
/// ```
#[macro_export]
macro_rules! names {
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::collections::hash_map::HashMap;

        let results: HashMap<String, String> = {
            let mut m = HashMap::new();
            $(
                m.insert($k.to_owned(), $v.to_owned());
            )*
            m
        };
        results
    }};
}



///////////////////////////////////////////////////////////////////////////////
// INTERNAL - ATTRIBUTE-VALUE
///////////////////////////////////////////////////////////////////////////////

fn json_to_attribute_value(value: Value) -> AttributeValue {
    use std::iter::FromIterator;

    match value {
        Value::Array(xs) => {
            let xs: Vec<AttributeValue> = xs
                .into_iter()
                .map(|x| json_to_attribute_value(x))
                .collect();
            AttributeValue {
                l: Some(xs),
                ..Default::default()
            }
        },
        Value::Object(xs) => {
            let xs: Vec<(String, AttributeValue)> = xs
                .into_iter()
                .map(|(k, v)| (k, json_to_attribute_value(v)))
                .collect();
            let xs: HashMap<String, AttributeValue> = HashMap::from_iter(xs);
            AttributeValue {
                m: Some(xs),
                ..Default::default()
            }
        },
        Value::String(x) => {
            if x.is_empty() {
                AttributeValue {
                    s: Some(String::from("\0")),
                    ..Default::default()
                }
            } else {
                AttributeValue {
                    s: Some(x),
                    ..Default::default()
                }
            }
        },
        Value::Number(x) => AttributeValue {
            n: Some(format!("{}", x)),
            ..Default::default()
        },
        Value::Bool(x) => AttributeValue {
            bool: Some(x),
            ..Default::default()
        },
        Value::Null => AttributeValue {
            null: Some(true),
            ..Default::default()
        },
    }
}

fn attribute_value_to_json(value: AttributeValue) -> Value {
    use std::iter::FromIterator;

    if value.b.is_some() {
        match String::from_utf8(value.b.expect("dynamodb sdk error").to_vec()) {
            Ok(x) => Value::String(x),
            _ => panic!()
        }
    }
    else if value.bool.is_some() {
        Value::Bool(value.bool.expect("dynamodb sdk error"))
    }
    else if value.bs.is_some() {
        let xs = value.bs
            .expect("dynamodb sdk error")
            .into_iter()
            .map(|x| {
                match String::from_utf8(x.to_vec()) {
                    Ok(x) => Value::String(x),
                    _ => panic!()
                }
            })
            .collect();
        Value::Array(xs)
    }
    else if value.l.is_some() {
        let xs: Vec<Value> = value.l
            .expect("dynamodb sdk error")
            .into_iter()
            .map(|x| attribute_value_to_json(x))
            .collect();
        Value::Array(xs)
    }
    else if value.m.is_some() {
        let xs: Vec<(String, Value)> = value.m
            .expect("dynamodb sdk error")
            .into_iter()
            .map(|(k, v)| (k, attribute_value_to_json(v)))
            .collect();
        let xs: serde_json::Map<String, Value> = serde_json::Map::from_iter(xs);
        Value::Object(xs)
    }
    else if value.n.is_some() {
        serde_json::from_str(value.n.expect("dynamodb sdk error").as_str()).expect("dynamodb sdk error")
    }
    else if value.ns.is_some() {
        let xs: Vec<Value> = value.ns
            .expect("dynamodb sdk error")
            .into_iter()
            .map(|x| serde_json::from_str(x.as_str()).expect("dynamodb sdk error"))
            .collect();
        Value::Array(xs)
    }
    else if value.s.is_some() {
        let text = value.s.expect("dynamodb sdk error");
        if text == String::from("\0") {
            Value::String(String::new())
        } else {
            Value::String(text)
        }
    }
    else if value.null.is_some() {
        Value::Null
    }
    else if value.ss.is_some() {
        let xs: Vec<Value> = value.ns
            .expect("dynamodb sdk error")
            .into_iter()
            .map(|x| Value::String(x))
            .collect();
        Value::Array(xs)
    } else {
        panic!()
    }
}


///////////////////////////////////////////////////////////////////////////////
// INTERNAL - ATTRIBUTE-VALUE HASH-MAP
///////////////////////////////////////////////////////////////////////////////

fn json_to_attribute_value_hashmap(value: Value) -> HashMap<String, AttributeValue> {
    json_to_attribute_value(value).m.expect("dynamodb sdk error")
}

fn attribute_value_hashmap_to_json_map(value: HashMap<String, AttributeValue>) -> serde_json::Map<String, Value> {
    use std::iter::FromIterator;
    let value: AttributeValue = AttributeValue {
        m: Some(value),
        ..Default::default()
    };
    match attribute_value_to_json(value) {
        Value::Object(xs) => {
            xs
        },
        _ => panic!()
    }
}



