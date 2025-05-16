pub mod currency_wrapper;
pub mod paywall_condition;
pub mod requestable_doc;
pub mod url_path;

pub use currency_wrapper::CurrencyWrapper;
pub use paywall_condition::PaywallCondition;
pub use requestable_doc::{DocumentAndPath, RequestableDoc};
pub use url_path::{UrlPath, UrlPathError};

use crate::utils::{HtmlAttributeSelector, HtmlAttributeSelectorError};

use std::fmt;
use currency::Currency;
use html_editor::operation::Selector;
use serde::Deserialize;

pub struct PaywallConfigV1 {
    paths: Vec<PaywallElement>,
}

#[derive(Deserialize)]
pub enum PriceSource {
    Hard(CurrencyWrapper),
    FromHtmlAttribute(HtmlAttributeSelector),
}

#[derive(Debug)]
pub enum PriceSourceExtractError {
    HtmlAttributeSelectorError(HtmlAttributeSelectorError),
}

impl fmt::Display for PriceSourceExtractError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PriceSourceExtractError::HtmlAttributeSelectorError(err) => {
                write!(f, "HtmlAttributeSelectorError: {}", err)
            }
        }
    }
}

impl PriceSource {
    pub fn get_price(
        &self,
        doc_and_path: &DocumentAndPath,
    ) -> Result<Currency, PriceSourceExtractError> {
        let doc = doc_and_path.get_document();

        match (self, doc) {
            (PriceSource::Hard(CurrencyWrapper { currency }), _) => Ok(currency.clone()),
            (PriceSource::FromHtmlAttribute(selector), RequestableDoc::HtmlNode(node)) => {
                let extracted = selector.get_attribute::<Currency>(&node);
                match extracted {
                    Ok(currency) => Ok(currency),
                    Err(error) => Err(PriceSourceExtractError::HtmlAttributeSelectorError(error)),
                }
            }
        }
    }
}

#[derive(Deserialize)]
pub struct PaywallElement {
    paywall_conditions: Vec<PaywallCondition>,
    price_source: PriceSource,
}

pub enum PaywallPriceOption {
    Price(Currency),
    ConditionsNotMet,
    PriceParsingError(String)
}

impl PaywallPriceOption {
    pub fn unwrap(&self) -> Currency {
        match self {
            PaywallPriceOption::Price(p) => p.clone(),
            PaywallPriceOption::ConditionsNotMet => panic!("PaywallPriceOption .unwrap() to Currency but variant is ConditionsNotMet"),
            PaywallPriceOption::PriceParsingError(_) => panic!("PaywallPriceOption .unwrap() to Currency but variant is PriceParsingError"),
        } 
    }
}

impl PaywallElement {
    pub fn get_price(&self, doc_and_path: &DocumentAndPath) -> PaywallPriceOption {
        let is_paywalled = self
            .paywall_conditions
            .iter()
            .map(|x| x.is_paywalled(doc_and_path))
            .all(|r| r);

        if !is_paywalled {
            return PaywallPriceOption::ConditionsNotMet;
        }

        let price = self.price_source.get_price(doc_and_path);

        match price {
            Ok(p) => PaywallPriceOption::Price(p),
            Err(e) => PaywallPriceOption::PriceParsingError(e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use html_editor::parse;

    #[test]
    fn test_paywall_element_simple() {
        let config_yml = r#"
        paywall_conditions:
          - !HasRegexPath "^/premium/.*$"
        price_source: !Hard $1.25
        "#;

        let element: PaywallElement = serde_yml::from_str(config_yml).unwrap();

        let doc_and_path = DocumentAndPath::new_from_html_and_path_str(
            "<html><head></head><body><div id=test data-price=\"$1.25\"/></body></html>",
            "/premium/test",
        )
        .unwrap();

        let currency_target = element.get_price(&doc_and_path).unwrap();
        let currency_expected = Currency::from_str("$1.25").unwrap();

        assert_eq!(currency_target, currency_expected);
    }

    #[test]
    fn test_price_source_hard() {
        let config_yml = r#"
        !Hard $1.25
        "#;

        let price_source: PriceSource = serde_yml::from_str(config_yml).unwrap();

        let doc_and_path = DocumentAndPath::new_from_html_and_path_str(
            "<html><head></head><body><div id=test data-price=\"$1.25\"/></body></html>",
            "/test/test",
        )
        .unwrap();

        let currency_target = price_source.get_price(&doc_and_path).unwrap();
        let currency_expected = Currency::from_str("$1.25").unwrap();

        assert_eq!(currency_target, currency_expected);
    }

    #[test]
    fn test_price_from_html_selector() {
        let config_yml = r#"
        !FromHtmlAttribute div#test:::data-price
        "#;

        let price_source: PriceSource = serde_yml::from_str(config_yml).unwrap();

        let document =
            parse("<html><head></head><body><div id=test data-price=\"$1.25\"/></body></html>")
                .unwrap()[0]
                .clone();

        let doc_and_path = DocumentAndPath::new_from_html_and_path_str(
            "<html><head></head><body><div id=test data-price=\"$1.25\"/></body></html>",
            "/test/test",
        )
        .unwrap();

        let currency_target = price_source.get_price(&doc_and_path).unwrap();
        let currency_expected = Currency::from_str("$1.25").unwrap();

        assert_eq!(currency_target, currency_expected);
    }
}
