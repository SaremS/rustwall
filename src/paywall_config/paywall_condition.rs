use html_editor::operation::{Queryable, Selector};
use html_editor::{parse, Node};
use regex::Regex;
use serde::{Deserialize, Deserializer};

use super::UrlPath;
use super::{RequestableDoc, RequestableDoc::HtmlNode};

#[derive(Deserialize)]
pub enum PaywallCondition {
    #[serde(deserialize_with = "deserialize_regex")]
    HasRegexPath(Regex),
    #[serde(deserialize_with = "deserialize_css_selector")]
    MatchesCssSelector(Selector),
}

impl PaywallCondition {
    pub fn is_paywalled(&self, url_path: &UrlPath, reqdoc: &RequestableDoc) -> bool {
        match (self, reqdoc) {
            (PaywallCondition::HasRegexPath(regex), _) => regex.is_match(&url_path.get_path()),
            (PaywallCondition::MatchesCssSelector(selector), HtmlNode(node)) => {
                node.query(&selector).is_some()
            }
            (_, _) => false,
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

        let path = UrlPath::new("/premium/asdf").unwrap();
        let document = parse("<html><head></head><body></body></html>");

        let node = HtmlNode(document.unwrap()[0].clone());

        assert!(condition.is_paywalled(&path, &node));
    }

    #[test]
    fn test_path_has_regex_path_has_paywall_false() {
        let config_yml = r#"
        !HasRegexPath "^/premium/.*$"
        "#;

        let condition: PaywallCondition = serde_yml::from_str(config_yml).unwrap();

        let path = UrlPath::new("/premiu/asdf").unwrap();
        let document = parse("<html><head></head><body></body></html>");

        let node = HtmlNode(document.unwrap()[0].clone());

        assert!(!condition.is_paywalled(&path, &node));
    }

    #[test]
    fn test_path_has_css_selector_true() {
        let config_yml = r#"
        !MatchesCssSelector "body"
        "#;

        let condition: PaywallCondition = serde_yml::from_str(config_yml).unwrap();

        let path = UrlPath::new("/premiu/asdf").unwrap();
        let document = parse("<html><head></head><body></body></html>");

        let node = HtmlNode(document.unwrap()[0].clone());

        assert!(condition.is_paywalled(&path, &node));
    }

    #[test]
    fn test_path_has_css_selector_false() {
        let config_yml = r#"
        !MatchesCssSelector "bod"
        "#;

        let condition: PaywallCondition = serde_yml::from_str(config_yml).unwrap();

        let path = UrlPath::new("/premiu/asdf").unwrap();
        let document = parse("<html><head></head><body></body></html>");

        let node = HtmlNode(document.unwrap()[0].clone());

        assert!(!condition.is_paywalled(&path, &node));
    }
}
