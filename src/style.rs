use std::collections::HashMap;
use std::io::Write;
use xml::EventWriter;
use xml::writer::{XmlEvent as XmlWriterEvent};
use crate::fodt_xml_write::FlatOdtXmlWrite;
use strum_macros::{EnumString, AsRefStr, EnumIter};

/// A style to be used in a Stylesheet
pub struct Style {
    name: String,
    pub family: StyleFamily,
    pub automatic: bool,
    pub parent_style_name: Option<String>,
    pub next_style_name: Option<String>,
    pub display_name: Option<String>,
    pub class: Option<String>,
    pub default_outline_level: Option<u8>,
    pub properties: HashMap<StylePropertyGroup, StyleItem>,
}

impl Style {
    pub fn new(name: String, family: StyleFamily) -> Self {
        Style {
            name,
            family,
            parent_style_name: None,
            next_style_name: None,
            display_name: None,
            default_outline_level: None,
            properties: HashMap::new(),
            automatic: false,
            class: None
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl FlatOdtXmlWrite for Style {
    fn write_flat_odt_xml(&self, writer: &mut EventWriter<&mut dyn Write>) -> xml::writer::Result<()> {
        let attr_value: String;
        //Top-level element
        let mut start = XmlWriterEvent::start_element("style:style")
            .attr("style:name", &self.name)
            .attr("style:family", self.family.as_ref());
        //Optional string attributes
        start = self.add_optional_string_attributes(start);
        if let Some(ref default_outline_level) = self.default_outline_level {
            attr_value = default_outline_level.to_string();
            start = start.attr("style:default-outline-level", attr_value.as_str());
        }
        writer.write(start)?;
        //Properties
        for (group, style_item) in  self.properties.iter() {
            let tag_name = format!("style:{}", group.as_ref());
            let mut start = XmlWriterEvent::start_element(tag_name.as_str());
            for (key, value)  in &style_item.simple_attributes {
                start = start.attr(key.as_str(), value.as_str());
            }
            writer.write(start)?;
            //Inner children
            writer.write(XmlWriterEvent::end_element())?;
        }
        writer.write(XmlWriterEvent::end_element())
    }

    fn optional_string_attributes(&self) -> Vec<(&'static str, Option<&str>)> {
        vec![
            ("style:parent-style-name", self.parent_style_name.as_deref()),
            ("style:display-name", self.display_name.as_deref()),
            ("style:next-style-name", self.next_style_name.as_deref()),
            ("style:class", self.class.as_deref()),
        ]
    }
}

// ======================================================================================
// Helper Structs
// ======================================================================================

#[derive(Default)]
pub struct StyleItem {
    simple_attributes: HashMap<String, String>,
    tab_stops: Vec<TabStop>
}

impl StyleItem {
    pub fn set(&mut self, key: &str, value: &str) {
        self.simple_attributes.insert(key.to_string(), value.to_string());
    }

    pub fn add_tab_stop(&mut self, tab_stop: TabStop) {
        self.tab_stops.push(tab_stop);
    }
}

pub struct TabStop {
    pub position: String,
    pub type_: Option<String>,
    //pub leader_char: Option<String>,
}

// ======================================================================================
// Style Families
// ======================================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum StyleFamily {
    Paragraph,
    Text,
    Table,
    TableColumn,
    TableRow,
    TableCell,
}

// ======================================================================================
// Property Groups
// ======================================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, EnumIter, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum StylePropertyGroup {
    ParagraphProperties,
    TextProperties,
    TableProperties,
    TableColumnProperties,
    TableRowProperties,
    TableCellProperties,
}