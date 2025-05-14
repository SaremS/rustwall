use html_editor::Node;

use super::{UrlPath, UrlPathError};

#[derive(Clone, Debug)]
pub struct DocumentAndPath {
    document: RequestableDoc,
    url_path: UrlPath,
}

impl DocumentAndPath {
    pub fn new_from_doc_and_path_str(
        document: &RequestableDoc,
        path: &str,
    ) -> Result<DocumentAndPath, UrlPathError> {
        let url_path = UrlPath::new(path);

        match url_path {
            Ok(up) => Ok(DocumentAndPath {
                document: document.clone(),
                url_path: up,
            }),
            Err(e) => Err(e),
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
}
