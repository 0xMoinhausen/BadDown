use crate::HtmlElement;

pub mod element;

pub struct HtmlDocument {
    head: HtmlHead,
    body: HtmlBody,
}

impl HtmlDocument {
    pub fn new(head: HtmlHead, body: HtmlBody) -> Self {
        Self { head, body }
    }

    pub fn as_string(&self) -> String {
        format!("{}{}", self.head.as_string(), self.body.as_string())
    }
}

pub struct HtmlBody {
    elements: Vec<Box<dyn HtmlElement>>,
}

impl HtmlBody {
    pub fn new(elements: Vec<Box<dyn HtmlElement>>) -> Self {
        Self { elements }
    }

    pub fn as_string(&self) -> String {
        let elements: String = self.elements.iter().map(|e| e.as_string()).collect();
        format!("<body>\n{elements}</body>\n")
    }
}

pub struct HtmlHead {
    title: String,
}

impl HtmlHead {
    pub fn new(title: String) -> Self {
        Self { title }
    }

    pub fn as_string(&self) -> String {
        format!("<head>\n<title>{}</title>\n</head>\n", self.title)
    }
}
