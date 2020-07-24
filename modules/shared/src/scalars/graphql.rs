use chrono::prelude::*;
use juniper::{serde::de, GraphQLScalarValue, ScalarValue};
use juniper::{ParseScalarResult, ParseScalarValue, Value};

#[derive(Debug, PartialEq, Clone, GraphQLScalarValue)]
pub enum ServerScalarValue {
    Int(i32),
    Long(i64),
    UnsignedInt(u32),
    UnsignedLong(u64),
    Float(f32),
    Double(f64),
    String(String),
    Boolean(bool),
    NaiveDateTime(NaiveDateTime),
}

impl ScalarValue for ServerScalarValue {
    type Visitor = ServerScalarValueVisitor;

    fn as_int(&self) -> Option<i32> {
        match *self {
            ServerScalarValue::Int(ref i) => Some(*i),
            _ => None,
        }
    }

    fn as_string(&self) -> Option<String> {
        match *self {
            ServerScalarValue::String(ref s) => Some(s.clone()),
            _ => None,
        }
    }

    fn as_str(&self) -> Option<&str> {
        match *self {
            ServerScalarValue::String(ref s) => Some(s),
            _ => None,
        }
    }

    fn as_float(&self) -> Option<f64> {
        match *self {
            ServerScalarValue::Int(ref i) => Some(*i as f64),
            ServerScalarValue::Float(ref f) => Some(*f as f64),
            ServerScalarValue::Double(ref f) => Some(*f),
            _ => None,
        }
    }

    fn as_boolean(&self) -> Option<bool> {
        match *self {
            ServerScalarValue::Boolean(ref b) => Some(*b),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct ServerScalarValueVisitor;

impl<'de> de::Visitor<'de> for ServerScalarValueVisitor {
    type Value = ServerScalarValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid input value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<ServerScalarValue, E> {
        Ok(ServerScalarValue::Boolean(value))
    }

    fn visit_i32<E>(self, value: i32) -> Result<ServerScalarValue, E>
    where
        E: de::Error,
    {
        Ok(ServerScalarValue::Int(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<ServerScalarValue, E>
    where
        E: de::Error,
    {
        if value <= i32::max_value() as i64 {
            self.visit_i32(value as i32)
        } else {
            Ok(ServerScalarValue::Long(value))
        }
    }

    fn visit_u32<E>(self, value: u32) -> Result<ServerScalarValue, E>
    where
        E: de::Error,
    {
        if value <= i32::max_value() as u32 {
            self.visit_i32(value as i32)
        } else {
            Ok(ServerScalarValue::UnsignedInt(value))
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<ServerScalarValue, E>
    where
        E: de::Error,
    {
        if value <= u32::max_value() as u64 {
            self.visit_u32(value as u32)
        } else {
            Ok(ServerScalarValue::UnsignedLong(value))
        }
    }

    fn visit_f32<E>(self, value: f32) -> Result<ServerScalarValue, E> {
        Ok(ServerScalarValue::Float(value))
    }

    fn visit_f64<E>(self, value: f64) -> Result<ServerScalarValue, E> {
        Ok(ServerScalarValue::Double(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<ServerScalarValue, E>
    where
        E: de::Error,
    {
        self.visit_string(value.into())
    }

    fn visit_string<E>(self, value: String) -> Result<ServerScalarValue, E> {
        Ok(ServerScalarValue::String(value))
    }
}

#[juniper::graphql_scalar(name = "Long")]
impl GraphQLScalar for i64 {
    // Define how to convert your custom scalar into a primitive type.
    fn resolve(&self) -> Value<ServerScalarValue> {
        Value::scalar(*self)
    }

    // Define how to parse a primitive type into your custom scalar.
    fn from_input_value(v: &InputValue<ServerScalarValue>) -> Option<i64> {
        v.as_scalar_value()
            .and_then(|v: &ServerScalarValue| v.as_str())
            .and_then(|s| s.parse().ok())
    }

    // Define how to parse a string value.
    fn from_str<'a>(value: ScalarToken<'a, ServerScalarValue>) -> ParseScalarResult<'a, ServerScalarValue> {
        <String as ParseScalarValue<ServerScalarValue>>::from_str(value)
    }
}

#[juniper::graphql_scalar(name = "UnsignedInt")]
impl GraphQLScalar for u32 {
    // Define how to convert your custom scalar into a primitive type.
    fn resolve(&self) -> Value<ServerScalarValue> {
        Value::scalar(*self)
    }

    // Define how to parse a primitive type into your custom scalar.
    fn from_input_value(v: &InputValue<ServerScalarValue>) -> Option<u32> {
        v.as_scalar_value()
            .and_then(|v: &ServerScalarValue| v.as_str())
            .and_then(|s| s.parse().ok())
    }

    // Define how to parse a string value.
    fn from_str<'a>(value: ScalarToken<'a, ServerScalarValue>) -> ParseScalarResult<'a, ServerScalarValue> {
        <String as ParseScalarValue<ServerScalarValue>>::from_str(value)
    }
}

#[juniper::graphql_scalar(name = "UnsignedLong")]
impl GraphQLScalar for u64 {
    // Define how to convert your custom scalar into a primitive type.
    fn resolve(&self) -> Value<ServerScalarValue> {
        Value::scalar(*self)
    }

    // Define how to parse a primitive type into your custom scalar.
    fn from_input_value(v: &InputValue<ServerScalarValue>) -> Option<u64> {
        v.as_scalar_value()
            .and_then(|v: &ServerScalarValue| v.as_str())
            .and_then(|s| s.parse().ok())
    }

    // Define how to parse a string value.
    fn from_str<'a>(value: ScalarToken<'a, ServerScalarValue>) -> ParseScalarResult<'a, ServerScalarValue> {
        <String as ParseScalarValue<ServerScalarValue>>::from_str(value)
    }
}

#[juniper::graphql_scalar(name = "Float")]
impl GraphQLScalar for f32 {
    // Define how to convert your custom scalar into a primitive type.
    fn resolve(&self) -> Value<ServerScalarValue> {
        Value::scalar(*self)
    }

    // Define how to parse a primitive type into your custom scalar.
    fn from_input_value(v: &InputValue<ServerScalarValue>) -> Option<f32> {
        v.as_scalar_value()
            .and_then(|v: &ServerScalarValue| v.as_str())
            .and_then(|s| s.parse().ok())
    }

    // Define how to parse a string value.
    fn from_str<'a>(value: ScalarToken<'a, ServerScalarValue>) -> ParseScalarResult<'a, ServerScalarValue> {
        <String as ParseScalarValue<ServerScalarValue>>::from_str(value)
    }
}

#[juniper::graphql_scalar]
impl GraphQLScalar for NaiveDateTime {
    // Define how to convert your custom scalar into a primitive type.
    fn resolve(&self) -> Value<ServerScalarValue> {
        Value::scalar(self.timestamp_millis().to_string())
    }

    // Define how to parse a primitive type into your custom scalar.
    fn from_input_value(v: &InputValue<ServerScalarValue>) -> Option<NaiveDateTime> {
        v.as_scalar_value()
            .and_then(|v: &ServerScalarValue| v.as_str())
            .and_then(|v| v.parse::<i64>().ok())
            .and_then(|v| Some(Utc.timestamp_millis(v).naive_utc()))
    }

    // Define how to parse a string value.
    fn from_str<'a>(value: ScalarToken<'a, ServerScalarValue>) -> ParseScalarResult<'a, ServerScalarValue> {
        <String as ParseScalarValue<ServerScalarValue>>::from_str(value)
    }
}
