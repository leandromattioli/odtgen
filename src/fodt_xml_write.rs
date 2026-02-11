use std::io::Write;
use xml::writer::events::StartElementBuilder;

pub trait FlatOdtXmlWrite {
    /// Export to XML
    fn write_flat_odt_xml(
        &self,
        writer: &mut xml::writer::EventWriter<&mut dyn Write>,
    ) -> xml::writer::Result<()>;

    /// Optional XML attributes with direct mapping to struct optional attributes.
    fn optional_string_attributes(&self) -> Vec<(&'static str, Option<&str>)> {
        Vec::new()
    }

    /// Helper for adding optional attributes
    fn add_optional_string_attributes<'a>(&'a self, mut start: StartElementBuilder<'a>) -> StartElementBuilder<'a> {
        for (key, value) in self.optional_string_attributes() {
            if let Some(v) = value {
                start = start.attr(key, v);
            }
        }
        start
    }
}