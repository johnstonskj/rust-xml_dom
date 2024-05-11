/*!
Provides a basic parser from text to DOM using the [quick-xml](https://crates.io/crates/quick-xml)
crate.

The parsing capability of quick-xml is limited in some ways, it does not support DTD handling other
than returning the entire DTD content as a string; therefore entities, notations, or entity
references are not constructed in the DOM. It does parse `Text`, `CDataSection`, and `Comment` nodes
but does limited entity processing or escaping.

# Example

```rust
use xml_dom::parser::read_xml;

let dom = read_xml(r#"<?xml version="1.0"?><xml/>"#);
assert!(dom.is_ok());
```

*/

use crate::level2::convert::as_document_mut;
use crate::level2::ext::{XmlDecl, XmlVersion};
use crate::level2::node_impl::Extension;
use crate::level2::*;
use crate::shared::error::Error as DOMError;
use quick_xml::events::{BytesCData, BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::reader::Reader;
use std::borrow::Borrow;
use std::io::BufRead;
use std::str::FromStr;

use thiserror::Error as E;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Errors constructing a DOM from text.
///
#[derive(Debug, E)]
pub enum Error {
    /// Usually a missing quote.
    #[error("invalid character")]
    InvalidCharacter,
    /// Everything else.
    #[error("malformed")]
    Malformed,
    /// Errors passed through from DOMError
    #[error("DOM error: {0}")]
    DOMError(#[from] DOMError),
    /// Errors passed through from quick-xml
    #[error("quick-xml error: {0}")]
    QuickXMLError(#[from] quick_xml::Error),
}

///
/// Result type for public function(s).
///
pub type Result<T> = std::result::Result<T, Error>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Parse the provided string into a DOM structure; if the result is OK, the result returned
/// can be safely assumed to be a `Document` node.
///
pub fn read_xml(xml: impl AsRef<str>) -> Result<RefNode> {
    inner_read(&mut Reader::from_str(xml.as_ref()))
}

///
/// Parse the provided string into a DOM structure; if the result is OK, the result returned
/// can be safely assumed to be a `Document` node.
///
pub fn read_reader<B: BufRead>(reader: B) -> Result<RefNode> {
    inner_read(&mut Reader::from_reader(reader))
}

impl<T> From<Error> for Result<T> {
    fn from(val: Error) -> Self {
        Err(val)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn inner_read<T: BufRead>(reader: &mut Reader<T>) -> Result<RefNode> {
    let _safe_to_ignore = reader.trim_text(true);

    let mut event_buffer: Vec<u8> = Vec::new();

    document(reader, &mut event_buffer)
}

///
/// This only needs to deal with the events that could start a document.
///
/// ```ebnf
/// document          ::= prolog element Misc* - Char* RestrictedChar Char*
///
/// prolog            ::= XMLDecl Misc* (doctypedecl Misc*)?
///
/// XMLDecl           ::= '<?xml' VersionInfo EncodingDecl? SDDecl? S?'?>'
///
/// doctypedecl       ::= '<!DOCTYPE' S Name (S ExternalID)? S? ('[' intSubset ']' S?)? '>'
///
/// Misc              ::= Comment | PI | S
///
/// Char              ::= [#x1-#xD7FF] | [#xE000-#xFFFD] | [#x10000-#x10FFFF]
///
/// RestrictedChar    ::= [#x1-#x8] | [#xB-#xC] | [#xE-#x1F] | [#x7F-#x84] | [#x86-#x9F]
///
/// S                 ::= (#x20 | #x9 | #xD | #xA)+
/// ```
///
fn document<T: BufRead>(reader: &mut Reader<T>, event_buffer: &mut Vec<u8>) -> Result<RefNode> {
    let mut document = get_implementation()
        .create_document(None, None, None)
        .unwrap();
    loop {
        match reader.read_event_into(event_buffer) {
            Ok(Event::Decl(ev)) => {
                let mut mut_document = document.borrow_mut();
                if let Extension::Document {
                    i_xml_declaration, ..
                } = &mut mut_document.i_extension
                {
                    if i_xml_declaration.is_some() {
                        error!("XML declaration must be first");
                        return Error::Malformed.into();
                    } else {
                        let (version, encoding, standalone) = make_decl(reader, ev)?;
                        *i_xml_declaration = Some(XmlDecl::new(
                            XmlVersion::from_str(&version).unwrap(),
                            encoding,
                            standalone,
                        ));
                    }
                }
            }
            Ok(Event::Start(ev)) => {
                let mut new_element = handle_start(reader, &mut document, None, ev)?;
                let _safe_to_ignore =
                    element(reader, event_buffer, &mut document, &mut new_element);
            }
            Ok(Event::Empty(ev)) => {
                let _safe_to_ignore = handle_start(reader, &mut document, None, ev)?;
            }
            Ok(Event::End(ev)) => {
                let _safe_to_ignore = handle_end(reader, &mut document, None, ev)?;
            }
            Ok(Event::Comment(ev)) => {
                let _safe_to_ignore = handle_comment(&mut document, None, ev)?;
            }
            Ok(Event::PI(ev)) => {
                let _safe_to_ignore = handle_pi(reader, &mut document, None, ev)?;
            }
            // Ok(Event::DocType(ev)) => {
            //     if prolog_pre_nodes
            //         .iter()
            //         .find(|n| n.kind == PreNodeKind::DocType)
            //         .is_some()
            //     {
            //         error!("only one document type allowed");
            //         return Error::Malformed.into();
            //     }
            //     prolog_pre_nodes.push(make_doc_type(reader, ev)?);
            // }
            Ok(Event::Eof) => return Ok(document),
            Ok(ev) => {
                error!("Unexpected parser event: {:?}", ev);
                return Error::Malformed.into();
            }
            Err(err) => {
                error!("Unexpected parser error: {:?}", err);
                return Error::from(err).into();
            }
        }
    }
}

///
/// Given a document that has been started, add to it.
///
/// ```ebnf
/// element           ::= EmptyElemTag | STag content ETag
/// STag              ::= '<' Name (S Attribute)* S? '>'
/// Attribute         ::= Name Eq AttValue
/// content           ::= CharData? ((element | Reference | CDSect | PI | Comment) CharData?)*
/// EmptyElemTag      ::= '<' Name (S Attribute)* S? '/>'
/// ```
///
fn element<T: BufRead>(
    reader: &mut Reader<T>,
    event_buffer: &mut Vec<u8>,
    document: &mut RefNode,
    parent_element: &mut RefNode,
) -> Result<RefNode> {
    loop {
        match reader.read_event_into(event_buffer) {
            Ok(Event::Start(ev)) => {
                let mut new_element = handle_start(reader, document, Some(parent_element), ev)?;
                let _safe_to_ignore = element(reader, event_buffer, document, &mut new_element)?;
            }
            Ok(Event::Empty(ev)) => {
                let _safe_to_ignore = handle_start(reader, document, Some(parent_element), ev)?;
            }
            Ok(Event::End(ev)) => {
                let _safe_to_ignore = handle_end(reader, document, Some(parent_element), ev)?;
                return Ok(parent_element.clone());
            }
            Ok(Event::Comment(ev)) => {
                let _safe_to_ignore = handle_comment(document, Some(parent_element), ev)?;
            }
            Ok(Event::PI(ev)) => {
                let _safe_to_ignore = handle_pi(reader, document, Some(parent_element), ev)?;
            }
            Ok(Event::Text(ev)) => {
                let _safe_to_ignore = handle_text(document, Some(parent_element), ev)?;
            }
            Ok(Event::CData(ev)) => {
                let _safe_to_ignore = handle_cdata(reader, document, Some(parent_element), ev)?;
            }
            Ok(ev) => {
                error!("Unexpected parser event: {:?}", ev);
                return Error::Malformed.into();
            }
            Err(err) => {
                error!("Unexpected parser error: {:?}", err);
                return Error::from(err).into();
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

fn handle_start<T: BufRead>(
    reader: &mut Reader<T>,
    document: &mut RefNode,
    parent_node: Option<&mut RefNode>,
    ev: BytesStart<'_>,
) -> Result<RefNode> {
    let mut element = {
        let mut_document = as_document_mut(document).unwrap();
        let name = ev.name().into_inner();
        let name = reader.decoder().decode(name)?;
        let new_node = mut_document.create_element(&name).unwrap();
        let mut actual_parent = match parent_node {
            None => document.clone(),
            Some(actual) => actual.clone(),
        };
        actual_parent.append_child(new_node)?
    };

    for attribute in ev.attributes() {
        let attribute = attribute.unwrap();
        let value = attribute.decode_and_unescape_value(reader)?;
        let name = reader.decoder().decode(attribute.key.into_inner())?;
        let attribute_node = document.create_attribute_with(name.as_ref(), &value)?;

        let _safe_to_ignore = element.set_attribute_node(attribute_node)?;
    }

    Ok(element)
}

fn handle_end<T: BufRead>(
    _reader: &mut Reader<T>,
    document: &mut RefNode,
    parent_node: Option<&mut RefNode>,
    _ev: BytesEnd<'_>,
) -> Result<RefNode> {
    Ok(match parent_node {
        None => document,
        Some(actual) => actual,
    }
    .clone())
}

fn handle_comment(
    document: &mut RefNode,
    parent_node: Option<&mut RefNode>,
    ev: BytesText<'_>,
) -> Result<RefNode> {
    let mut_document = as_document_mut(document).unwrap();
    let text = make_text(ev)?;
    let new_node = mut_document.create_comment(&text);
    let actual_parent = match parent_node {
        None => document,
        Some(actual) => actual,
    };
    actual_parent.append_child(new_node).map_err(|e| e.into())
}

fn handle_text(
    document: &mut RefNode,
    parent_node: Option<&mut RefNode>,
    ev: BytesText<'_>,
) -> Result<RefNode> {
    let mut_document = as_document_mut(document).unwrap();
    let text = make_text(ev)?;
    let new_node = mut_document.create_text_node(&text);
    let actual_parent = match parent_node {
        None => document,
        Some(actual) => actual,
    };
    actual_parent.append_child(new_node).map_err(|e| e.into())
}

fn handle_cdata<T: BufRead>(
    reader: &mut Reader<T>,
    document: &mut RefNode,
    parent_node: Option<&mut RefNode>,
    ev: BytesCData<'_>,
) -> Result<RefNode> {
    let mut_document = as_document_mut(document).unwrap();
    let text = make_cdata(reader, ev)?;
    let new_node = mut_document.create_cdata_section(text.as_ref()).unwrap();
    let actual_parent = match parent_node {
        None => document,
        Some(actual) => actual,
    };
    actual_parent.append_child(new_node).map_err(|e| e.into())
}

fn handle_pi<T: BufRead>(
    _reader: &mut Reader<T>,
    document: &mut RefNode,
    parent_node: Option<&mut RefNode>,
    ev: BytesText<'_>,
) -> Result<RefNode> {
    let mut_document = as_document_mut(document).unwrap();
    let text = ev.unescape()?;
    let parts = text.splitn(2, ' ').collect::<Vec<&str>>();
    let (target, data) = match parts.len() {
        1 => (parts[0].to_string(), None),
        2 => {
            let data = parts[1].trim();
            if data.is_empty() {
                (parts[0].to_string(), None)
            } else {
                (parts[0].to_string(), Some(data.to_string()))
            }
        }
        _ => return Error::Malformed.into(),
    };
    let new_node = match data {
        None => mut_document
            .create_processing_instruction(&target, None)
            .unwrap(),
        Some(s) => mut_document
            .create_processing_instruction(&target, Some(s.as_str()))
            .unwrap(),
    };
    let actual_parent = match parent_node {
        None => document,
        Some(actual) => actual,
    };
    actual_parent.append_child(new_node).map_err(|e| e.into())
}

// ------------------------------------------------------------------------------------------------

fn make_text(ev: BytesText<'_>) -> Result<String> {
    Ok(ev.unescape()?.to_string())
}

fn make_cdata<T: BufRead>(reader: &mut Reader<T>, ev: BytesCData<'_>) -> Result<String> {
    let cdata_bytes = ev.into_inner();
    let decoded_string = reader.decoder().decode(cdata_bytes.as_ref())?;
    Ok(decoded_string.to_string())
}

fn make_decl<T: BufRead>(
    reader: &mut Reader<T>,
    ev: BytesDecl<'_>,
) -> Result<(String, Option<String>, Option<bool>)> {
    let version = ev.version().unwrap();
    let version = version.borrow();
    let version = reader.decoder().decode(version).unwrap();
    let version = unquote(version.to_string())?;
    let encoding = if let Some(ev_value) = ev.encoding() {
        let encoding = ev_value.unwrap();
        let encoding = encoding.borrow();
        let encoding = reader.decoder().decode(encoding).unwrap();
        Some(encoding.to_string())
    } else {
        None
    };
    let standalone = if let Some(ev_value) = ev.standalone() {
        let standalone = ev_value.unwrap();
        let standalone = standalone.borrow();
        let standalone = reader.decoder().decode(standalone).unwrap();
        Some(standalone == "yes")
    } else {
        None
    };
    Ok((version, encoding, standalone))
}

#[allow(clippy::if_same_then_else)]
fn unquote(s: String) -> Result<String> {
    if s.starts_with('"') && s.ends_with('"') {
        Ok(s[1..s.len() - 1].to_string())
    } else if s.starts_with('\'') && s.ends_with('\'') {
        Ok(s[1..s.len() - 1].to_string())
    } else if s.starts_with('"') || s.starts_with('\'') {
        Error::InvalidCharacter.into()
    } else {
        Ok(s)
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn test_good_xml(xml: &str) {
        let dom = read_xml(xml);
        println!("------------------------------------------------------------------------------");
        println!("{:#?}", dom);
        assert!(dom.is_ok());
        println!("------------------------------------------------------------------------------");
        let dom = dom.unwrap();
        println!("{}", dom);
        println!("------------------------------------------------------------------------------");
    }

    #[test]
    fn test_shortest_document() {
        test_good_xml("<xml/>");
    }

    #[test]
    fn test_shortish_document() {
        test_good_xml("<?xml version=\"1.0\"?> <xml/>");
    }

    #[test]
    fn test_commented_document() {
        test_good_xml("<!-- start here --><xml/><!-- end here -->");
    }

    #[test]
    fn test_commented_element() {
        test_good_xml("<xml><!-- I'm inside --></xml>");
    }

    #[test]
    fn test_pi() {
        test_good_xml("<?xml-stylesheet type=\"text/xsl\" href=\"style.xsl\"?><xml/>");
    }

    #[test]
    fn test_nested_document() {
        test_good_xml("<xml><xslt/></xml>");
    }

    #[test]
    fn test_attributes() {
        test_good_xml("<xml id=\"11\"></xml>");
    }

    #[test]
    fn test_its_complicated() {
        test_good_xml(
            r###"
<?xml version="1.0"?>
<?xml-stylesheet type="text/xsl" href="style.xsl"?>
<root>
  This is text
  <list>
    <!-- just one for now -->
    <item id="1"/>
  </list>
  <![CDATA[
  This is OK, <markup/> is allowed here
  ]]>
</root>
"###,
        );
    }
}
