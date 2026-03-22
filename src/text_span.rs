use std::convert::Into;
use std::io::Write;
use xml::EventWriter;
use xml::writer::{XmlEvent as XmlWriterEvent};
use crate::container::Container;
use crate::fodt_xml_write::FlatOdtXmlWrite;
use crate::text::Text;

pub struct TextSpan {
    pub style_name: Option<String>,
    pub content: Container
}

impl TextSpan {
    pub fn new() -> Self {
        Self {
            style_name: None,
            content: Container::new()
        }
    }

    /// Helper to create a simple span with some text and a given style.
    pub fn from_text_and_style<T: Into<String>, U: Into<String>>(text: T, style_name: U) -> Self {
        let mut span = TextSpan::new();
        span.content.add(Text {text: text.into()});
        span.style_name = Some(style_name.into());
        span
    }

    pub fn from_text<T: Into<String>>(text: T) -> Self {
        let mut span = TextSpan::new();
        span.content.add(Text {text: text.into()});
        span
    }
}

impl FlatOdtXmlWrite for TextSpan {
    fn write_flat_odt_xml(&self, writer: &mut EventWriter<&mut dyn Write>) -> xml::writer::Result<()> {
        let mut start = XmlWriterEvent::start_element("text:span");
        start = self.add_optional_string_attributes(start);
        writer.write(start)?;
        self.content.write_flat_odt_xml(writer)?;
        writer.write(XmlWriterEvent::end_element())
    }

    fn optional_string_attributes(&self) -> Vec<(&'static str, Option<&str>)> {
        vec![
            ("text:style-name", self.style_name.as_deref())
        ]
    }
}
