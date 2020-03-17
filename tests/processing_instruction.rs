use xml_dom::convert::{as_document, as_processing_instruction_mut};

mod common;

#[test]
fn test_set_data() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let mut node = document
        .create_processing_instruction("test", Some("some data"))
        .unwrap();
    let processing_instruction = as_processing_instruction_mut(&mut node).unwrap();
    let expected_value = Some("some data".to_string());
    assert_eq!(processing_instruction.data(), expected_value);

    assert!(processing_instruction.set_data("nothing here").is_ok());
    let expected_value = Some("nothing here".to_string());
    assert_eq!(processing_instruction.data(), expected_value);
}

#[test]
fn test_unset_data() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let mut node = document
        .create_processing_instruction("test", Some("some data"))
        .unwrap();
    let processing_instruction = as_processing_instruction_mut(&mut node).unwrap();
    let expected_value = Some("some data".to_string());
    assert_eq!(processing_instruction.data(), expected_value);

    assert!(processing_instruction.unset_data().is_ok());
    assert!(processing_instruction.data().is_none());
}

#[test]
fn test_reserved_name() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();

    assert!(document
        .create_processing_instruction("xml", Some("reserved-name"))
        .is_err());

    assert!(document
        .create_processing_instruction("XML", Some("reserved-name"))
        .is_err());

    assert!(document
        .create_processing_instruction("xMl", Some("reserved-name"))
        .is_err());

    assert!(document
        .create_processing_instruction("xml-ok", Some("should-work"))
        .is_ok());
}
