use std::str::FromStr;
use xml_dom::level2::convert::{as_document, as_document_type, as_element};
use xml_dom::level2::{get_implementation, Name};

pub mod common;

#[test]
fn test_create_document_no_element() {
    let implementation = get_implementation();
    let document_node = implementation.create_document(None, None, None).unwrap();
    let document = as_document(&document_node).unwrap();
    assert!(document.document_element().is_none());
}

#[test]
fn test_create_document_namespaced_name() {
    let implementation = get_implementation();
    let document_node = implementation
        .create_document(Some(common::RDF_NS), Some("rdf:RDF"), None)
        .unwrap();
    let document = as_document(&document_node).unwrap();
    assert!(document.document_element().is_some());
}

#[test]
fn test_create_document_local_name() {
    let implementation = get_implementation();
    let document_node = implementation
        .create_document(None, Some("rdf:RDF"), None)
        .unwrap();
    let document = as_document(&document_node).unwrap();
    assert!(document.document_element().is_some());
}

#[test]
fn test_create_document_invalid_name() {
    let implementation = get_implementation();
    let document_node = implementation.create_document(Some(common::RDF_NS), None, None);
    assert!(document_node.is_err());
}

#[test]
fn test_create_document() {
    let implementation = get_implementation();
    let document_node = implementation
        .create_document(Some(common::RDF_NS), Some("rdf:RDF"), None)
        .unwrap();
    let document = as_document(&document_node).unwrap();
    assert!(document.document_element().is_some());
    let root_node = document.document_element().unwrap();
    let root_element = as_element(&root_node).unwrap();
    let expected_name = Name::new_ns(common::RDF_NS, "rdf:RDF").unwrap();
    assert_eq!(root_element.tag_name(), expected_name.to_string());
}

#[test]
fn test_create_document_type() {
    let implementation = get_implementation();
    let document_type_node = implementation
        .create_document_type(
            "html",
            Some("-//W3C//DTD XHTML 1.0 Transitional//EN"),
            Some("http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd"),
        )
        .unwrap();
    let document_type = as_document_type(&document_type_node).unwrap();
    assert_eq!(document_type.name(), Name::from_str("html").unwrap());
    let public_id = document_type.public_id().unwrap();
    assert_eq!(
        public_id,
        "-//W3C//DTD XHTML 1.0 Transitional//EN".to_string()
    );
    let system_id = document_type.system_id().unwrap();
    assert_eq!(
        system_id,
        "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd".to_string()
    );

    let document_node = implementation
        .create_document(
            Some(common::RDF_NS),
            Some("rdf:RDF"),
            Some(document_type_node.clone()),
        )
        .unwrap();
    let document = as_document(&document_node).unwrap();
    assert!(document.doc_type().is_some());
    let stored_doc_type = document.doc_type().unwrap();
    assert_eq!(&document_type_node, &stored_doc_type);
}
