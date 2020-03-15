#![allow(dead_code)]

use std::fmt::{Binary, Display, Formatter, Result};
use std::ops::{BitAnd, BitOr};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This type encapsulates a set of options that a client can set that affect the processing of
/// nodes as they are added/removed from the DOM.
///
/// This type has a set of methods that turn on options, i.e. `set_assume_ids`,  and retrieve the
/// state of an option, i.e. `has_assume_ids`.
///
/// # Example
///
/// ```rust
/// use xml_dom::*;
/// use xml_dom::convert::*;
///
/// let mut options = ProcessingOptions::new();
/// options.set_assume_ids();
///
/// let implementation = get_implementation();
/// let mut document_node = implementation
///     .create_document_with_options(
///         "http://www.w3.org/1999/xhtml",
///         "html",
///         None,
///         options)
///     .unwrap();
/// ```
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProcessingOptions(u8);

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
#[derive(Clone, Debug)]
#[repr(u8)]
enum ProcessingOptionFlags {
    AssumeIDs = 0b00000001,
    ParseEntities = 0b00000010,
    AddNamespaces = 0b00000100,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for ProcessingOptions {
    fn default() -> Self {
        Self(0)
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ProcessingOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "ProcessingOptions {{")?;

        let mut option_strings: Vec<&str> = Vec::new();
        if self.has_assume_ids() {
            option_strings.push("AssumeIDs");
        }
        if self.has_parse_entities() {
            option_strings.push("ParseEntities");
        }
        if self.has_add_namespaces() {
            option_strings.push("AddNamespaces");
        }
        write!(f, "{}", option_strings.join(", "))?;

        write!(f, "}}")
    }
}

// ------------------------------------------------------------------------------------------------

impl Binary for ProcessingOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if f.alternate() {
            write!(f, "{:#010b}", self.0)
        } else {
            write!(f, "{:08b}", self.0)
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl BitAnd for ProcessingOptions {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

// ------------------------------------------------------------------------------------------------

impl BitOr for ProcessingOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

// ------------------------------------------------------------------------------------------------

impl ProcessingOptions {
    ///
    /// Construct a new `ProcessingOptions` instance with all options off.
    ///
    pub fn new() -> Self {
        Default::default()
    }
    ///
    /// Returns true if all options are `false`.
    ///
    pub fn has_none(&self) -> bool {
        self.0 == 0
    }
    ///
    /// Returns `true` if the document will automatically assume certain attributes will be treated
    /// as XML `id` values, else `false`.
    ///
    pub fn has_assume_ids(&self) -> bool {
        self.0 & (ProcessingOptionFlags::AssumeIDs as u8) != 0
    }
    ///
    /// Returns `true` if the document will parse entities inside text nodes and create
    /// `EntityReference` nodes, else `false`.
    ///
    pub fn has_parse_entities(&self) -> bool {
        self.0 & (ProcessingOptionFlags::ParseEntities as u8) != 0
    }
    ///
    /// Returns `true` if the document will automatically add namespace attributes to elements if
    /// qualified names are added that do not have current mappings., else `false`.
    ///
    pub fn has_add_namespaces(&self) -> bool {
        self.0 & (ProcessingOptionFlags::AddNamespaces as u8) != 0
    }
    ///
    /// TBD.
    ///
    /// **Note:** if an attribute with the qualified name `xml:id`, and the namespace is set to the
    /// XML namespace `http://www.w3.org/XML/1998/namespace` then the value is known to be an ID.
    ///
    /// See xml:id Version 1.0, §4 [Processing xml:id Attributes](https://www.w3.org/TR/xml-id/#processing)
    /// for more details.
    ///
    pub fn set_assume_ids(&mut self) {
        self.0 = self.0 | (ProcessingOptionFlags::AssumeIDs as u8)
    }
    ///
    /// TBD
    ///
    pub fn set_parse_entities(&mut self) {
        self.0 = self.0 | (ProcessingOptionFlags::ParseEntities as u8)
    }
    ///
    /// TBD
    ///
    pub fn set_add_namespaces(&mut self) {
        self.0 = self.0 | (ProcessingOptionFlags::AddNamespaces as u8)
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none() {
        let options = ProcessingOptions::default();

        assert!(options.has_none());
        assert!(!options.has_assume_ids());
        assert!(!options.has_parse_entities());
        assert!(!options.has_add_namespaces());

        assert_eq!(format!("{}", options), r"ProcessingOptions {}".to_string());
        assert_eq!(format!("{:b}", options), r"00000000".to_string());
        assert_eq!(format!("{:#b}", options), r"0b00000000".to_string());

        let new_options = ProcessingOptions::new();
        assert_eq!(options, new_options);
    }
}
