use crate::shared::syntax::*;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
//  Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum SpaceHandling {
    Default,
    Preserve,
}

// ------------------------------------------------------------------------------------------------
//  Public Functions
// ------------------------------------------------------------------------------------------------

///
/// From XML 1.1 §3.3.3 [Attribute-Value Normalization](https://www.w3.org/TR/xml11/#AVNormalize):
///
/// Before the value of an attribute is passed to the application or checked for validity, the XML
/// processor must normalize the attribute value by applying the algorithm below, or by using some
/// other method such that the value passed to the application is the same as that produced by the
/// algorithm.
///
/// 1. All line breaks must have been normalized on input to `#xA` as described in 2.11 End-of-Line
///    Handling, so the rest of this algorithm operates on text normalized in this way.
/// 2. Begin with a normalized value consisting of the empty string.
/// 3. For each character, entity reference, or character reference in the unnormalized attribute
///    value, beginning with the first and continuing to the last, do the following:
///    * For a character reference, append the referenced character to the normalized value.
///    * For an entity reference, recursively apply step 3 of this algorithm to the replacement text
///      of the entity.
///    * For a white space character (`#x20`, `#xD`, `#xA`, `#x9`), append a space character (`#x20`)
///      to the normalized value.
///    * For another character, append the character to the normalized value.
///
/// If the attribute type is not CDATA, then the XML processor must further process the normalized
/// attribute value by discarding any leading and trailing space (`#x20`) characters, and by
/// replacing sequences of space (`#x20`) characters by a single space (`#x20`) character.
///
/// Note that if the unnormalized attribute value contains a character reference to a white space
/// character other than space (`#x20`), the normalized value contains the referenced character
/// itself (`#xD`, `#xA` or `#x9`). This contrasts with the case where the unnormalized value
/// contains a white space character (not a reference), which is replaced with a space character
/// (`#x20`) in the normalized value and also contrasts with the case where the unnormalized value
/// contains an entity reference whose replacement text contains a white space character; being
/// recursively processed, the white space character is replaced with a space character (`#x20`) in
/// the normalized value.
///
/// All attributes for which no declaration has been read should be treated by a non-validating
/// processor as if declared CDATA.
///
/// It is an error if an attribute value contains a reference to an entity for which no declaration
/// has been read.
///
pub(crate) fn normalize_attribute_value(value: &String, is_cdata: bool) -> String {
    fn char_from_entity(entity: &str, hex: bool) -> String {
        let code_point = &entity[2..entity.len() - 1];
        let code_point = u32::from_str_radix(code_point, if hex { 16 } else { 10 }).unwrap();
        let character = char::try_from(code_point).unwrap();
        character.to_string()
    }
    let step_1 = normalize_end_of_lines(value);
    let step_3 = if step_1.is_empty() {
        step_1
    } else {
        let find = regex::Regex::new(
            r"(?P<char>&#\d+;)|(?P<char_hex>&#x[0-9a-fA-F]+;)|(?P<ws>[\u{20}\u{09}\u{0A}\u{0D}])",
        )
        .unwrap();
        let mut step_2 = String::new();
        let mut last_end = 0;
        for capture in find.captures_iter(&step_1) {
            let (start, end, replacement) = if let Some(a_match) = capture.name("char") {
                let replacement = char_from_entity(a_match.as_str(), false);
                (a_match.start(), a_match.end(), replacement)
            } else if let Some(a_match) = capture.name("char_hex") {
                let replacement = char_from_entity(a_match.as_str(), true);
                (a_match.start(), a_match.end(), replacement)
            } else if let Some(a_match) = capture.name("ws") {
                (a_match.start(), a_match.end(), "\u{20}".to_string())
            } else {
                panic!("unexpected result");
            };
            step_2.push_str(&value[last_end..start]);
            step_2.push_str(&replacement);
            last_end = end;
        }
        if last_end < value.len() {
            step_2.push_str(&value[last_end..]);
        }
        step_2
    };
    if is_cdata {
        step_3
    } else {
        step_3.trim_matches(' ').to_string()
    }
}

///
/// From XML 1.1 §2.11 [End-of-Line Handling](https://www.w3.org/TR/xml11/#sec-line-ends):
///
/// XML parsed entities are often stored in computer files which, for editing convenience, are
/// organized into lines. These lines are typically separated by some combination of the characters
/// CARRIAGE RETURN `(#xD`) and LINE FEED (`#xA`).
///
/// To simplify the tasks of applications, the XML processor must behave as if it normalized all line
/// breaks in external parsed entities (including the document entity) on input, before parsing, by
/// translating all of the following to a single `#xA` character:
///
/// * the two-character sequence `#xD` `#xA`
/// * the two-character sequence `#xD` `#x85`
/// * the single character `#x85`
/// * the single character `#x2028`
/// * any `#xD` character that is not immediately followed by `#xA` or `#x85`.
///
/// The characters `#x85` and `#x2028` cannot be reliably recognized and translated until an entity's
/// encoding declaration (if present) has been read. Therefore, it is a fatal error to use them
/// within the XML declaration or text declaration.
///
pub(crate) fn normalize_end_of_lines(value: &String) -> String {
    if value.is_empty() {
        value.to_string()
    } else {
        let line_ends = regex::Regex::new(r"\u{0D}[\u{0A}\u{85}]?|\u{85}|\u{2028}").unwrap();
        let result = line_ends.replace_all(value, "\u{0A}");
        result.to_string()
    }
    .to_string()
}

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

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_handling_default() {
        let sh = SpaceHandling::default();
        assert_eq!(sh, SpaceHandling::Default);
    }

    #[test]
    fn test_space_handling_display() {
        assert_eq!(
            format!("{}", SpaceHandling::Default),
            format!(
                "{}{}{}=\"{}\"",
                XML_NS_ATTRIBUTE, XML_NS_SEPARATOR, XML_NS_ATTR_SPACE, XML_NS_ATTR_SPACE_DEFAULT
            )
        );
        assert_eq!(
            format!("{}", SpaceHandling::Preserve),
            format!(
                "{}{}{}=\"{}\"",
                XML_NS_ATTRIBUTE, XML_NS_SEPARATOR, XML_NS_ATTR_SPACE, XML_NS_ATTR_SPACE_PRESERVE
            )
        );
    }

    #[test]
    fn test_space_handling_from_str() {
        assert_eq!(
            SpaceHandling::from_str(XML_NS_ATTR_SPACE_DEFAULT).unwrap(),
            SpaceHandling::Default
        );
        assert_eq!(
            SpaceHandling::from_str(XML_NS_ATTR_SPACE_PRESERVE).unwrap(),
            SpaceHandling::Preserve
        );
        assert!(SpaceHandling::from_str("").is_err());
        assert!(SpaceHandling::from_str("other").is_err());
    }

    #[test]
    fn test_end_of_line_handling() {
        let input = "one\u{0D}two\u{0D}\u{0A}\u{0A}three\u{0A}\u{0D}\u{85}four\u{85}five\u{2028}";
        let output = normalize_end_of_lines(&input.to_string());
        assert_eq!(
            output,
            "one\u{0A}two\u{0A}\u{0A}three\u{0A}\u{0A}four\u{0A}five\u{0A}".to_string()
        )
    }
}
