use html_editor::{parse, Node};

use super::{UrlPath, UrlPathError};

#[derive(Clone, Debug)]
pub struct DocumentAndPath {
    document: RequestableDoc,
    url_path: UrlPath,
}

#[derive(Debug)]
pub enum DocumentAndPathError {
    UrlPathInvalidFormat(String),
    UrlPathOrHtmlError((Option<String>, Option<String>)),
}

impl DocumentAndPath {
    pub fn new_from_doc_and_path_str(
        document: &RequestableDoc,
        path: &str,
    ) -> Result<DocumentAndPath, DocumentAndPathError> {
        let url_path = UrlPath::new(path);

        match url_path {
            Ok(up) => Ok(DocumentAndPath {
                document: document.clone(),
                url_path: up,
            }),
            Err(UrlPathError::InvalidFormat(e)) => {
                Err(DocumentAndPathError::UrlPathInvalidFormat(e))
            }
        }
    }

    pub fn new_from_html_and_path_str(
        html_str: &str,
        path: &str,
    ) -> Result<DocumentAndPath, DocumentAndPathError> {
        let url_path = UrlPath::new(path);
        let node = parse(html_str);

        match (url_path, node) {
            (Ok(path), Ok(node)) => Ok(DocumentAndPath {
                document: RequestableDoc::HtmlNode(node[0].clone()),
                url_path: path,
            }),
            (Err(UrlPathError::InvalidFormat(e)), Ok(_)) => {
                Err(DocumentAndPathError::UrlPathOrHtmlError((Some(e), None)))
            }
            (Err(UrlPathError::InvalidFormat(e1)), Err(e2)) => Err(
                DocumentAndPathError::UrlPathOrHtmlError((Some(e1), Some(e2.to_string()))),
            ),
            (Ok(_), Err(e2)) => Err(DocumentAndPathError::UrlPathOrHtmlError((
                None,
                Some(e2.to_string()),
            ))),
        }
    }

    pub fn new(document: &RequestableDoc, url_path: &UrlPath) -> DocumentAndPath {
        return DocumentAndPath {
            document: document.clone(),
            url_path: url_path.clone(),
        };
    }

    pub fn get_document(&self) -> &RequestableDoc {
        return &self.document;
    }

    pub fn get_url_path(&self) -> &UrlPath {
        return &self.url_path;
    }

    pub fn get_url_path_as_str(&self) -> &str {
        return self.url_path.get_path();
    }
}

#[derive(Clone, Debug)]
pub enum RequestableDoc {
    HtmlNode(Node),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_and_path_happy_path() {
        let node: Node = Node::new_element(
            "h1",
            vec![("class", "title")],
            vec![Node::Text("Hello, world!".to_string())],
        );
        let doc = RequestableDoc::HtmlNode(node);

        let path_str = "/test/test";

        let doc_and_path = DocumentAndPath::new_from_doc_and_path_str(&doc, path_str).unwrap();
    }

    #[test]
    fn test_document_and_path_sad_path() {
        let node: Node = Node::new_element(
            "h1",
            vec![("class", "title")],
            vec![Node::Text("Hello, world!".to_string())],
        );
        let doc = RequestableDoc::HtmlNode(node);

        let path_str = "test/test";

        let doc_and_path = DocumentAndPath::new_from_doc_and_path_str(&doc, path_str);

        match doc_and_path {
            Ok(_) => panic!(),
            _ => {}
        }
    }

    #[test]
    fn test_document_and_path_from_strs_success() {
        let path = "/test/test";
        let html = "<html><head></head><body></body></html>";

        let doc_and_path = DocumentAndPath::new_from_html_and_path_str(html, path);

        match doc_and_path {
            Err(_) => panic!(),
            _ => {}
        }
    }

    #[test]
    fn test_document_and_path_from_strs_2errs() {
        let path = "test/test";
        let html = "<html><head</head><body></body></html>";

        let doc_and_path = DocumentAndPath::new_from_html_and_path_str(html, path);

        match doc_and_path {
            Ok(_) => panic!(),
            Err(DocumentAndPathError::UrlPathOrHtmlError((None, None))) => panic!(),
            Err(DocumentAndPathError::UrlPathOrHtmlError((Some(_), None))) => panic!(),
            Err(DocumentAndPathError::UrlPathOrHtmlError((None, Some(_)))) => panic!(),
            _ => {}
        }
    }
}
