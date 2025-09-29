use std::{f64, fmt, marker::PhantomData};
use serde::de::{self, Deserializer, Unexpected};
use serde::ser::Serializer;
use serde_with::{DeserializeAs, SerializeAs};

pub struct AsString;

impl<'de> DeserializeAs<'de, String> for AsString {
    fn deserialize_as<D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DeserializeStringWithVisitor)
    }
}

impl SerializeAs<String> for AsString
{

    fn serialize_as<S>(source: &String, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(source)
    }
}

/// De-serialize either a `null`, `str`, `i64`, `f64`, `u64`, or `bool`
/// as a *signed* value.
///
/// # Errors
/// Returns an error if a string is non-empty and not a valid numeric
/// value, or if the unsigned value `u64` *overflows* when converted
/// to `i64`.
///
/// # Returns
/// The signed (`i64`) value of a string or number.
///
pub struct AsI64;

impl<'de> DeserializeAs<'de, i64> for AsI64 {
    fn deserialize_as<D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DeserializeI64WithVisitor)
    }
}

impl SerializeAs<i64> for AsI64
{

    fn serialize_as<S>(source: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(*source)
    }
}

/// De-serialize either a `null`, `str`, `f64`, `u64`, `i64`, or `bool`
/// as a *float* value.
///
/// # Errors
/// Returns an error if a string is non-empty and not a valid numeric value.
///
/// # Returns
/// The floating point (`f64`) value of a string or number.
///
pub struct AsF64;

impl<'de> DeserializeAs<'de, f64> for AsF64 {
    fn deserialize_as<D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DeserializeF64WithVisitor)
    }
}

impl SerializeAs<f64> for AsF64
{

    fn serialize_as<S>(source: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64(*source)
    }
}

struct DeserializeI64WithVisitor;

impl de::Visitor<'_> for DeserializeI64WithVisitor {
    type Value = i64;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a signed integer or a string")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v {
            Ok(1)
        } else {
            Ok(0)
        }
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match i64::try_from(v) {
            Ok(v) => Ok(v),
            Err(_) => Err(E::custom(format!(
                "overflow: Unable to convert unsigned value `{v:?}` to i64"
            ))),
        }
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.round() as i64)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(n) = v.parse::<i64>() {
            Ok(n)
        } else if v.is_empty() {
            Ok(0)
        } else if let Ok(f) = v.parse::<f64>() {
            Ok(f.round() as i64)
        } else {
            Err(E::invalid_value(Unexpected::Str(v), &self))
        }
    }

    /// We encounter a `null` value; this default implementation returns a
    /// "zero" value.
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(0)
    }
}

struct DeserializeF64WithVisitor;

impl de::Visitor<'_> for DeserializeF64WithVisitor {
    type Value = f64;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a float or a string")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v {
            Ok(1.0)
        } else {
            Ok(0.0)
        }
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v as f64)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v as f64)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(f) = v.parse::<f64>() {
            Ok(f)
        } else if v.is_empty() {
            Ok(0.0)
        } else {
            Err(E::invalid_value(Unexpected::Str(v), &self))
        }
    }

    /// We encounter a `null` value; this default implementation returns a
    /// "zero" value.
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(0.0)
    }
}

struct DeserializeStringWithVisitor;

impl de::Visitor<'_> for DeserializeStringWithVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string, bool, or a number")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_owned())
    }

    /// We encounter a `null` value; this default implementation returns an
    /// "empty" string.
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(String::new())
    }
}

pub struct AsEnum<T>(PhantomData<T>);

impl<'de, T> DeserializeAs<'de, T> for AsEnum<T>
where
    T: TryFrom<i64>,
    <T as TryFrom<i64>>::Error: fmt::Display,
{

    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        // First, deserialize to an i64 using the flexible visitor.
        let value = deserializer.deserialize_any(DeserializeI64WithVisitor)?;
        // Then, try to convert the i64 into the enum.
        T::try_from(value).map_err(|err| de::Error::custom(err.to_string()))
    }
}

impl<T> SerializeAs<T> for AsEnum<T>
where
    T: Into<i64> + Copy,
{

    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let int_value: i64 = (*source).into();
        serializer.serialize_i64(int_value)
    }
}
