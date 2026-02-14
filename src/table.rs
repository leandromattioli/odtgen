use std::io::Write;
use xml::EventWriter;
use crate::container::Container;
use crate::fodt_xml_write::FlatOdtXmlWrite;
use xml::writer::{XmlEvent as XmlWriterEvent};

pub struct Table {
    pub name: String,
    pub style_name: Option<String>,

    pub columns: Vec<TableColumn>,
    pub rows: Vec<TableRow>,
}

impl Table {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            style_name: None,
            columns: Vec::new(),
            rows: Vec::new()
        }
    }

    pub fn add_column(&mut self, column: TableColumn) {
        self.columns.push(column);
    }

    pub fn add_row(&mut self, row: TableRow) {
        self.rows.push(row);
    }
}

impl FlatOdtXmlWrite for Table {
    fn write_flat_odt_xml(&self, writer: &mut EventWriter<&mut dyn Write>) -> xml::writer::Result<()> {
        let mut start = XmlWriterEvent::start_element("table:table")
            .attr("table:name", &self.name);
        start = self.add_optional_string_attributes(start);
        writer.write(start)?;
        //Column specs
        for column in &self.columns {
            column.write_flat_odt_xml(writer)?;
        }
        //Rows and data
        for row in &self.rows {
            row.write_flat_odt_xml(writer)?;
        }
        //Finish
        writer.write(XmlWriterEvent::end_element())
    }

    fn optional_string_attributes(&self) -> Vec<(&'static str, Option<&str>)> {
        vec![
            ("table:style-name", self.style_name.as_deref()),
        ]
    }
}

// ===============================================================================================
// Columns
// ===============================================================================================

pub struct TableColumn {
    pub style_name: Option<String>,
    pub default_cell_style_name: Option<String>,
    pub number_columns_repeated: Option<u32>,
}

impl TableColumn {
    pub fn new() -> Self {
        Self {
            style_name: None,
            default_cell_style_name: None,
            number_columns_repeated: None
        }
    }
}

impl FlatOdtXmlWrite for TableColumn {
    fn write_flat_odt_xml(&self, writer: &mut EventWriter<&mut dyn Write>) -> xml::writer::Result<()> {
        let attr : String;
        let mut start = XmlWriterEvent::start_element("table:table-column");
        start = self.add_optional_string_attributes(start);
        if let Some(ref repeat) = self.number_columns_repeated {
            attr = repeat.to_string();
            start = start.attr("table:number-columns-repeated", attr.as_str());
        }
        writer.write(start)?;
        writer.write(XmlWriterEvent::end_element())
    }

    //noinspection DuplicatedCode
    fn optional_string_attributes(&self) -> Vec<(&'static str, Option<&str>)> {
        vec![
            ("table:style-name", self.style_name.as_deref()),
            ("table:default-cell-style-name", self.default_cell_style_name.as_deref()),
        ]
    }
}

// ===============================================================================================
// Rows
// ===============================================================================================

pub struct TableRow {
    pub cells: Vec<TableCell>,
    pub default_cell_style_name: Option<String>,
    pub number_rows_repeated: Option<u32>,
    pub style_name: Option<String>,
}

impl TableRow {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            default_cell_style_name: None,
            style_name: None,
            number_rows_repeated: None,
        }
    }

    pub fn add_cell(&mut self, cell: TableCell) {
        self.cells.push(cell);
    }
}

impl FlatOdtXmlWrite for TableRow {
    fn write_flat_odt_xml(&self, writer: &mut EventWriter<&mut dyn Write>) -> xml::writer::Result<()> {
        let attr : String;
        let mut start = XmlWriterEvent::start_element("table:table-row");
        start = self.add_optional_string_attributes(start);
        if let Some(ref repeat) = self.number_rows_repeated {
            attr = repeat.to_string();
            start = start.attr("table:number-rows-repeated", attr.as_str());
        }
        writer.write(start)?;
        for cell in &self.cells {
            cell.write_flat_odt_xml(writer)?;
        }
        writer.write(XmlWriterEvent::end_element())
    }

    //noinspection DuplicatedCode
    fn optional_string_attributes(&self) -> Vec<(&'static str, Option<&str>)> {
        vec![
            ("table:style-name", self.style_name.as_deref()),
            ("table:default-cell-style-name", self.default_cell_style_name.as_deref()),
        ]
    }
}

// ===============================================================================================
// Cells
// ===============================================================================================

pub struct TableCell {
    pub style_name: Option<String>,
    pub value_type: Option<String>,
    pub content: Container,
}

impl TableCell {
    pub fn new() -> Self {
        Self {
            style_name: None,
            value_type: None,
            content: Container::new()
        }
    }
}

impl FlatOdtXmlWrite for TableCell {
    fn write_flat_odt_xml(&self, writer: &mut EventWriter<&mut dyn Write>) -> xml::writer::Result<()> {
        let mut start = XmlWriterEvent::start_element("table:table-cell");
        start = self.add_optional_string_attributes(start);
        writer.write(start)?;
        self.content.write_flat_odt_xml(writer)?;
        writer.write(XmlWriterEvent::end_element())
    }

    fn optional_string_attributes(&self) -> Vec<(&'static str, Option<&str>)> {
        vec![
            ("table:style-name", self.style_name.as_deref()),
            ("office:value-type", self.value_type.as_deref()),
        ]
    }
}
