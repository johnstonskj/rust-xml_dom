#![allow(dead_code)]

// ------------------------------------------------------------------------------------------------
// Pure Syntactic Tokens
// ------------------------------------------------------------------------------------------------

pub(crate) const XML_PI_START: &str = "<?";
pub(crate) const XML_PI_END: &str = ">";

pub(crate) const XML_COMMENT_START: &str = "<!--";
pub(crate) const XML_COMMENT_END: &str = "-->";

pub(crate) const XML_CDATA_START: &str = "<![CDATA[";
pub(crate) const XML_CDATA_END: &str = "]]>";

pub(crate) const XML_DOCTYPE_START: &str = "<!DOCTYPE";
pub(crate) const XML_DOCTYPE_END: &str = ">";
pub(crate) const XML_DOCTYPE_ENTITY_START: &str = "[";
pub(crate) const XML_DOCTYPE_ENTITY_END: &str = "]";
pub(crate) const XML_DOCTYPE_PUBLIC: &str = "PUBLIC";
pub(crate) const XML_DOCTYPE_SYSTEM: &str = "SYSTEM";

pub(crate) const XML_ELEMENT_START_START: &str = "<";
pub(crate) const XML_ELEMENT_START_END: &str = ">";
pub(crate) const XML_ELEMENT_END_START: &str = "</";
pub(crate) const XML_ELEMENT_END_END: &str = ">";

pub(crate) const XML_EMPTY: &str = "";

// ------------------------------------------------------------------------------------------------
// Namespace Support
// ------------------------------------------------------------------------------------------------

pub(crate) const XML_NS_ATTRIBUTE: &str = "xml";
pub(crate) const XML_NS_URI: &str = "http://www.w3.org/XML/1998/namespace";

pub(crate) const XMLNS_NS_ATTRIBUTE: &str = "xmlns";
pub(crate) const XMLNS_NS_URI: &str = "http://www.w3.org/2000/xmlns/";

pub(crate) const XML_NS_SEPARATOR: &str = ":";

// ------------------------------------------------------------------------------------------------
// DOM Node Names
// ------------------------------------------------------------------------------------------------

pub(crate) const XML_NAME_CDATA: &str = "#cdata-section";
pub(crate) const XML_NAME_COMMENT: &str = "#comment";
pub(crate) const XML_NAME_DOCUMENT: &str = "#document";
pub(crate) const XML_NAME_TEXT: &str = "#text";

// ------------------------------------------------------------------------------------------------
// DOM Features
// ------------------------------------------------------------------------------------------------

pub(crate) const XML_FEATURE_CORE: &str = "Core"; // DOM Level-2 "Fundamental Interfaces"
pub(crate) const XML_FEATURE_XML: &str = "XML"; // DOM Level-2 "Extended Interfaces"

pub(crate) const XML_FEATURE_V1: &str = "1.0";
pub(crate) const XML_FEATURE_V2: &str = "2.0";

// ------------------------------------------------------------------------------------------------
// Character Entities
// ------------------------------------------------------------------------------------------------

pub(crate) const XML_ESC_AMP_CHAR: char = '&';
pub(crate) const XML_ESC_AMP_ENT: &str = "&amp;";
pub(crate) const XML_ESC_AMP_NUM: &str = "&#38;";

pub(crate) const XML_ESC_APOS_CHAR: char = '\'';
pub(crate) const XML_ESC_APOS_ENT: &str = "&apos;";
pub(crate) const XML_ESC_APOS_NUM: &str = "&#39;";

pub(crate) const XML_ESC_GT_CHAR: char = '>';
pub(crate) const XML_ESC_GT_ENT: &str = "&gt;";
pub(crate) const XML_ESC_GT_NUM: &str = "&#62;";

pub(crate) const XML_ESC_LT_CHAR: char = '<';
pub(crate) const XML_ESC_LT_ENT: &str = "&lt;";
pub(crate) const XML_ESC_LT_NUM: &str = "&#60;";

pub(crate) const XML_ESC_QUOT_CHAR: char = '"';
pub(crate) const XML_ESC_QUOT_ENT: &str = "&amp;";
pub(crate) const XML_ESC_QUOT_NUM: &str = "&#34;";
