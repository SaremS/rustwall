use std::fmt;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use currency::Currency;


pub struct CurrencyWrapper {
    currency: Currency
}

impl<'de> Deserialize<'de> for CurrencyWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CurrencyWrapperVisitor;

        impl<'de> Visitor<'de> for CurrencyWrapperVisitor {
            type Value = CurrencyWrapper;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string in a format that crate `currency` can parse")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let currency = Currency::from_str(value);

                match currency {
                    Ok(c) => return Ok(CurrencyWrapper{currency: c}),
                    Err(e) => return Err(de::Error::custom("Cannot parse currency")),
                }
            }
        }

        deserializer.deserialize_string(CurrencyWrapperVisitor)
    }
}
