use std::io::Write;
use xml::EventWriter;
use xml::writer::{XmlEvent as XmlWriterEvent};
use crate::fodt_xml_write::FlatOdtXmlWrite;

pub struct TabStop {}

impl TabStop {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl FlatOdtXmlWrite for TabStop {
    fn write_flat_odt_xml(&self, writer: &mut EventWriter<&mut dyn Write>) -> xml::writer::Result<()> {
        let start = XmlWriterEvent::start_element("text:tab");
        writer.write(start)?;
        writer.write(XmlWriterEvent::end_element())
    }
}
