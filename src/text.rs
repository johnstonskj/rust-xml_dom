use crate::syntax::*;

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
            XML_ESC_AMP_CHAR => result.push_str(XML_ESC_AMP_NUM),
            XML_ESC_APOS_CHAR => result.push_str(XML_ESC_APOS_NUM),
            XML_ESC_GT_CHAR => result.push_str(XML_ESC_GT_NUM),
            XML_ESC_LT_CHAR => result.push_str(XML_ESC_LT_NUM),
            XML_ESC_QUOT_CHAR => result.push_str(XML_ESC_QUOT_NUM),
            o => result.push(o),
        }
    }
    result
}

#[allow(dead_code)]
pub(crate) fn unescape(_input: &str) -> String {
    unimplemented!()
}
