use chrono::prelude::*;
use derive_more::From;
use juniper::{ParseScalarResult, ParseScalarValue, Value};

#[derive(Clone, Debug, From)]
pub struct UnixTime(pub NaiveDateTime);

#[juniper::graphql_scalar(name = "NaiveDateTime")]
impl<S> GraphQLScalar for UnixTime
where
    S: ScalarValue,
{
    // Define how to convert your custom scalar into a primitive type.
    fn resolve(&self) -> Value {
        Value::scalar(self.0.timestamp_millis().to_string())
    }

    // Define how to parse a primitive type into your custom scalar.
    fn from_input_value(v: &InputValue) -> Option<UnixTime> {
        v.as_scalar_value()
            .and_then(|v| v.as_str())
            .and_then(|v| v.parse::<i64>().ok())
            .and_then(|v| Some(UnixTime(Utc.timestamp_millis(v).naive_utc())))
    }

    // Define how to parse a string value.
    fn from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        <String as ParseScalarValue<S>>::from_str(value)
    }
}
