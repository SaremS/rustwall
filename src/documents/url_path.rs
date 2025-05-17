use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrlPath {
    path: String,
}

#[derive(Debug, Error)]
pub enum UrlPathError {
    #[error("Invalid URL path format: {0}")]
    InvalidFormat(String),
}

impl UrlPath {
    pub fn new(path_str: &str) -> Result<Self, UrlPathError> {
        // Basic validation: must start with '/', cannot contain '?' or '#'
        if path_str.starts_with('/') && !path_str.contains('?') && !path_str.contains('#') {
            Ok(UrlPath {
                path: path_str.to_string(),
            })
        } else {
            Err(UrlPathError::InvalidFormat(path_str.to_string()))
        }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_url_path() {
        let path = "/articles/123";
        let url_path = UrlPath::new(path).unwrap();
        assert_eq!(url_path.get_path(), path);

        let root_path = "/";
        let url_path_root = UrlPath::new(root_path).unwrap();
        assert_eq!(url_path_root.get_path(), root_path);

        let complex_path = "/a/b/c-d_e.f";
        let url_path_complex = UrlPath::new(complex_path).unwrap();
        assert_eq!(url_path_complex.get_path(), complex_path);
    }

    #[test]
    fn test_invalid_url_path_no_leading_slash() {
        let path = "articles/123";
        let result = UrlPath::new(path);
        assert!(result.is_err());
        match result {
            Err(UrlPathError::InvalidFormat(p)) => assert_eq!(p, path),
            _ => panic!("Expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_invalid_url_path_contains_query() {
        let path = "/articles/123?query=param";
        let result = UrlPath::new(path);
        assert!(result.is_err());
        match result {
            Err(UrlPathError::InvalidFormat(p)) => assert_eq!(p, path),
            _ => panic!("Expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_invalid_url_path_contains_fragment() {
        let path = "/articles/123#section";
        let result = UrlPath::new(path);
        assert!(result.is_err());
        match result {
            Err(UrlPathError::InvalidFormat(p)) => assert_eq!(p, path),
            _ => panic!("Expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_empty_path_is_invalid() {
        let path = "";
        let result = UrlPath::new(path);
        assert!(result.is_err());
    }
}
