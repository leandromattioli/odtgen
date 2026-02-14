use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use crate::container::Container;
use xml::reader::{EventReader, XmlEvent as REvent, ParserConfig};
use xml::writer::{EventWriter, XmlEvent as WEvent, EmitterConfig};
use crate::fodt_xml_write::FlatOdtXmlWrite;
use crate::paragraph::Paragraph;
use crate::stylesheet::Stylesheet;

const BLANK_FODT: &str = include_str!("../assets/blank.fodt");

/// The main structure of the document tree
pub struct Document {
    pub stylesheet: Stylesheet,
    pub header: Container,
    pub body: Container
}

impl Document {

    /// Create a new blank document with the default stylesheet
    pub fn new() -> Self {
        Document {
            stylesheet: Stylesheet::default(),
            header: Container::new(),
            body: Container::new()
        }
    }

    /// Copy the XML structure until a marker (in the form of a comment) is found.
    ///
    /// # Arguments
    ///
    /// * `reader`: The XML reader pointing to the "next" event.
    /// * `writer`: An XML writer
    /// * `marker`: The marker which will make the function returns.
    ///
    /// returns: Result<(), String>
    fn copy_until_marker<R, W>(
        reader: &mut EventReader<R>,
        writer: &mut EventWriter<W>,
        marker: &str,
    ) -> Result<(), String>
    where
        R: std::io::Read,
        W: std::io::Write,
    {
        loop {
            let ev = reader.next().map_err(|e| e.to_string())?;

            match ev {
                REvent::EndDocument => panic!("Document end found!"),
                REvent::Comment(text) => {
                    if (&text).trim() == marker {
                        break
                    }
                }
                other => {
                    let ev = other.as_writer_event().ok_or("Falha ao construir XmlEvent de escrita!")?;
                    writer.write(ev).map_err(|e| e.to_string())?;
                }
            }
        }
        Ok(())
    }

    /// Write the header of the document
    fn write_header(&self, writer: &mut EventWriter<&mut dyn Write>) -> xml::writer::Result<()> {
        if self.header.len() > 0 {
            writer.write(WEvent::start_element("style:header"))?;
            self.header.write_flat_odt_xml(writer)?;
            writer.write(WEvent::end_element())?;
        }
        Ok(())
    }

    pub fn save(&mut self, output: &PathBuf) -> Result<(), String> {
        let mut file = File::create(output).map_err(|e| e.to_string())?;
        self.generate_fodt(&mut file)?;
        Ok(())
    }

    /// Export the document as Flat ODT (single XML)
    pub fn generate_fodt(&mut self, out: &mut dyn Write) -> Result<(), String> {
        let mut reader = EventReader::new_with_config(
            BLANK_FODT.as_bytes(),
            ParserConfig::new()
                .ignore_comments(false)
        );

        let mut writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(out);

        Document::copy_until_marker(&mut reader, &mut writer, "__STYLES__")?;
        self.stylesheet.write_styles(&mut writer).map_err(|e| e.to_string())?;

        Document::copy_until_marker(&mut reader, &mut writer, "__AUTOMATIC_STYLES__")?;
        self.stylesheet.write_automatic_styles(&mut writer).map_err(|e| e.to_string())?;

        Document::copy_until_marker(&mut reader, &mut writer, "__HEADER__")?;
        self.write_header(&mut writer).map_err(|e| e.to_string())?;

        //Document::copy_until_marker(&mut reader, &mut writer, "__FOOTER__")?;
        //self.write_header(&mut writer).map_err(|e| e.to_string())?;

        Document::copy_until_marker(&mut reader, &mut writer, "__BODY__")?;
        if self.body.len() == 0 {
            self.body.add(Paragraph::from_text_and_style("", "Standard"));
        }
        self.body.write_flat_odt_xml(&mut writer).map_err(|e| e.to_string())?;

        // resto do documento
        loop {
            let read_evt = reader.next().map_err(|e| e.to_string())?;
            if matches!(read_evt, REvent::EndDocument) {
                break;
            }
            let ev = read_evt.as_writer_event().unwrap();
            writer.write(ev).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}