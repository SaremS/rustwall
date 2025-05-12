pub mod paywall_condition;
pub mod requestable_doc;
pub mod url_path;
pub mod currency_wrapper;

pub use paywall_condition::PaywallCondition;
pub use requestable_doc::RequestableDoc;
pub use url_path::UrlPath;
pub use currency_wrapper::CurrencyWrapper;

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
    FromPaywall,
}


struct PaywallElement {
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
    }

    #[test]
    fn test_price_from_html_selector() {
        let config_yml = r#"
        !FromHtmlAttribute div#test:::data-price
        "#;

        let price_source: PriceSource = serde_yml::from_str(config_yml).unwrap();
    }
}
