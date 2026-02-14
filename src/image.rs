use std::io::Write;
use std::path::PathBuf;
use xml::EventWriter;
use crate::fodt_xml_write::FlatOdtXmlWrite;
use xml::writer::{XmlEvent as XmlWriterEvent};
use base64::prelude::*;
use strum_macros::{AsRefStr, EnumString};

pub struct Image {
    pub path: PathBuf,
    pub width_cm: f32,
    pub height_cm: f32,
    pub anchor: ImageAnchor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ImageAnchor {
    AsChar,
    Paragraph,
    Page,
}

impl Image {
    pub fn new(path: &PathBuf, width_cm: f32, height_cm: f32, anchor: ImageAnchor) -> Self {
        Self {
            path: path.clone(),
            width_cm,
            height_cm,
            anchor
        }
    }
    fn mime_type(&self) -> &'static str {
        match self.path.extension().and_then(|e| e.to_str()) {
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("svg") => "image/svg+xml",
            _ => "application/octet-stream",
        }
    }
}

impl FlatOdtXmlWrite for Image {
    fn write_flat_odt_xml(&self, writer: &mut EventWriter<&mut dyn Write>) -> xml::writer::Result<()> {

        let image_data = std::fs::read(&self.path)
            .expect("Falha ao ler imagem");

        let base64_data = BASE64_STANDARD.encode(image_data);

        writer.write(
            XmlWriterEvent::start_element("draw:frame")
                .attr("text:anchor-type", self.anchor.as_ref())
                .attr("svg:width", &format!("{}cm", self.width_cm))
                .attr("svg:height", &format!("{}cm", self.height_cm))
        )?;

        writer.write(
            XmlWriterEvent::start_element("draw:image")
                .attr("draw:mime-type", self.mime_type())
        )?;

        writer.write(XmlWriterEvent::start_element("office:binary-data"))?;
        writer.write(XmlWriterEvent::characters(base64_data.as_str()))?;
        writer.write(XmlWriterEvent::end_element())?; // binary-data

        writer.write(XmlWriterEvent::end_element())?; // draw:image
        writer.write(XmlWriterEvent::end_element()) // draw:frame
    }
}