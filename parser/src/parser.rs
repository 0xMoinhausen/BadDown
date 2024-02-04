use std::{iter, mem, str::Chars};
use html::{element::*, HtmlDocument};

use crate::html::{HtmlBody, HtmlHead};

mod html;

pub struct Parser<'a> {
    input_chars: iter::Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn parse(input_chars: iter::Peekable<Chars<'a>>, file_stem: String) -> HtmlDocument {
        let mut parser = Self { input_chars };
        let head = HtmlHead::new(file_stem);
        let body = HtmlBody::new(parser.parse_top_level_elements());
        HtmlDocument::new(head, body)
    }

    fn parse_top_level_elements(&mut self) -> Vec<Box<dyn HtmlElement>> {
        let mut elements: Vec<Box<dyn HtmlElement>> = Vec::new();

        while let Some(current_char) = self.input_chars.peek() {
            let element: Box<dyn HtmlElement> = match current_char {
                '°' => {
                    self.input_chars.next();
                    let heading_type = self.parse_heading_type();
                    let content = self.parse_text_until('\n');
                    Box::new(Heading::new(heading_type, content))
                },
                '{' => {
                    self.input_chars.next();
                    Box::new(self.parse_list('{', '}', BulletList::new))
                },
                '[' => {
                    self.input_chars.next();
                    Box::new(self.parse_list('[', ']', NumberedList::new))
                }
                '@' => {
                    self.input_chars.next();
                    Box::new(self.parse_image())
                },
                _ => Box::new(self.parse_paragraph()),
            };
            elements.push(element)
        }
        elements
    }
    
    fn parse_paragraph(&mut self) -> Paragraph {
        let until_chars = vec!['{', '\n', '['];
        let mut content = self.parse_elements_until(&until_chars);
        
        while self.input_chars.peek() == Some(&'\n') {
            self.input_chars.next();
            if let Some(current_char) = self.input_chars.peek() {
                if until_chars.contains(current_char) {
                    if *current_char == '\n' {
                        self.input_chars.next();
                    }
                    break;
                }
            }
            content.push(Box::new(NewLine));
            content.append(&mut self.parse_elements_until(&until_chars));
        }
        Paragraph::new(content)
    }

    fn parse_elements_until(&mut self, until_chars: &Vec<char>) -> Vec<Box<dyn HtmlElement>> {
        let mut elements: Vec<Box<dyn HtmlElement>> = Vec::new();
        let mut text = String::new();

        while let Some(current_char) = self.input_chars.peek() {
            if until_chars.contains(current_char) {
                break;
            }
            match current_char {
                '°' => {
                    break;
                },
                '>' => {
                    elements.push(Box::new(Text::new(mem::replace(&mut text, String::new()))));
                    self.input_chars.next();
                    if self.input_chars.peek() == Some(&'>') {
                        self.input_chars.next();
                        elements.push(Box::new(self.parse_link()));
                    } else {
                        self.input_chars.next();
                        text.push('>');
                    }
                },
                '@' => {
                    elements.push(Box::new(Text::new(mem::replace(&mut text, String::new()))));
                    self.input_chars.next();
                    elements.push(Box::new(self.parse_image()));
                },
                '*' => {
                    let content = self.content_from_parse_element_until(&mut elements, vec!['*'], &mut text);
                    elements.push(Box::new(Bold::new(content)));
                }
                '_' => {
                    let content = self.content_from_parse_element_until(&mut elements, vec!['_'], &mut text);
                    elements.push(Box::new(Cursive::new(content)));
                }
                '\'' => {
                    let content = self.content_from_parse_element_until(&mut elements, vec!['\''], &mut text);
                    elements.push(Box::new(Monospace::new(content)));
                },
                '|' => {
                    self.input_chars.next();
                    if self.input_chars.peek() == Some(&'-') {
                        self.input_chars.next();
                        elements.push(Box::new(HorizontalRule));
                    } else {
                        text.push('|')
                    }
                },
                '\n' => {
                    self.input_chars.next();
                    self.add_text_to_elements(&mut text, &mut elements);
                    let current_char = self.input_chars.peek();
                    match current_char {
                        Some(&'{') => {
                            self.input_chars.next();
                            elements.push(Box::new(self.parse_list('{', '}', BulletList::new)));
                        },
                        Some(&'[') => {
                           self.input_chars.next();
                           elements.push(Box::new(self.parse_list('[', ']', BulletList::new)));
                        },
                        None | Some(&'°') => (),
                        _ => elements.push(Box::new(NewLine)),
                    }
                },
                '\\' => {
                    self.input_chars.next();
                    text.push(self.input_chars.next().unwrap_or('\\'));
                }, 
                _ => {
                    text.push(self.input_chars.next().expect("Unexpected end of input after peeking a character."));
                },
                
            };
        }
        self.add_text_to_elements(&mut text, &mut elements);
        elements
    }

    fn content_from_parse_element_until(&mut self, elements: &mut Vec<Box<dyn HtmlElement>>, until_chars: Vec<char>, text: &mut String) -> Vec<Box<dyn HtmlElement>> {
        self.input_chars.next();
        self.add_text_to_elements(text, elements);
        let content = self.parse_elements_until(&until_chars);
        self.input_chars.next();
        content
    }

    fn parse_list<F, T>(&mut self, begin_char: char, end_char: char, list_constructor: F) -> T
        where F: Fn(Vec<Vec<Box<dyn HtmlElement>>>) -> T,
              T: HtmlElement,
    {
        let mut list_items = Vec::new();
        loop {
            let item = self.parse_elements_until(&vec![end_char]);
            list_items.push(item);
            self.input_chars.next();
            if self.input_chars.peek() != Some(&'\n') {
                break;
            }
            self.input_chars.next();
            let peek = self.input_chars.peek();
            if peek != Some(&begin_char) {
                break;
            }
            self.input_chars.next();
        }
        list_constructor(list_items)
    }

    fn parse_link(&mut self)  -> Link {
        let name = self.parse_text_until('~');
        let mut link = String::new();
        while let Some(current_char) = self.input_chars.next() {
            if current_char == '<' && self.input_chars.peek() == Some(&'<') {
                self.input_chars.next();
                break;
            }
            link.push(current_char)
        }
        Link::new(name, link)
    }

    fn parse_image(&mut self)  -> Image {
        let name = self.parse_text_until('~');
        let src = self.parse_text_until('@');
        Image::new(name, src)
    }

    fn add_text_to_elements(&self, text: &mut String, elements: &mut Vec<Box<dyn HtmlElement>>) {
        elements.push(Box::new(Text::new(mem::replace(text, String::new()))));
    }

    fn parse_heading_type(&mut self) -> HeadingType {
        let mut count = 1;
        while Some(&'°') == self.input_chars.peek() {
            self.input_chars.next();
            count += 1;
        };

        match count {
            1 => HeadingType::H1,
            2 => HeadingType::H2,
            3 => HeadingType::H3,
            4 => HeadingType::H4,
            5 => HeadingType::H5,
            _ => HeadingType::H6,
        }
    }

    fn parse_text_until(&mut self, until_char: char) -> String {
        let mut text = String::new();
        while let Some(current_char) = self.input_chars.next() {
            if current_char == until_char {
                break;
            }
            text.push(current_char)
        }
        text
    }
}