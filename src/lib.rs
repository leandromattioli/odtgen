#![doc = include_str!("../README.md")]

pub mod container;
pub mod paragraph;
pub mod document;
pub mod fodt_xml_write;
pub mod style;
pub mod table;
pub mod stylesheet;
pub mod stylesheet_parser;
pub mod image;
pub mod text;
pub mod text_span;
pub mod tab;

pub mod prelude {
    pub use crate::document::Document;
    pub use crate::paragraph::Paragraph;
    pub use crate::table::{Table, TableColumn, TableRow, TableCell};
    pub use crate::text::Text;
    pub use crate::image::{Image, ImageAnchor};
    pub use crate::style::{Style, StyleFamily, StyleItem, StylePropertyGroup};
    pub use crate::stylesheet::Stylesheet;
    pub use crate::stylesheet_parser::StylesheetParser;
}