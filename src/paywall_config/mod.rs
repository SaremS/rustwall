pub mod paywall_condition;
pub mod requestable_doc;
pub mod url_path;

pub use paywall_condition::PaywallCondition;
pub use requestable_doc::RequestableDoc;
pub use url_path::UrlPath;

use currency::Currency;
use html_editor::operation::Selector;
use serde::Deserialize;

pub struct PaywallConfigV1 {
    paths: Vec<PaywallElement>,
}

pub enum PriceSource {
    Hard(Currency),
    FromHtmlSelector(Selector),
    FromPaywall,
}

struct PaywallElement {
    paywall_conditions: Vec<PaywallCondition>,
    paywall_identifier: String,
    price_source: PriceSource,
}

#[cfg(test)]
mod tests {
    use super::*;

    /*#[test]
    fn test_price_source_hard() {
        let config_yml = r#"
        !Hard $1.25
        "#;

        let price_source: PriceSource = serde_yml::from_str(config_yml).unwrap();
    }

    #[test]
    fn test_price_from_html_selector() {
        let config_yml = r#"
        !FromHtmlSelector .paywall
        "#;

        let price_source: PriceSource = serde_yml::from_str(config_yml).unwrap();
    }*/
}
