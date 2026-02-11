use std::convert::Into;
use std::io::Write;
use xml::EventWriter;
use xml::writer::{XmlEvent as XmlWriterEvent};
use crate::container::Container;
use crate::fodt_xml_write::FlatOdtXmlWrite;
use crate::text::Text;

pub struct Paragraph {
    pub style_name: Option<String>,
    pub content: Container
}

impl Paragraph {
    pub fn new() -> Self {
        Self {
            style_name: None,
            content: Container::new()
        }
    }

    /// Helper to create a simple paragraph with some text and a given style.
    pub fn from_text_and_style<T: Into<String>>(text: T, style_name: T) -> Self {
        let mut par = Paragraph::new();
        par.content.add(Text {text: text.into()});
        par.style_name = Some(style_name.into());
        par
    }
}

impl FlatOdtXmlWrite for Paragraph {
    fn write_flat_odt_xml(&self, writer: &mut EventWriter<&mut dyn Write>) -> xml::writer::Result<()> {
        let mut start = XmlWriterEvent::start_element("text:p");
        start = self.add_optional_string_attributes(start);
        writer.write(start)?;
        self.content.write_flat_odt_xml(writer)?;
        writer.write(XmlWriterEvent::end_element())
    }

    fn optional_string_attributes(&self) -> Vec<(&'static str, Option<&str>)> {
        vec![
            ("text:style_name", self.style_name.as_deref())
        ]
    }
}