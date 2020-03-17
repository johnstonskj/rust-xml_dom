use crate::syntax::*;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
//  Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) enum SpaceHandling {
    Default,
    Preserve,
}

// ------------------------------------------------------------------------------------------------
//  Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Escape character data according to XML 1.1
/// [§2.4 Character Data and Markup](https://www.w3.org/TR/xml11/#dt-chardata). This is the
/// do-everything version, not attempting to separate the rules defined below by node type.
///
/// # Specification
///
/// Text consists of intermingled character data and markup. [Definition: **Markup** takes the form
/// of start-tags, end-tags, empty-element tags, entity references, character references, comments,
/// CDATA section delimiters, document type declarations, processing instructions, XML declarations,
/// text declarations, and any white space that is at the top level of the document entity (that is,
/// outside the document element and not inside any other markup).]
///
/// [Definition: All text that is not markup constitutes the **character data** of the document].
///
/// The ampersand character (&) and the left angle bracket (<) must not appear in their literal
/// form, except when used as markup delimiters, or within a comment, a processing instruction, or
/// a CDATA section. If they are needed elsewhere, they must be escaped using either numeric
/// character references or the strings "&amp;" and "&lt;" respectively. The right angle bracket
/// (>) may be represented using the string "&gt;", and must, for compatibility, be escaped using
/// either "&gt;" or a character reference when it appears in the string "]]>" in content, when that
/// string is not marking the end of a CDATA section.
///
/// In the content of elements, character data is any string of characters which does not contain
/// the start-delimiter of any markup or the CDATA-section-close delimiter, "]]>". In a CDATA
/// section, character data is any string of characters not including the CDATA-section-close
/// delimiter.
///
/// To allow attribute values to contain both single and double quotes, the apostrophe or
/// single-quote character (') may be represented as "&apos;", and the double-quote character (")
/// as "&quot;".
///
pub(crate) fn escape(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    for c in input.chars() {
        match c {
            XML_ESC_AMP_CHAR => result.push_str(&to_entity(XML_ESC_AMP_CHAR)),
            XML_ESC_APOS_CHAR => result.push_str(&to_entity(XML_ESC_APOS_CHAR)),
            XML_ESC_GT_CHAR => result.push_str(&to_entity(XML_ESC_GT_CHAR)),
            XML_ESC_LT_CHAR => result.push_str(&to_entity(XML_ESC_LT_CHAR)),
            XML_ESC_QUOT_CHAR => result.push_str(&to_entity(XML_ESC_QUOT_CHAR)),
            o => result.push(o),
        }
    }
    result
}

#[allow(dead_code)]
pub(crate) fn unescape(_input: &str) -> String {
    unimplemented!()
}

pub(crate) fn to_entity(c: char) -> String {
    format!(
        "{}{}{}",
        XML_NUMBERED_ENTITYREF_START, c as u16, XML_ENTITYREF_END
    )
}

#[allow(dead_code)]
pub(crate) fn to_entity_hex(c: char) -> String {
    format!(
        "{}{:X}{}",
        XML_HEX_NUMBERED_ENTITYREF_START, c as u16, XML_ENTITYREF_END
    )
}

///
/// From [XML 1.0 §2.2](https://www.w3.org/TR/REC-xml/#charsets)
///
/// Definition: A parsed entity contains **text**, a sequence of characters, which may represent
/// markup or character data. Definition: A **character** is an atomic unit of text as specified by
/// ISO/IEC 10646:2000. Legal characters are tab, carriage return, line feed, and the legal
/// characters of Unicode and ISO/IEC 10646. The versions of these standards cited in A.1 Normative
/// References were current at the time this document was prepared. New characters may be added to
/// these standards by amendments or new editions. Consequently, XML processors must accept any
/// character in the range specified for `Char`.
///
/// ```ebnf
/// Char  ::= #x9 | #xA | #xD | [#x20-#xD7FF] | [#xE000-#xFFFD] | [#x10000-#x10FFFF]
///       /* any Unicode character, excluding the surrogate blocks, FFFE, and FFFF. */
/// ```
///
/// Document authors are encouraged to avoid "compatibility characters", as defined in section 2.3
/// of Unicode. The characters defined in the following ranges are also discouraged. They are either
/// control characters or permanently undefined Unicode characters:
///
/// ```text
/// [#x7F-#x84], [#x86-#x9F], [#xFDD0-#xFDEF],
/// [#x1FFFE-#x1FFFF], [#x2FFFE-#x2FFFF], [#x3FFFE-#x3FFFF],
/// [#x4FFFE-#x4FFFF], [#x5FFFE-#x5FFFF], [#x6FFFE-#x6FFFF],
/// [#x7FFFE-#x7FFFF], [#x8FFFE-#x8FFFF], [#x9FFFE-#x9FFFF],
/// [#xAFFFE-#xAFFFF], [#xBFFFE-#xBFFFF], [#xCFFFE-#xCFFFF],
/// [#xDFFFE-#xDFFFF], [#xEFFFE-#xEFFFF], [#xFFFFE-#xFFFFF],
/// [#x10FFFE-#x10FFFF].
/// ```
///
#[allow(dead_code)]
pub(crate) fn is_xml_10_char(c: char) -> bool {
    c == '\u{0009}'
        || c == '\u{000A}'
        || c == '\u{000D}'
        || (c >= '\u{0020}' && c <= '\u{D7FF}')
        || (c >= '\u{E000}' && c <= '\u{FFFD}')
        || (c >= '\u{10000}' && c <= '\u{10FFF}')
}

#[allow(dead_code)]
pub(crate) fn is_xml_10_restricted_char(c: char) -> bool {
    c == XML_ESC_AMP_CHAR
        || c == XML_ESC_APOS_CHAR
        || c == XML_ESC_GT_CHAR
        || c == XML_ESC_LT_CHAR
        || c == XML_ESC_QUOT_CHAR
}

///
/// From [XML 11 §2.2](https://www.w3.org/TR/xml11/#charsets)
///
/// ```ebnf
/// Char            ::=   [#x1-#xD7FF] | [#xE000-#xFFFD] | [#x10000-#x10FFFF]
///                 /* any Unicode character, excluding the surrogate blocks, FFFE, and FFFF. */
/// ```
///
#[allow(dead_code)]
pub(crate) fn is_xml_11_char(c: char) -> bool {
    //
    // below ranges are always valid for XML 1.1 documents
    // from https://en.wikipedia.org/wiki/XML#Valid_characters
    //
    (c >= '\u{0001}' && c <= '\u{D7FF}')
        || (c >= '\u{E000}' && c <= '\u{FFFD}')
        || (c >= '\u{10000}' && c <= '\u{10FFF}')
}

///
/// From [XML 11 §2.2](https://www.w3.org/TR/xml11/#charsets)
///
/// ```ebnf
/// RestrictedChar  ::=  #x1-#x8] | [#xB-#xC] | [#xE-#x1F] | [#x7F-#x84] | [#x86-#x9F]
/// ```
///
#[allow(dead_code)]
pub(crate) fn is_xml_11_restricted_char(c: char) -> bool {
    //
    // below ranges are always valid for XML 1.1 documents
    // from https://en.wikipedia.org/wiki/XML#Valid_characters
    //
    (c >= '\u{01}' && c <= '\u{08}')
        || (c >= '\u{0B}' && c <= '\u{0C}')
        || (c >= '\u{0E}' && c <= '\u{1F}')
        || (c >= '\u{7F}' && c <= '\u{84}')
        || (c >= '\u{86}' && c <= '\u{9F}')
}

///
/// S (white space) consists of one or more space (#x20) characters, carriage returns, line feeds,
/// or tabs.
///
/// ```ebnf
/// S ::= (#x20 | #x9 | #xD | #xA)+
/// ```
///
/// The presence of #xD in the above production is maintained purely for backward compatibility
/// with the First Edition. As explained in 2.11 End-of-Line Handling, all #xD characters literally
/// present in an XML document are either removed or replaced by #xA characters before any other
/// processing is done. The only way to get a #xD character to match this production is to use a
/// character reference in an entity value literal.
///
#[allow(dead_code)]
pub(crate) fn is_xml_space(c: char) -> bool {
    c == '\u{09}' || c == '\u{0A}' || c == '\u{0D}' || c == '\u{20}'
}

///
/// ```ebnf
/// NameStartChar   ::=  ":" | [A-Z] | "_" | [a-z] | [#xC0-#xD6] | [#xD8-#xF6] | [#xF8-#x2FF] |
///                      [#x370-#x37D] | [#x37F-#x1FFF] | [#x200C-#x200D] | [#x2070-#x218F] |
///                      [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF] | [#xFDF0-#xFFFD] |
///                      [#x10000-#xEFFFF]
/// ```
///
#[allow(dead_code)]
pub(crate) fn is_xml_name_start_char(c: char) -> bool {
    c == ':'
        || (c >= 'A' && c <= 'Z')
        || c == '_'
        || (c >= 'a' && c <= 'z')
        || (c >= '\u{C0}' && c <= '\u{D6}')
        || (c >= '\u{D8}' && c <= '\u{F6}')
        || (c >= '\u{0F8}' && c <= '\u{2FF}')
        || (c >= '\u{370}' && c <= '\u{37D}')
        || (c >= '\u{037F}' && c <= '\u{1FFF}')
        || (c >= '\u{200C}' && c <= '\u{200D}')
        || (c >= '\u{2070}' && c <= '\u{218F}')
        || (c >= '\u{2C00}' && c <= '\u{2FEF}')
        || (c >= '\u{3001}' && c <= '\u{D7FF}')
        || (c >= '\u{F900}' && c <= '\u{FDCF}')
        || (c >= '\u{FDF0}' && c <= '\u{FFFD}')
        || (c >= '\u{10000}' && c <= '\u{EFFFF}')
}

///
/// ```ebnf
/// NameChar   ::=  NameStartChar | "-" | "." | [0-9] | #xB7 |
///                 [#x0300-#x036F] | [#x203F-#x2040]
/// ```
///
pub(crate) fn is_xml_name_char(c: char) -> bool {
    is_xml_name_start_char(c)
        || c == '-'
        || c == '.'
        || (c >= '0' && c <= '9')
        || c == '\u{B7}'
        || (c >= '\u{0300}' && c <= '\u{036F}')
        || (c >= '\u{203F}' && c <= '\u{2040}')
}

///
/// ```ebnf
/// Name   ::=  NameStartChar (NameChar)*
/// ```
///
pub(crate) fn is_xml_name(s: &str) -> bool {
    !s.is_empty() && s.starts_with(is_xml_name_start_char) && s[1..].chars().all(is_xml_name_char)
}

///
/// ```ebnf
/// Names   ::=  Name (#x20 Name)*
/// ```
///
#[allow(dead_code)]
pub(crate) fn is_xml_names(s: &str) -> bool {
    !s.is_empty() && s.split(' ').all(is_xml_name)
}

///
/// ```ebnf
/// Nmtoken   ::=  (NameChar)+
/// ```
///
#[allow(dead_code)]
pub(crate) fn is_xml_nmtoken(s: &str) -> bool {
    !s.is_empty() && s.chars().all(is_xml_name_char)
}

///
/// ```ebnf
/// Nmtokens   ::=  Nmtoken (#x20 Nmtoken)*
/// ```
///
#[allow(dead_code)]
pub(crate) fn is_xml_nmtokens(s: &str) -> bool {
    !s.is_empty() && s.split(' ').all(is_xml_nmtoken)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for SpaceHandling {
    fn default() -> Self {
        SpaceHandling::Default
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SpaceHandling {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}{}{}=\"{}\"",
            XML_NS_ATTRIBUTE,
            XML_NS_SEPARATOR,
            XML_NS_ATTR_SPACE,
            match self {
                SpaceHandling::Default => XML_NS_ATTR_SPACE_DEFAULT,
                SpaceHandling::Preserve => XML_NS_ATTR_SPACE_PRESERVE,
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl FromStr for SpaceHandling {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == XML_NS_ATTR_SPACE_DEFAULT {
            Ok(SpaceHandling::Default)
        } else if s == XML_NS_ATTR_SPACE_PRESERVE {
            Ok(SpaceHandling::Preserve)
        } else {
            Err(())
        }
    }
}
