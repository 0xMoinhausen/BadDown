pub trait HtmlElement {
    fn as_string(&self) -> String;
}

pub struct Text {
    content: String,
}

impl Text {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl HtmlElement for Text {
    fn as_string(&self) -> String {
        self.content.clone()
    }
}

pub enum HeadingType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

pub struct Heading {
    h_type: HeadingType,
    content: String,
}

impl Heading {
    pub fn new(h_type: HeadingType, content: String) -> Self {
        Heading {
            h_type,
            content,
        }
    }
}

impl HtmlElement for Heading {
    fn as_string(&self) -> String {
        match self.h_type {
            HeadingType::H1 => format!("<h1>{}</h1>\n", self.content),
            HeadingType::H2 => format!("<h2>{}</h2>\n", self.content),
            HeadingType::H3 => format!("<h3>{}</h3>\n", self.content),
            HeadingType::H4 => format!("<h4>{}</h4>\n", self.content),
            HeadingType::H5 => format!("<h5>{}</h5>\n", self.content),
            HeadingType::H6 => format!("<h6>{}</h6>\n", self.content),
        }
    }
}

pub struct Paragraph {
    content: Vec<Box<dyn HtmlElement>>,
}

impl Paragraph {
    pub fn new(content: Vec<Box<dyn HtmlElement>>) -> Paragraph {
        Paragraph { content }
    }
}

impl HtmlElement for Paragraph {
    fn as_string(&self) -> String {
        let content_html: Vec<String> = self.content.iter().map(|item| item.as_string()).collect();
        format!("<p>\n{}\n</p>\n", content_html.join(""))
    }
}

pub struct NewLine;

impl HtmlElement for NewLine {
    fn as_string(&self) -> String {
        "<br>\n".to_string()
    }
}

pub struct Bold {
    content: Vec<Box<dyn HtmlElement>>,
}

impl Bold {
    pub fn new(content: Vec<Box<dyn HtmlElement>>) -> Self {
        Bold { content }
    }
}

impl HtmlElement for Bold {
    fn as_string(&self) -> String {
        format!("<strong>{}</strong>", self.content.iter().map(|item| item.as_string()).collect::<String>())
    }
}

pub struct Cursive {
    content: Vec<Box<dyn HtmlElement>>,
}

impl Cursive {
    pub fn new(content: Vec<Box<dyn HtmlElement>>) -> Self {
        Cursive { content }
    }
}

impl HtmlElement for Cursive {
    fn as_string(&self) -> String {
        format!("<em>{}</em>", self.content.iter().map(|item| item.as_string()).collect::<String>())
    }
}

pub struct Monospace {
    content: Vec<Box<dyn HtmlElement>>,
}

impl Monospace {
    pub fn new(content: Vec<Box<dyn HtmlElement>>) -> Self {
        Monospace { content }
    }
}

impl HtmlElement for Monospace {
    fn as_string(&self) -> String {
        format!("<code>{}</code>", self.content.iter().map(|item| item.as_string()).collect::<String>())
    }
}

pub struct HorizontalRule;

impl HtmlElement for HorizontalRule {
    fn as_string(&self) -> String {
        "<hr>".to_string()
    }
}

pub struct BulletList {
    items: Vec<Vec<Box<dyn HtmlElement>>>,
}

impl BulletList {
    pub fn new(items: Vec<Vec<Box<dyn HtmlElement>>>) -> Self {
        BulletList { items }
    }
}

impl HtmlElement for BulletList {
    fn as_string(&self) -> String {
        let items_html: Vec<String> = self.items
            .iter()
            .map(|inner_vec| {
                let inner_html: Vec<String> = inner_vec.iter().map(|item| item.as_string()).collect();
                format!("<li>{}</li>\n", inner_html.join(""))
            })
            .collect();
        format!("<ul>\n{}</ul>\n", items_html.join(""))
    }
}

pub struct NumberedList {
    items: Vec<Vec<Box<dyn HtmlElement>>>,
}

impl NumberedList {
    pub fn new(items: Vec<Vec<Box<dyn HtmlElement>>>) -> Self {
        NumberedList { items }
    }
}

impl HtmlElement for NumberedList {
    fn as_string(&self) -> String {
        let items_html: Vec<String> = self.items
            .iter()
            .map(|inner_vec| {
                let inner_html: Vec<String> = inner_vec.iter().map(|item| item.as_string()).collect();
                format!("<li>{}</li>\n", inner_html.join(""))
            })
            .collect();
        format!("<ol>\n{}</ol>\n", items_html.join(""))
    }
}


pub struct Link {
    name: String,
    link: String,
}

impl Link {
    pub fn new(name: String, link: String) -> Self {
        Link {
            name,
            link,
        }
    }
}

impl HtmlElement for Link {
    fn as_string(&self) -> String {
        format!("<a href=\"{}\">{}</a>", self.link, self.name)
    }
}

pub struct Image {
    name: String,
    src: String,
}

impl Image {
    pub fn new(name: String, src: String) -> Self {
        Image {
            name: name,
            src: src,
        }
    }
}

impl HtmlElement for Image {
    fn as_string(&self) -> String {
        format!("<img src=\"{}\" alt=\"{}\">\n", self.src, self.name)
    }
}
