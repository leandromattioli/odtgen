use std::collections::HashMap;
use std::io::Write;
use xml::EventWriter;
use crate::fodt_xml_write::FlatOdtXmlWrite;
use crate::style::Style;
use crate::stylesheet_parser::StylesheetParser;

const WRITER_STYLES_YAML: &str = include_str!("../assets/writer_default_stylesheet.yaml");

pub struct Stylesheet {
    styles: HashMap<String, Style>,
}

impl Stylesheet {
    pub fn new() -> Self {
        Stylesheet {
            styles: HashMap::new(),
        }
    }

    pub(crate) fn write_styles(&self, writer: &mut EventWriter<&mut dyn Write>) -> xml::writer::Result<()> {
        for (_key, style) in &self.styles {
            if style.automatic {
                continue;
            }
            style.write_flat_odt_xml(writer)?;
        }
        Ok(())
    }

    pub(crate) fn write_automatic_styles(&self, writer: &mut EventWriter<&mut dyn Write>) -> xml::writer::Result<()> {
        for (_key, style) in &self.styles {
            if !style.automatic {
                continue;
            }
            style.write_flat_odt_xml(writer)?;
        }
        Ok(())
    }

    /// Add a style to the stylesheet. A previous one with the same will be overridden.
    pub fn add_style(&mut self, style: Style) {
        let name = style.name();
        self.styles.insert(name.to_string(), style);
    }

    /// Append another stylesheet to this one, overwriting styles with the same name.
    ///
    /// Documents are created with a default stylesheet. This method is the recommended way
    /// to extend/modify the default stylesheet.
    #[warn(unused)]
    pub fn extend(&mut self, other: Stylesheet) {
        self.styles.extend(other.styles);
    }
}

impl Default for Stylesheet {
    fn default() -> Self {
        let stylesheet = StylesheetParser::parse_yaml(WRITER_STYLES_YAML);
        stylesheet.expect("Fail to read default stylesheet!")
    }
}