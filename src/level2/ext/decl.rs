/*!
This module provides support types for the [`DocumentDecl`](trait.DocumentDecl.html) trait.
*/

use crate::shared::syntax::{
    XML_DECL_ENCODING, XML_DECL_END, XML_DECL_STANDALONE, XML_DECL_STANDALONE_NO,
    XML_DECL_STANDALONE_YES, XML_DECL_START, XML_DECL_VERSION, XML_DECL_VERSION_10,
    XML_DECL_VERSION_11,
};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
//  Public Types
// ------------------------------------------------------------------------------------------------

///
/// Captures the supported version of the XML specification itself, as used in `XmlDecl`.
///
#[derive(Clone, Debug, PartialEq)]
pub enum XmlVersion {
    /// Version 1.0 [`<https://www.w3.org/TR/xml>`]
    V10,
    /// Version 1.1 [`<https://www.w3.org/TR/xml11>`]
    V11,
}

///
/// The following productions are taken from XML 1.1 [§2.8 Prolog and Document Type
/// Declaration](https://www.w3.org/TR/xml11/#sec-prolog-dtd).
///
/// ```ebnf
/// prolog       ::=  XMLDecl Misc* (doctypedecl Misc*)?
/// XMLDecl      ::=  '<?xml' VersionInfo EncodingDecl? SDDecl? S? '?>'
/// VersionInfo  ::=  S 'version' Eq ("'" VersionNum "'" | '"' VersionNum '"')
/// Eq           ::=  S? '=' S?
/// VersionNum   ::=  '1.1'
/// Misc         ::=  Comment | PI | S
///
/// EncodingDecl ::=  S 'encoding' Eq ('"' EncName '"' | "'" EncName "'" )
/// EncName      ::=  [A-Za-z] ([A-Za-z0-9._] | '-')*
///
/// SDDecl       ::=  S 'standalone' Eq (("'" ('yes' | 'no') "'") | ('"' ('yes' | 'no') '"'))
/// ```
///
#[derive(Clone, Debug)]
pub struct XmlDecl {
    version: XmlVersion,
    encoding: Option<String>,
    standalone: Option<bool>,
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

pub(crate) const ENCODING_SEP_CHAR: char = '-';

fn is_encoding_start_char(c: char) -> bool {
    c.is_ascii_uppercase() || c.is_ascii_lowercase()
}

fn is_encoding_rest_char(c: char) -> bool {
    c.is_ascii_uppercase() || c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '_'
}

fn is_encoding_sub_string(s: &str) -> bool {
    s.chars().all(is_encoding_rest_char)
}

fn is_encoding(s: &str) -> bool {
    !s.is_empty()
        && s.starts_with(is_encoding_start_char)
        && s[1..].split(ENCODING_SEP_CHAR).all(is_encoding_sub_string)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for XmlVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                XmlVersion::V10 => XML_DECL_VERSION_10,
                XmlVersion::V11 => XML_DECL_VERSION_11,
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl FromStr for XmlVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == XML_DECL_VERSION_10 {
            Ok(XmlVersion::V10)
        } else if s == XML_DECL_VERSION_11 {
            Ok(XmlVersion::V11)
        } else {
            Err(())
        }
    }
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

impl Default for XmlDecl {
    fn default() -> Self {
        Self {
            version: XmlVersion::V10,
            encoding: None,
            standalone: None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for XmlDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{} {}=\"{}\"",
            XML_DECL_START, XML_DECL_VERSION, self.version
        )?;
        if let Some(encoding) = &self.encoding {
            write!(f, " {}=\"{}\"", XML_DECL_ENCODING, encoding)?;
        }
        if let Some(standalone) = &self.standalone {
            write!(
                f,
                " {}=\"{}\"",
                XML_DECL_STANDALONE,
                if *standalone {
                    XML_DECL_STANDALONE_YES
                } else {
                    XML_DECL_STANDALONE_NO
                }
            )?;
        }
        write!(f, "{}", XML_DECL_END)
    }
}

// ------------------------------------------------------------------------------------------------

impl XmlDecl {
    ///
    /// Construct a new `XmlDecl`.
    ///
    pub fn new(version: XmlVersion, encoding: Option<String>, standalone: Option<bool>) -> Self {
        if let Some(encoding) = &encoding {
            if !is_encoding(encoding) {
                panic!("XML encoding declaration value is not valid");
            }
        }
        Self {
            version,
            encoding,
            standalone,
        }
    }
    ///
    /// Return the `version` asserted in this declaration.
    ///
    pub fn version(&self) -> XmlVersion {
        self.version.clone()
    }
    ///
    /// Return the `encoding` value in this declaration.
    ///
    pub fn encoding(&self) -> Option<String> {
        self.encoding.clone()
    }
    ///
    /// Return the `standalone` value in this declaration.
    ///
    pub fn standalone(&self) -> Option<bool> {
        self.standalone
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_display() {
        assert_eq!(format!("{}", XmlVersion::V10), "1.0".to_string());
        assert_eq!(format!("{}", XmlVersion::V11), "1.1".to_string());
    }

    #[test]
    fn test_version_parse() {
        let parsed = XmlVersion::from_str("1.0");
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap(), XmlVersion::V10);

        let parsed = XmlVersion::from_str("1.1");
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap(), XmlVersion::V11);
    }

    #[test]
    fn test_version_parse_err() {
        let parsed = XmlVersion::from_str("1.2");
        assert!(parsed.is_err());
    }

    #[test]
    fn test_decl_display_default() {
        let decl = XmlDecl::default();
        assert_eq!(format!("{}", decl), "<?xml version=\"1.0\"?>".to_string());
    }

    #[test]
    fn test_decl_display() {
        let decl = XmlDecl::new(XmlVersion::V10, None, None);
        assert_eq!(format!("{}", decl), "<?xml version=\"1.0\"?>".to_string());

        let decl = XmlDecl::new(XmlVersion::V10, Some("UTF-8".to_string()), None);
        assert_eq!(
            format!("{}", decl),
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>".to_string()
        );

        let decl = XmlDecl::new(XmlVersion::V10, Some("UTF-8".to_string()), Some(true));
        assert_eq!(
            format!("{}", decl),
            "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>".to_string()
        );

        let decl = XmlDecl::new(XmlVersion::V10, Some("UTF-8".to_string()), Some(false));
        assert_eq!(
            format!("{}", decl),
            "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>".to_string()
        );

        let decl = XmlDecl::new(XmlVersion::V10, None, Some(false));
        assert_eq!(
            format!("{}", decl),
            "<?xml version=\"1.0\" standalone=\"no\"?>".to_string()
        );
    }

    #[test]
    fn test_encoding_strings() {
        let encodings = vec![
            "UTF-8",
            "UTF-16",
            "ISO-10646-UCS-2",
            "ISO-10646-UCS-4",
            "ISO-8859-1",
            "ISO-8859-2",
            "ISO-2022-JP",
            "Shift_JIS",
            "EUC-JP",
        ];
        for encoding in encodings {
            assert!(is_encoding(encoding));
        }
    }

    #[test]
    fn test_encoding_string_errs() {
        let bad_encodings = vec![
            "",
            "16-UTF",
            "ISO/10646-UCS-2",
            "ISO 10646-UCS-4",
            "ISO§8859-1",
        ];
        for encoding in bad_encodings {
            assert!(!is_encoding(encoding));
        }
    }
}
