use xml_dom::convert::{as_attribute_mut, as_document};

pub mod common;

#[test]
fn test_set_data() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let mut node = document.create_attribute_with("test", "some data").unwrap();
    let attribute = as_attribute_mut(&mut node).unwrap();
    let expected_value = Some("some data".to_string());
    assert_eq!(attribute.value(), expected_value);

    assert!(attribute.set_value("nothing here").is_ok());
    let expected_value = Some("nothing here".to_string());
    assert_eq!(attribute.value(), expected_value);
}

#[test]
fn test_unset_data() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let mut node = document.create_attribute_with("test", "some data").unwrap();
    let attribute = as_attribute_mut(&mut node).unwrap();
    let expected_value = Some("some data".to_string());
    assert_eq!(attribute.value(), expected_value);

    assert!(attribute.unset_value().is_ok());
    assert!(attribute.value().is_none());
}
