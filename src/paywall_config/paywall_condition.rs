use html_editor::operation::{Queryable, Selector};
use html_editor::parse;
use regex::Regex;
use serde::{Deserialize, Deserializer};

use crate::documents::{UrlPath, DocumentAndPath, RequestableDoc, RequestableDoc::HtmlNode};

/// Conditions which specify a given [document and path](DocumentAndPath) to be a paywall
#[derive(Deserialize)]
pub enum PaywallCondition {
    #[serde(deserialize_with = "deserialize_regex")]
    HasRegexPath(Regex),
    #[serde(deserialize_with = "deserialize_css_selector")]
    MatchesCssSelector(Selector),
}

impl PaywallCondition {
    /// Check if either [document and/or path](DocumentAndPath) match a paywall condition
    pub fn is_paywalled(&self, doc_and_path: &DocumentAndPath) -> bool {
        let url_path = doc_and_path.get_url_path_as_str();
        let reqdoc = doc_and_path.get_document();

        match (self, reqdoc) {
            (PaywallCondition::HasRegexPath(regex), _) => regex.is_match(&url_path),
            (PaywallCondition::MatchesCssSelector(selector), HtmlNode(node)) => {
                node.query(&selector).is_some()
            } //(_, _) => false,
        }
    }
}

fn deserialize_regex<'de, D>(deserializer: D) -> Result<Regex, D::Error>
where
    D: Deserializer<'de>,
{
    let regex_str: String = String::deserialize(deserializer)?;
    return Regex::new(&regex_str).map_err(serde::de::Error::custom);
}

fn deserialize_css_selector<'de, D>(deserializer: D) -> Result<Selector, D::Error>
where
    D: Deserializer<'de>,
{
    let css_selector: &str = &String::deserialize(deserializer)?;
    return Ok(Selector::from(css_selector));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_has_regex_path_has_paywall_true() {
        let config_yml = r#"
        !HasRegexPath "^/premium/.*$"
        "#;

        let condition: PaywallCondition = serde_yml::from_str(config_yml).unwrap();

        let document = parse("<html><head></head><body></body></html>");
        let node = HtmlNode(document.unwrap()[0].clone());

        let doc_and_path =
            DocumentAndPath::new_from_doc_and_path_str(&node, "/premium/asdf").unwrap();

        assert!(condition.is_paywalled(&doc_and_path));
    }

    #[test]
    fn test_path_has_regex_path_has_paywall_false() {
        let config_yml = r#"
        !HasRegexPath "^/premium/.*$"
        "#;

        let condition: PaywallCondition = serde_yml::from_str(config_yml).unwrap();

        let document = parse("<html><head></head><body></body></html>");
        let node = HtmlNode(document.unwrap()[0].clone());

        let doc_and_path =
            DocumentAndPath::new_from_doc_and_path_str(&node, "/premiu/asdf").unwrap();

        assert!(!condition.is_paywalled(&doc_and_path));
    }

    #[test]
    fn test_path_has_css_selector_true() {
        let config_yml = r#"
        !MatchesCssSelector "body"
        "#;

        let condition: PaywallCondition = serde_yml::from_str(config_yml).unwrap();

        let document = parse("<html><head></head><body></body></html>");
        let node = HtmlNode(document.unwrap()[0].clone());

        let doc_and_path =
            DocumentAndPath::new_from_doc_and_path_str(&node, "/premiu/asdf").unwrap();

        assert!(condition.is_paywalled(&doc_and_path));
    }

    #[test]
    fn test_path_has_css_selector_false() {
        let config_yml = r#"
        !MatchesCssSelector "bod"
        "#;

        let condition: PaywallCondition = serde_yml::from_str(config_yml).unwrap();

        let document = parse("<html><head></head><body></body></html>");
        let node = HtmlNode(document.unwrap()[0].clone());

        let doc_and_path =
            DocumentAndPath::new_from_doc_and_path_str(&node, "/premiu/asdf").unwrap();

        assert!(!condition.is_paywalled(&doc_and_path));
    }
}
