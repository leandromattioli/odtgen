use std::io::Write;
use xml::EventWriter;
use xml::writer::{XmlEvent as XmlWriterEvent};
use crate::fodt_xml_write::FlatOdtXmlWrite;

pub struct Text {
    pub text: String
}

impl FlatOdtXmlWrite for Text {
    fn write_flat_odt_xml(&self, writer: &mut EventWriter<&mut dyn Write>) -> xml::writer::Result<()> {
        writer.write(XmlWriterEvent::characters(&self.text))
    }
}
