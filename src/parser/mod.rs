/*!
One-line description.

More detailed description, with

# Example

*/

use crate::level2::convert::{as_document, as_document_decl_mut};
use crate::level2::ctraits::*;
use crate::{get_implementation, RefNode, XmlDecl, XmlVersion};
use quick_xml::events::{BytesDecl, BytesText, Event};
use quick_xml::Reader;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum Error {
    HierarchyRequest,
    InvalidCharacter,
    NotSupported,
    IO,
    Encoding,
    Malformed,
}

pub type Result<T> = std::result::Result<T, Error>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn read_xml(xml: &str) -> Result<RefNode> {
    let mut reader = Reader::from_str(xml);
    let _safe_to_ignore = reader.trim_text(true);

    let mut event_buffer: Vec<u8> = Vec::new();

    document(&mut reader, &mut event_buffer)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Error {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl std::error::Error for Error {}

impl<T> Into<Result<T>> for Error {
    fn into(self) -> Result<T> {
        Err(self)
    }
}

impl From<quick_xml::Error> for Error {
    fn from(err: quick_xml::Error) -> Self {
        error!("quick_xml::Error: {:?}", err);
        match err {
            quick_xml::Error::Io(_) => Error::IO,
            quick_xml::Error::Utf8(_) => Error::Encoding,
            quick_xml::Error::UnexpectedEof(_) => Error::Malformed,
            quick_xml::Error::EndEventMismatch { .. } => Error::Malformed,
            quick_xml::Error::UnexpectedToken(_) => Error::Malformed,
            quick_xml::Error::UnexpectedBang => Error::Malformed,
            quick_xml::Error::TextNotFound => Error::Malformed,
            quick_xml::Error::XmlDeclWithoutVersion(_) => Error::Malformed,
            quick_xml::Error::NameWithQuote(_) => Error::Malformed,
            quick_xml::Error::NoEqAfterName(_) => Error::Malformed,
            quick_xml::Error::UnquotedValue(_) => Error::Malformed,
            quick_xml::Error::DuplicatedAttribute(_, _) => Error::Malformed,
            quick_xml::Error::EscapeError(_) => Error::InvalidCharacter,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
enum PreNodeKind {
    XmlDecl,
    ProcessingInstruction,
    Comment,
    CData,
    Text,
    DocType,
    Element,
}

#[derive(Clone, Debug)]
struct PreNode {
    kind: PreNodeKind,
    name: String,
    value: String,
    attributes: HashMap<String, String>,
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

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
fn document(reader: &mut Reader<&[u8]>, event_buffer: &mut Vec<u8>) -> Result<RefNode> {
    let (prolog_nodes, element_pre_node) = prolog(reader, event_buffer)?;

    let document_node = make_document_node(prolog_nodes, element_pre_node)?;

    let document_element = {
        let ref_document = as_document(&document_node).unwrap();
        ref_document.document_element().unwrap()
    };
    let _safe_to_ignore = element(reader, event_buffer, &mut document_element.clone())?;

    misc(reader, event_buffer, &mut document_node.clone())
}

fn prolog(
    reader: &mut Reader<&[u8]>,
    event_buffer: &mut Vec<u8>,
) -> Result<(Vec<PreNode>, PreNode)> {
    let mut prolog_pre_nodes: Vec<PreNode> = Vec::default();
    loop {
        match reader.read_event(event_buffer) {
            Ok(Event::Decl(ev)) => {
                if !prolog_pre_nodes.is_empty() {
                    error!("XML declaration must be first");
                    return Error::Malformed.into();
                }
                prolog_pre_nodes.push(make_decl(reader, ev)?);
            }
            Ok(Event::Comment(ev)) => prolog_pre_nodes.push(make_comment(reader, ev)?),
            Ok(Event::PI(ev)) => prolog_pre_nodes.push(make_pi(reader, ev)?),
            Ok(Event::DocType(ev)) => {
                if prolog_pre_nodes
                    .iter()
                    .find(|n| n.kind == PreNodeKind::DocType)
                    .is_some()
                {
                    error!("only one document type allowed");
                    return Error::Malformed.into();
                }
                prolog_pre_nodes.push(make_doc_type(reader, ev)?);
            }
            Ok(Event::Start(ev)) | Ok(Event::Empty(ev)) => {
                return Ok((
                    prolog_pre_nodes,
                    PreNode {
                        kind: PreNodeKind::Element,
                        name: ev.unescape_and_decode(reader)?,
                        value: String::new(),
                        attributes: Default::default(),
                    },
                ));
            }
            Ok(_) => return Error::Malformed.into(),
            Err(err) => return Error::from(err).into(),
        }
    }
}

fn element(
    reader: &mut Reader<&[u8]>,
    event_buffer: &mut Vec<u8>,
    ref_self: &mut RefNode,
) -> Result<RefNode> {
    loop {
        let _child_node = match reader.read_event(event_buffer) {
            Ok(Event::Start(_ev)) => {}
            Ok(Event::End(_ev)) => return Ok(ref_self.clone()),
            Ok(Event::Empty(_ev)) => {}
            Ok(Event::Text(ev)) => {
                let document = ref_self.owner_document().unwrap();
                let pre_node = make_text(reader, ev)?;
                let node = document.create_comment(&pre_node.value);
                let _safe_to_ignore = ref_self.append_child(node).unwrap();
            }
            Ok(Event::Comment(ev)) => {
                let document = ref_self.owner_document().unwrap();
                let pre_node = make_comment(reader, ev)?;
                let node = document.create_comment(&pre_node.value);
                let _safe_to_ignore = ref_self.append_child(node).unwrap();
            }
            Ok(Event::CData(ev)) => {
                let document = ref_self.owner_document().unwrap();
                let pre_node = make_cdata(reader, ev)?;
                let node = document.create_cdata_section(&pre_node.value).unwrap();
                let _safe_to_ignore = ref_self.append_child(node).unwrap();
            }
            Ok(Event::PI(ev)) => {
                let document = ref_self.owner_document().unwrap();
                let pre_node = make_pi(reader, ev)?;
                let node = document
                    .create_processing_instruction(&pre_node.name, Some(&pre_node.value))
                    .unwrap();
                let _safe_to_ignore = ref_self.append_child(node).unwrap();
            }
            Ok(_) => return Error::Malformed.into(),
            Err(err) => return Error::from(err).into(),
        };
    }
}

fn misc(
    reader: &mut Reader<&[u8]>,
    event_buffer: &mut Vec<u8>,
    parent: &mut RefNode,
) -> Result<RefNode> {
    loop {
        match reader.read_event(event_buffer) {
            Ok(Event::Comment(ev)) => {
                let document = parent.owner_document().unwrap();
                let pre_node = make_comment(reader, ev)?;
                let node = document.create_comment(&pre_node.value);
                let _safe_to_ignore = parent.append_child(node).unwrap();
            }
            Ok(Event::PI(ev)) => {
                let document = parent.owner_document().unwrap();
                let pre_node = make_pi(reader, ev)?;
                let node = document
                    .create_processing_instruction(&pre_node.name, Some(&pre_node.value))
                    .unwrap();
                let _safe_to_ignore = parent.append_child(node).unwrap();
            }
            Ok(_) => return Error::Malformed.into(),
            Err(err) => return Error::from(err).into(),
        };
    }
}

///
/// Given a document that has been started, add to it.
///
fn read_loop_inner(
    mut reader: Reader<&[u8]>,
    mut event_buffer: Vec<u8>,
    mut _document: RefNode,
) -> Result<RefNode> {
    loop {
        let _child_node = match reader.read_event(&mut event_buffer) {
            Ok(Event::Start(_ev)) => {}
            Ok(Event::End(_ev)) => {}
            Ok(Event::Empty(_ev)) => {}
            Ok(Event::Text(_ev)) => {}
            Ok(Event::Comment(_ev)) => {}
            Ok(Event::CData(_ev)) => {}
            Ok(Event::Decl(_ev)) => {}
            Ok(Event::PI(_ev)) => {}
            Ok(Event::DocType(_ev)) => {}
            Ok(_) => return Error::Malformed.into(),
            Err(err) => return Error::from(err).into(),
        };
    }
}

// ------------------------------------------------------------------------------------------------

fn make_text(reader: &mut Reader<&[u8]>, ev: BytesText<'_>) -> Result<PreNode> {
    Ok(PreNode {
        kind: PreNodeKind::Text,
        name: String::new(),
        value: ev.unescape_and_decode(&reader)?,
        attributes: Default::default(),
    })
}

fn make_comment(reader: &mut Reader<&[u8]>, ev: BytesText<'_>) -> Result<PreNode> {
    Ok(PreNode {
        kind: PreNodeKind::Comment,
        name: String::new(),
        value: ev.unescape_and_decode(&reader)?,
        attributes: Default::default(),
    })
}

fn make_cdata(reader: &mut Reader<&[u8]>, ev: BytesText<'_>) -> Result<PreNode> {
    Ok(PreNode {
        kind: PreNodeKind::CData,
        name: String::new(),
        value: ev.unescape_and_decode(&reader)?,
        attributes: Default::default(),
    })
}

fn make_decl(reader: &mut Reader<&[u8]>, ev: BytesDecl<'_>) -> Result<PreNode> {
    let mut pre_node = PreNode {
        kind: PreNodeKind::XmlDecl,
        name: String::new(),
        value: String::new(),
        attributes: Default::default(),
    };
    let version = ev.version().unwrap();
    let version = version.borrow();
    let version = reader.decode(version).unwrap();
    let _safe_to_ignore = pre_node
        .attributes
        .insert("version".to_string(), unquote(version.to_string())?);
    if let Some(ev_value) = ev.encoding() {
        let encoding = ev_value.unwrap();
        let encoding = encoding.borrow();
        let encoding = reader.decode(encoding).unwrap();
        let _safe_to_ignore = pre_node
            .attributes
            .insert("encoding".to_string(), unquote(encoding.to_string())?);
    }
    if let Some(ev_value) = ev.standalone() {
        let standalone = ev_value.unwrap();
        let standalone = standalone.borrow();
        let standalone = reader.decode(standalone).unwrap();
        let _safe_to_ignore = pre_node
            .attributes
            .insert("standalone".to_string(), unquote(standalone.to_string())?);
    }
    Ok(pre_node)
}

fn make_doc_type(reader: &mut Reader<&[u8]>, ev: BytesText<'_>) -> Result<PreNode> {
    Ok(PreNode {
        kind: PreNodeKind::DocType,
        name: String::new(),
        value: ev.unescape_and_decode(&reader)?,
        attributes: Default::default(),
    })
}

fn make_pi(reader: &mut Reader<&[u8]>, ev: BytesText<'_>) -> Result<PreNode> {
    Ok(PreNode {
        kind: PreNodeKind::ProcessingInstruction,
        name: String::new(),
        value: ev.unescape_and_decode(&reader)?,
        attributes: Default::default(),
    })
}

fn make_document_node(prolog_nodes: Vec<PreNode>, root_element: PreNode) -> Result<RefNode> {
    let mut document_node = match prolog_nodes.iter().find(|n| n.kind == PreNodeKind::DocType) {
        None => {
            println!("{:?}", root_element);
            get_implementation().create_document("", &root_element.name, None)
        }
        Some(doc_type) => {
            let doc_type_node =
                get_implementation().create_document_type(&doc_type.name, None, None);
            get_implementation().create_document(
                "",
                &root_element.name,
                Some(doc_type_node.unwrap()),
            )
        }
    }
    .unwrap();
    let mut_document = as_document_decl_mut(&mut document_node).unwrap();
    let document_element = mut_document.document_element().unwrap();

    for prolog_node in prolog_nodes
        .iter()
        .filter(|n| n.kind != PreNodeKind::DocType)
    {
        match prolog_node.kind {
            PreNodeKind::XmlDecl => {
                let standalone = match prolog_node.attributes.get("standalone") {
                    None => None,
                    Some(s) => Some(s == "yes"),
                };
                let decl = XmlDecl::new(
                    XmlVersion::from_str(prolog_node.attributes.get("version").unwrap()).unwrap(),
                    prolog_node.attributes.get("encoding").cloned(),
                    standalone,
                );
                let _safe_to_ignore = mut_document.set_xml_declaration(decl).unwrap();
            }
            PreNodeKind::ProcessingInstruction => {
                let node = mut_document
                    .create_processing_instruction("", Some(""))
                    .unwrap();
                let _safe_to_ignore = mut_document
                    .insert_before(node, Some(document_element.clone()))
                    .unwrap();
            }
            PreNodeKind::Comment => {
                let node = mut_document.create_comment(&prolog_node.name);
                let _safe_to_ignore = mut_document
                    .insert_before(node, Some(document_element.clone()))
                    .unwrap();
            }
            _ => return Error::Malformed.into(),
        }
    }

    Ok(document_node)
}

fn unquote(s: String) -> Result<String> {
    if s.starts_with('"') && s.ends_with('"') {
        Ok(s[1..s.len() - 1].to_string())
    } else if s.starts_with('\'') && s.ends_with('\'') {
        Ok(s[1..s.len() - 1].to_string())
    } else if s.starts_with('"') || s.starts_with('\'') {
        return Error::InvalidCharacter.into();
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

    #[test]
    fn display_shortest_document() {
        println!("{:#?}", read_xml("<xml/>"));
    }
    /*
    #[allow(unused_must_use)]
    fn setup_logging() {
        // env_logger::Builder::from_default_env()
        //     .filter_module("upnp_rs::common::xml::read", LevelFilter::Trace)
        //     .try_init();
    }

    #[test]
    fn test_xml_read_minimal() {
        const TEST_DOC: &str = "<root></root>";
        setup_logging();
        let doc = read_xml(TEST_DOC);
        assert!(doc.is_ok());
        let doc = doc.unwrap();
        assert!(doc.document_element.is_some());
        assert_eq!(doc.document_element.unwrap().name.to_string(), "root");
        println!("{:#?}", doc);
    }

    #[test]
    fn test_xml_read_minimal_with_namespace() {
        const TEST_DOC: &str = "<root xmlns=\"urn:schemas-upnp-org:device-1-0\"></root>";
        setup_logging();
        let doc = read_xml(TEST_DOC);
        assert!(doc.is_ok());
        let doc = doc.unwrap();
        assert!(doc.document_element.is_some());
        assert_eq!(doc.document_element.unwrap().name.to_string(), "root");
        assert_eq!(doc.document_element.unwrap().attributes.len(), 1);
        let ns = doc.document_element.unwrap().attributes.get(0).unwrap();
        assert_eq!(ns.name.to_string(), "xmlns");
        assert_eq!(ns.value, "urn:schemas-upnp-org:device-1-0");
        println!("{:#?}", doc);
    }

    #[test]
    fn test_xml_read_minimal_with_decl() {
        const TEST_DOC: &str =
            "<?xml version=\"1.0\"?><root xmlns=\"urn:schemas-upnp-org:device-1-0\"></root>";
        setup_logging();
        let doc = read_xml(TEST_DOC);
        assert!(doc.is_ok());
        let doc = doc.unwrap();
        assert!(doc.document_element.is_some());
        assert_eq!(doc.document_element.unwrap().name.to_string(), "root");
        assert_eq!(doc.document_element.unwrap().attributes.len(), 1);
        let ns = doc.document_element.unwrap().attributes.get(0).unwrap();
        assert_eq!(ns.name.to_string(), "xmlns");
        assert_eq!(ns.value, "urn:schemas-upnp-org:device-1-0");
        assert_eq!(doc.processing_instructions.len(), 1);
        let pi = doc.processing_instructions.get(0).unwrap();
        assert_eq!(pi.target, "xml");
        println!("{:#?}", doc);
    }
    */
}
