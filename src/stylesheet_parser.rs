//! Parse stylesheets from YAML
//!
//! Sample YAML file:
//!
//! ```yaml
//! - name: TableHeader
//!   family: table-cell
//!   _scope: automatic
//!   table-cell-properties:
//!     fo:padding-top: 0.1cm
//!     fo:padding-bottom: 0.1cm
//!     fo:border-top: "1pt solid #000000"
//!     fo:border-bottom: "0.5pt solid #000000"
//!
//! - name: TableHeaderParagraph
//!   family: paragraph
//!   _scope: automatic
//!   text-properties:
//!     fo:font-family: "Liberation Sans"
//!     fo:font-weight: bold
//! ```

use serde_yaml_ng::Sequence;
use serde_yaml_ng::Value;
use strum::IntoEnumIterator;
use crate::stylesheet::Stylesheet;
use crate::style::{Style, StyleFamily, StyleItem, StylePropertyGroup, TabStop};

pub struct StylesheetParser {

}

impl StylesheetParser {
    pub fn parse_yaml(yaml: &str) -> Result<Stylesheet, String> {
        let styles_seq: Sequence = serde_yaml_ng::from_str(&yaml).map_err(|e| e.to_string())?;
        let mut stylesheet = Stylesheet::new();
        for style_dict in styles_seq {
            let style_dict = style_dict.as_mapping()
                .expect("All styles should be mappings!");
            //Base
            let name = Self::yaml_required_string(style_dict, "name");
            let family =  Self::yaml_required_string(style_dict, "family");
            let family = family.as_str().parse::<StyleFamily>().map_err(|e| e.to_string())?;
            let mut style = Style::new(name, family);
            //Normal style or Automatic style
            if let Some(scope) = Self::yaml_optional_string(style_dict, "_scope") {
                if scope == "automatic" {
                    style.automatic = true;
                }
            }
            //Parent style name, next style name, class...
            style.parent_style_name = Self::yaml_optional_string(style_dict, "parent-style-name");
            style.next_style_name = Self::yaml_optional_string(style_dict, "next-style-name");
            style.class = Self::yaml_optional_string(style_dict, "class");
            style.display_name = Self::yaml_optional_string(style_dict, "display-name");
            style.default_outline_level = Self::yaml_optional_u64(style_dict, "default-outline-level")
                .map(|v| v.try_into().ok())
                .flatten();
            //Properties
            Self::parse_properties(style_dict, &mut style)?;
            stylesheet.add_style(style);
        }
        Ok(stylesheet)
    }

    fn parse_properties(mapping: &serde_yaml_ng::Mapping, style: &mut Style) -> Result<(), String> {
        for group in StylePropertyGroup::iter() {
            let group_key = group.as_ref();
            //If the current property exists in the dictionary
            if let Some(property_val) = mapping.get(group_key) {
                if !property_val.is_mapping() {
                    return Err(format!("Property {} should be a dictionary!", group_key));
                }
                let property_dict = property_val.as_mapping().unwrap();
                let mut style_item = StyleItem::default();
                for (k, v) in property_dict {
                    let error = format!("Invalid key: {:?}!", k);
                    let k = k.as_str().unwrap_or(error.as_str());
                    match v {
                        Value::String(s) => { style_item.set(k, s)},
                        Value::Number(n) => { style_item.set(k, n.to_string().as_str())},
                        Value::Sequence(seq) => {
                            if k == "tab-stops" {
                                if group == StylePropertyGroup::ParagraphProperties {
                                    Self::parse_tab_stops(seq, &mut style_item)?;
                                }
                                else {
                                    return Err("Tab stops only allowed in paragraph properties!".to_string());
                                }
                            }
                        }
                        Value::Bool(b) => {
                            style_item.set(k, &b.to_string().as_str())
                        }
                        _ => return Err(format!("Invalid value for: '{}'", k)),
                    };
                }
                style.properties.insert(group, style_item);
            }
        }
        Ok(())
    }

    fn parse_tab_stops(tab_stops_seq: &serde_yaml_ng::Sequence, style_item: &mut StyleItem) -> Result<(), String> {
        for tab_stop in tab_stops_seq {
            match tab_stop {
                Value::Mapping(mapping) => {
                    let position = Self::yaml_required_string(mapping, "position");
                    let type_ = Self::yaml_optional_string(mapping, "type");
                    let tab_stop = TabStop {
                        position,
                        type_,
                    };
                    style_item.add_tab_stop(tab_stop);
                }
                _ => return Err("Tab stops must be mappings!".to_string()),
            }
        }
        Ok(())
    }

    fn yaml_optional_string(mapping: &serde_yaml_ng::Mapping, key: &str) -> Option<String> {
        mapping.get(key)
            .and_then(|v| v.as_str())
            .map(str::to_string)
    }

    fn yaml_optional_u64(mapping: &serde_yaml_ng::Mapping, key: &str) -> Option<u64> {
        mapping.get(key)
            .and_then(|v| v.as_u64())
    }

    fn yaml_required_string(mapping: &serde_yaml_ng::Mapping, key: &str) -> String {
        mapping.get(key)
            .and_then(|v| v.as_str())
            .expect("Required field invalid or not found!")
            .to_string()
    }

}