use crate::{data::VOID_TAGS, Doctype, Element, Node};

/// Stringify into html.
pub trait Htmlifiable {
    /// Convert the object to html string.
    ///
    /// ```
    /// use html_editor::{Node, Element};
    /// use html_editor::operation::*;
    ///
    /// let node: Node = Node::new_element(
    ///     "script",
    ///     vec![
    ///         ("src", "index.js"),
    ///         ("defer", "")
    ///     ],
    ///     vec![]
    /// );
    /// assert_eq!(node.html(), r#"<script src="index.js" defer></script>"#);
    ///
    /// let nodes: Vec<Node> = vec![node.clone()];
    /// assert_eq!(nodes.html(), r#"<script src="index.js" defer></script>"#);
    ///
    /// let element: Element = node.into_element();
    /// assert_eq!(element.html(), r#"<script src="index.js" defer></script>"#);
    /// ```
    fn html(&self) -> String;
}

impl Htmlifiable for Element {
    fn html(&self) -> String {
        let children_html = match self.name.as_str() {
            "style" | "script" => {
                // <style> and <script> tags should not have their contents escaped
                let mut html = String::new();
                for node in &self.children {
                    if let Node::Text(text) = node {
                        html.push_str(text.as_str());
                    } else {
                        html.push_str(node.html().as_str());
                    }
                }
                html
            }
            _ => self.children.html(),
        };

        if self.attrs.is_empty() {
            return if VOID_TAGS.contains(&self.name.as_str()) {
                format!("<{}>", self.name)
            } else {
                format!("<{}>{}</{}>", self.name, children_html, self.name)
            };
        }
        let attrs = self
            .attrs
            .iter()
            .map(|(k, v)| {
                if v.is_empty() {
                    k.to_string()
                } else {
                    format!(r#"{}="{}""#, k, html_escape::encode_double_quoted_attribute(&v).into_owned())
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        if VOID_TAGS.contains(&self.name.as_str()) {
            format!("<{} {}>", self.name, attrs,)
        } else {
            format!(
                "<{} {}>{}</{}>",
                self.name,
                attrs,
                children_html,
                self.name
            )
        }
    }
}

impl Htmlifiable for Node {
    fn html(&self) -> String {
        match self {
            Node::Element(element) => element.html(),
            Node::Text(text) => html_escape::encode_text(text).into_owned(),
            Node::Comment(comment) => format!("<!--{}-->", comment),
            Node::Doctype(doctype) => match &doctype {
                Doctype::Html => "<!DOCTYPE html>".to_string(),
                Doctype::Xml { version, encoding } => {
                    format!(r#"<?xml version="{}" encoding="{}"?>"#, version, encoding)
                }
            },
            Node::RawHTML(html) => html.to_owned(),
        }
    }
}

impl Htmlifiable for Vec<Node> {
    fn html(&self) -> String {
        let mut html = String::new();
        for node in self {
            html.push_str(node.html().as_str());
        }
        html
    }
}
