use std::io::Write;
use crate::fodt_xml_write::FlatOdtXmlWrite;

/// Base struct representing things that can have children
pub struct Container {
    children: Vec<Box<dyn FlatOdtXmlWrite>>,
}

impl Container {
    pub fn new() -> Self {
        Self { children: Vec::new() }
    }

    /// Add a new child to the container.
    pub fn add<T: FlatOdtXmlWrite + 'static>(&mut self, child: T) {
        self.children.push(Box::new(child));
    }

    /// Get the number of children for this container.
    pub fn len(&self) -> usize {
        self.children.len()
    }
}

impl FlatOdtXmlWrite for Container {
    fn write_flat_odt_xml(
        &self,
        writer: &mut xml::writer::EventWriter<&mut dyn Write>,
    ) -> xml::writer::Result<()> {
        for c in &self.children {
            c.write_flat_odt_xml(writer)?;
        }
        Ok(())
    }
}
