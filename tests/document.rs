use std::str::FromStr;
use xml_dom::convert::{
    as_attribute, as_cdata_section, as_comment, as_document, as_element, as_processing_instruction,
    as_text,
};
use xml_dom::Name;

mod common;

const TEST_TEXT: &str = "Here is some useless text for testing";

#[test]
fn test_create_attribute() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let node = document.create_attribute("test").unwrap();
    let attribute = as_attribute(&node).unwrap();
    assert!(attribute.parent_node().is_none());
    assert!(attribute.owner_document().is_some());
    assert!(attribute.node_value().is_none());
    assert!(attribute.specified());
    let expected_name = Name::from_str("test").unwrap();
    assert_eq!(attribute.name(), expected_name);
    assert!(attribute.value().is_none());
    assert!(!attribute.has_child_nodes());
}

#[test]
fn test_create_attribute_ns() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let node = document
        .create_attribute_ns(common::DC_NS, "dc:test")
        .unwrap();
    let attribute = as_attribute(&node).unwrap();
    assert!(attribute.parent_node().is_none());
    assert!(attribute.owner_document().is_some());
    assert!(attribute.node_value().is_none());
    assert!(attribute.specified());
    let expected_name = Name::new_ns(common::DC_NS, "dc:test").unwrap();
    assert_eq!(attribute.name(), expected_name);
    assert!(attribute.value().is_none());
    assert!(!attribute.has_child_nodes());
}

#[test]
fn test_create_attribute_with() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let node = document
        .create_attribute_with("test", "some 'data'")
        .unwrap();
    let attribute = as_attribute(&node).unwrap();
    assert!(attribute.parent_node().is_none());
    assert!(attribute.owner_document().is_some());
    assert!(attribute.specified());
    let expected_name = Name::from_str("test").unwrap();
    assert_eq!(attribute.name(), expected_name);
    let expected_value = Some("some &#39;data&#39;".to_string());
    assert_eq!(attribute.value(), expected_value);
    assert!(!attribute.has_child_nodes());
}

#[test]
fn test_create_cdata_section() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let node = document.create_cdata_section(TEST_TEXT).unwrap();
    let cdata_section = as_cdata_section(&node).unwrap();
    assert!(cdata_section.parent_node().is_none());
    assert!(cdata_section.owner_document().is_some());
    assert_eq!(cdata_section.name(), Name::for_cdata());
    assert_eq!(cdata_section.length(), TEST_TEXT.len());
    assert_eq!(cdata_section.data(), Some(TEST_TEXT.to_string()));
    assert!(!cdata_section.has_child_nodes());
}

#[test]
#[ignore]
fn test_create_document_fragment() {
    unimplemented!()
}

#[test]
#[ignore]
fn test_create_entity_reference() {
    unimplemented!()
}

#[test]
fn test_create_comment() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let node = document.create_comment(TEST_TEXT);
    let comment = as_comment(&node).unwrap();
    assert!(comment.parent_node().is_none());
    assert!(comment.owner_document().is_some());
    assert_eq!(comment.name(), Name::for_comment());
    assert_eq!(comment.length(), TEST_TEXT.len());
    assert_eq!(comment.data(), Some(TEST_TEXT.to_string()));
    assert!(!comment.has_child_nodes());
}

#[test]
fn test_create_element() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let node = document.create_element("test").unwrap();
    let element = as_element(&node).unwrap();
    assert!(element.parent_node().is_none());
    assert!(element.owner_document().is_some());
    assert!(element.node_value().is_none());
    let expected_name = Name::from_str("test").unwrap();
    assert_eq!(element.name(), expected_name);
    assert!(!element.has_attributes());
    assert!(!element.has_child_nodes());
}

#[test]
fn test_create_element_ns() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let node = document
        .create_element_ns(common::DC_NS, "dc:test")
        .unwrap();
    let element = as_element(&node).unwrap();
    assert!(element.parent_node().is_none());
    assert!(element.owner_document().is_some());
    assert!(element.node_value().is_none());
    let expected_name = Name::new_ns(common::DC_NS, "dc:test").unwrap();
    assert_eq!(element.name(), expected_name);
    assert!(!element.has_attributes());
    assert!(!element.has_child_nodes());
}

#[test]
fn test_create_processing_instruction() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let node = document
        .create_processing_instruction("test", Some(TEST_TEXT))
        .unwrap();
    let processing_instruction = as_processing_instruction(&node).unwrap();
    assert!(processing_instruction.parent_node().is_none());
    assert!(processing_instruction.owner_document().is_some());
    let expected_name = Name::from_str("test").unwrap();
    assert_eq!(processing_instruction.name(), expected_name);
    assert_eq!(processing_instruction.target(), "test".to_string());
    assert_eq!(processing_instruction.length(), TEST_TEXT.len());
    assert_eq!(processing_instruction.data(), Some(TEST_TEXT.to_string()));
    assert!(!processing_instruction.has_child_nodes());
}

#[test]
fn test_create_text_node() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let node = document.create_text_node(TEST_TEXT);
    let text = as_text(&node).unwrap();
    assert!(text.parent_node().is_none());
    assert!(text.owner_document().is_some());
    assert_eq!(text.name(), Name::for_text());
    assert_eq!(text.length(), TEST_TEXT.len());
    assert_eq!(text.data(), Some(TEST_TEXT.to_string()));
    assert!(!text.has_child_nodes());
}

#[test]
fn test_get_element_by_id() {
    let root_node = common::create_example_rdf_document();
    let document = as_document(&root_node).unwrap();

    assert!(document.get_element_by_id("main").is_none());
}

#[test]
fn test_get_elements() {
    let root_node = common::create_example_rdf_document();
    let document = as_document(&root_node).unwrap();

    let elements = document.get_elements_by_tag_name("dc:creator");
    assert_eq!(elements.len(), 1);

    let elements = document.get_elements_by_tag_name("dc:created");
    assert_eq!(elements.len(), 0);

    let elements = document.get_elements_by_tag_name("*");
    assert_eq!(elements.len(), 6);
}

#[test]
fn test_get_elements_ns() {
    let root_node = common::create_example_rdf_document();
    let document = as_document(&root_node).unwrap();

    let elements = document.get_elements_by_tag_name_ns(common::DC_NS, "creator");
    assert_eq!(elements.len(), 1);

    let elements = document.get_elements_by_tag_name_ns(common::DC_NS, "created");
    assert_eq!(elements.len(), 0);

    let elements = document.get_elements_by_tag_name_ns(common::DC_NS, "*");
    assert_eq!(elements.len(), 4);

    let elements = document.get_elements_by_tag_name_ns(common::RDF_NS, "*");
    assert_eq!(elements.len(), 2);
}
