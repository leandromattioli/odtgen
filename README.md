# odtgen

Generate **Flat ODT (.fodt)** documents programatically.

## Features

- Styles and stylesheets parsed from YAML files
- Paragraphs (with styles)
- Simple tables
- Embedded images (as Base64)
- Pure Rust
- No LibreOffice dependency
- Flat ODT output (human-readable XML)

## Status

Early-stage project. APIs may change.

## Examples

### Hello World
```rust
use odtgen::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::create("output.fodt").expect("Failed to create file!");
    let mut doc = Document::new(); //document created with a default stylesheet
    doc.header.add(Paragraph::from_text_and_style("Sample Header", "Heading1"));
    doc.body.add(Paragraph::from_text_and_style("Hello World!", "Standard"));
    doc.generate_fodt(&mut file).expect("Failed to generate file!");
}
```
### Image
```rust
use odtgen::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::create("image.fodt").expect("Failed to create file!");
    let mut doc = Document::new(); //document created with a default stylesheet
    let mut par = Paragraph::new();
    let path = "/some/path/to/image.png".into(); //change this path to point to an actual image!
    let image = Image::new(&path, 4.0, 3.5, ImageAnchor::AsChar);
    par.content.add(image);
    doc.body.add(par);
    doc.generate_fodt(&mut file).expect("Failed to generate file!");
}
```