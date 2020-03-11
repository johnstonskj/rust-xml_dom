use std::str::FromStr;
use xml_dom::convert::{as_document, as_document_type, as_element};
use xml_dom::{get_implementation, Name};

mod common;

#[test]
fn test_create_document() {
    let implementation = get_implementation();
    let document_node = implementation
        .create_document(common::RDF_NS, "rdf:RDF", None)
        .unwrap();
    let document = as_document(&document_node).unwrap();
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
            "-//W3C//DTD XHTML 1.0 Transitional//EN",
            "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd",
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
        .create_document(common::RDF_NS, "rdf:RDF", Some(document_type_node.clone()))
        .unwrap();
    let document = as_document(&document_node).unwrap();
    let stored_doc_type = document.doc_type().unwrap();
    assert_eq!(&document_type_node, &stored_doc_type);
}
