use currency::Currency;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

/// Wraps [currency::Currency](https://docs.rs/currency/latest/currency/struct.Currency.html) to enable direct serde_yml deserialization
/// # Examples
/// ```
/// use currency::Currency;
/// use serde::Deserialize;
/// let config_yml = r#"
///     $1.00
///     "#;
///
/// let currency_wrapper_target: CurrencyWrapper = serde_yml::from_str(config_yml).unwrap();
/// let currency = Currency::from_str("$1.00").unwrap();
/// let currency_wrapper_expected = CurrencyWrapper { currency: currency };
/// assert_eq!(currency_wrapper_target, currency_wrapper_expected);
/// ```
#[derive(Debug, Clone)]
pub struct CurrencyWrapper {
    pub currency: Currency,
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
                    Ok(c) => return Ok(CurrencyWrapper { currency: c }),
                    Err(e) => return Err(de::Error::custom("Cannot parse currency")),
                }
            }
        }

        deserializer.deserialize_string(CurrencyWrapperVisitor)
    }
}

impl PartialEq for CurrencyWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.currency == other.currency
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_eq() {
        let currency = Currency::from_str("$1.00").unwrap();
        let currency_wrapper1 = CurrencyWrapper {
            currency: currency.clone(),
        };
        let currency_wrapper2 = CurrencyWrapper { currency: currency };

        assert_eq!(currency_wrapper1, currency_wrapper2);
    }

    #[test]
    fn test_deserialize_succes() {
        let config_yml = r#"
        $1.00
        "#;

        let currency_wrapper_target: CurrencyWrapper = serde_yml::from_str(config_yml).unwrap();
        let currency = Currency::from_str("$1.00").unwrap();
        let currency_wrapper_expected = CurrencyWrapper { currency: currency };

        assert_eq!(currency_wrapper_target, currency_wrapper_expected);
    }
}
