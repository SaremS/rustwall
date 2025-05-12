pub mod currency_wrapper;
pub mod paywall_condition;
pub mod requestable_doc;
pub mod url_path;

pub use currency_wrapper::CurrencyWrapper;
pub use paywall_condition::PaywallCondition;
pub use requestable_doc::RequestableDoc;
pub use url_path::UrlPath;

use crate::utils::HtmlAttributeSelector;

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

pub struct PaywallElement {
    paywall_conditions: Vec<PaywallCondition>,
    price_source: PriceSource,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_source_hard() {
        let config_yml = r#"
        !Hard $1.25
        "#;

        let price_source: PriceSource = serde_yml::from_str(config_yml).unwrap();

        let currency_config = r#"
        $1.25
        "#;

        let currency_wrapper_target: CurrencyWrapper = serde_yml::from_str(config_yml).unwrap();

        match price_source {
            PriceSource::Hard(c) => assert_eq!(c, currency_wrapper_target),
            _ => panic!(),
        }
    }

    #[test]
    fn test_price_from_html_selector() {
        let config_yml = r#"
        !FromHtmlAttribute div#test:::data-price
        "#;

        let price_source: PriceSource = serde_yml::from_str(config_yml).unwrap();
    }
}
