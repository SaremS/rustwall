pub mod currency_wrapper;
pub mod paywall_condition;
pub mod requestable_doc;
pub mod url_path;

pub use currency_wrapper::CurrencyWrapper;
pub use paywall_condition::PaywallCondition;
pub use requestable_doc::{DocumentAndPath, RequestableDoc};
pub use url_path::{UrlPath, UrlPathError};

use crate::utils::{HtmlAttributeSelector, HtmlAttributeSelectorError};

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

impl PriceSource {
    pub fn get_price(&self, doc: &RequestableDoc) -> Result<Currency, PriceSourceExtractError> {
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

/*impl PaywallElement {
    pub fn get_price(&self, doc: &RequestableDoc) -> Option<Currency> {
        self.paywall_conditions.iter().map(|x| x.is)
    }
}*/

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

        let condition: PaywallElement = serde_yml::from_str(config_yml).unwrap();
    }

    #[test]
    fn test_price_source_hard() {
        let config_yml = r#"
        !Hard $1.25
        "#;

        let price_source: PriceSource = serde_yml::from_str(config_yml).unwrap();

        let document = parse("<html><head></head><body></body></html>").unwrap()[0].clone();
        let html_node = RequestableDoc::HtmlNode(document);

        let currency_target = price_source.get_price(&html_node).unwrap();
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
        let html_node = RequestableDoc::HtmlNode(document);

        let currency_target = price_source.get_price(&html_node).unwrap();
        let currency_expected = Currency::from_str("$1.25").unwrap();

        assert_eq!(currency_target, currency_expected);
    }
}
