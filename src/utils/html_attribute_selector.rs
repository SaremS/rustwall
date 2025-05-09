use std::fmt;
use html_editor::operation::Selector;
use serde::{Deserialize, Deserializer};
use serde::de::{self, Visitor, SeqAccess, MapAccess};

pub struct HtmlAttributeSelector {
    html_selector: Selector,
    attribute_name: String
}

impl<'de> Deserialize<'de> for HtmlAttributeSelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HtmlAttributeSelectorVisitor;

        impl<'de> Visitor<'de> for HtmlAttributeSelectorVisitor {
            type Value = HtmlAttributeSelector;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string in the format 'html_selector:::attribute_name'")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let parts: Vec<&str> = value.splitn(2, ":::").collect();
                
                let html_selector = Selector::from(parts[0]);
                let attribute_name = parts[1].to_string();

                return Ok(HtmlAttributeSelector{
                    html_selector,
                    attribute_name,
                });
            }
        }

        deserializer.deserialize_string(HtmlAttributeSelectorVisitor)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_html_attribute_selector_correctly() {
        let config_yml = r#"
        div#test:::data-test
        "#;

        let selector: HtmlAttributeSelector = serde_yml::from_str(config_yml).unwrap();
    }
}
