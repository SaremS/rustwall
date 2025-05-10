use html_editor::operation::{Queryable, Selector};
use html_editor::{Node,parse};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::error::Error as StdError;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct HtmlAttributeSelector {
    html_selector: Selector,
    attribute_name: String,
}

#[derive(Debug)]
pub enum HtmlAttributeSelectorError {
    ElementNotFound,
    AttributeNotFound(String),
    ConversionError { value: String, target_type: String },
}

impl fmt::Display for HtmlAttributeSelectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HtmlAttributeSelectorError::ElementNotFound => {
                write!(f, "No HTML element found matching the selector")
            }
            HtmlAttributeSelectorError::AttributeNotFound(attr_name) => {
                write!(
                    f,
                    "Attribute '{}' not found on the selected element",
                    attr_name
                )
            }
            HtmlAttributeSelectorError::ConversionError { value, target_type } => {
                write!(
                    f,
                    "Failed to convert attribute value '{}' to type {}",
                    value, target_type
                )
            }
        }
    }
}

impl HtmlAttributeSelector {
    pub fn get_attribute<T>(&self, html_node: &Node) -> Result<T, HtmlAttributeSelectorError>
    where
        T: FromStr,
    {
        let target_element = html_node
            .query(&self.html_selector)
            .ok_or(HtmlAttributeSelectorError::ElementNotFound)?;

        let attrs = &target_element.attrs;

        let target_attribute = attrs
            .into_iter()
            .find(|x| x.0 == self.attribute_name.clone())
            .map(|x| (&x.1).to_string())
            .ok_or(HtmlAttributeSelectorError::AttributeNotFound(
                self.attribute_name.clone(),
            ))?;

        let target_output = target_attribute.parse::<T>().map_err(|_| {
            HtmlAttributeSelectorError::ConversionError {
                value: target_attribute.to_string(),
                target_type: std::any::type_name::<T>().to_string(),
            }
        });

        return target_output;
    }
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

                if parts.len() != 2 {
                    return Err(de::Error::invalid_length(parts.len(), &self));
                }

                let html_selector = Selector::from(parts[0]);
                let attribute_name = parts[1].to_string();

                return Ok(HtmlAttributeSelector {
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
    fn test_deserialize_html_attribute_selector_valid_input() {
        let config_yml = r#"
        div#test:::data-test
        "#;

        let selector: Result<HtmlAttributeSelector, _> = serde_yml::from_str(config_yml);
        assert!(selector.is_ok());
    }

    #[test]
    fn test_deserialize_html_attribute_selector_invalid_input() {
        let config_yml = r#"
        div#test:data-test
        "#;

        let selector: Result<HtmlAttributeSelector, _> = serde_yml::from_str(config_yml);
        assert!(selector.is_err());
    }

    #[test]
    fn test_get_attribute_success() {
        let config_yml = r#"
        div#test:::data-test
        "#;

        let selector: HtmlAttributeSelector= serde_yml::from_str(config_yml).unwrap();
        let document = parse("<html><head></head><body><div id=\"test\" data-test=\"123\"/></body></html>");

        let node = document.unwrap()[0].clone();

        let result: u16 = selector.get_attribute::<u16>(&node).unwrap();

        assert_eq!(result, 123);
    }
    
    #[test]
    fn test_get_attribute_element_not_found() {
        let config_yml = r#"
        div#test:::data-test
        "#;

        let selector: HtmlAttributeSelector= serde_yml::from_str(config_yml).unwrap();
        let document = parse("<html><head></head><body><div id=\"test123\" data-test=\"123\"/></body></html>");

        let node = document.unwrap()[0].clone();

        let result = selector.get_attribute::<u16>(&node);

        match result {
            Err(HtmlAttributeSelectorError::ElementNotFound) => {},
            _ => panic!()
        }
    }

    #[test]
    fn test_get_attribute_attribute_not_found() {
        let config_yml = r#"
        div#test:::data-test
        "#;

        let selector: HtmlAttributeSelector= serde_yml::from_str(config_yml).unwrap();
        let document = parse("<html><head></head><body><div id=\"test\" data-test-123=\"123\"/></body></html>");

        let node = document.unwrap()[0].clone();

        let result = selector.get_attribute::<u16>(&node);

        match result {
            Err(HtmlAttributeSelectorError::AttributeNotFound(_)) => {},
            _ => panic!()
        }
    }

    #[test]
    fn test_get_attribute_conversion_impossible() {
        let config_yml = r#"
        div#test:::data-test
        "#;

        let selector: HtmlAttributeSelector= serde_yml::from_str(config_yml).unwrap();
        let document = parse("<html><head></head><body><div id=\"test\" data-test=\"asdf\"/></body></html>");

        let node = document.unwrap()[0].clone();

        let result = selector.get_attribute::<f32>(&node);

        match result {
            Err(HtmlAttributeSelectorError::ConversionError{value: v, target_type: t}) => {},
            _ => panic!()
        }
    }
}
