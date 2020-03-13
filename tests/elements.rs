//
// `get_elements_by_tag_name` and `get_elements_by_tag_name_ns` tested in `document.rs`.
//

use xml_dom::convert::{as_attribute_mut, as_document, as_element, as_element_mut};
use xml_dom::RefNode;

mod common;

#[allow(unused_must_use)]
fn create_example_element() -> RefNode {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let mut element_node = document.document_element().unwrap();
    let element = as_element_mut(&mut element_node).unwrap();

    element.set_attribute("one", "ONE");
    element.set_attribute_ns(common::DC_NS, "dc:two", "TWO");

    let attribute = document.create_attribute_with("three", "THREE").unwrap();
    element.set_attribute_node(attribute);

    let mut attribute = document
        .create_attribute_ns(common::DC_NS, "dc:four")
        .unwrap();
    {
        let attribute = as_attribute_mut(&mut attribute).unwrap();
        attribute.set_value("FOUR");
    }
    element.set_attribute_node_ns(attribute);

    element_node
}

#[test]
fn test_add_attributes() {
    let element_node = create_example_element();
    let element = as_element(&element_node).unwrap();

    // Success -- proof is in the `create_example_element` function.
    assert_eq!(element.attributes().len(), 4);
}

#[test]
fn test_remove_attributes_success() {
    let mut element_node = create_example_element();
    let element = as_element_mut(&mut element_node).unwrap();
    assert_eq!(element.attributes().len(), 4);

    // Success
    assert!(element.remove_attribute("one").is_ok());
    assert_eq!(element.attributes().len(), 3);

    assert!(element.remove_attribute("dc:two").is_ok());
    assert_eq!(element.attributes().len(), 2);

    assert!(element.remove_attribute_ns(common::DC_NS, "four").is_ok());
    assert_eq!(element.attributes().len(), 1);
}

#[test]
fn test_remove_attributes_failure() {
    let mut element_node = create_example_element();
    let element = as_element_mut(&mut element_node).unwrap();
    assert_eq!(element.attributes().len(), 4);

    // Success
    assert!(element.remove_attribute("dc:one").is_ok());
    assert_eq!(element.attributes().len(), 4);

    assert!(element.remove_attribute("two").is_ok());
    assert_eq!(element.attributes().len(), 4);

    assert!(element
        .remove_attribute_ns(common::XMLNS_NS, "four")
        .is_ok());
    assert_eq!(element.attributes().len(), 4);
}

#[test]
fn test_has_attribute() {
    let element_node = create_example_element();
    let element = as_element(&element_node).unwrap();

    // Success
    assert!(element.has_attribute("one"));
    assert!(element.has_attribute("dc:two"));
    assert!(element.has_attribute("three"));
    assert!(element.has_attribute("dc:four"));

    assert!(element.has_attribute_ns(common::DC_NS, "two"));
    assert!(element.has_attribute_ns(common::DC_NS, "four"));

    // Failure
    assert!(!element.has_attribute("five"));
    assert!(!element.has_attribute("dc:one"));
    assert!(!element.has_attribute("two"));

    assert!(!element.has_attribute_ns(common::DC_NS, "three"));
    assert!(!element.has_attribute_ns(common::XMLNS_NS, "two"));
}

#[test]
fn test_get_attribute() {
    let element_node = create_example_element();
    let element = as_element(&element_node).unwrap();

    // Success
    assert!(element.get_attribute("one").is_some());
    assert!(element.get_attribute("dc:two").is_some());
    assert!(element.get_attribute("three").is_some());
    assert!(element.get_attribute("dc:four").is_some());

    assert!(element.get_attribute_ns(common::DC_NS, "two").is_some());
    assert!(element.get_attribute_ns(common::DC_NS, "four").is_some());

    // Failure
    assert!(element.get_attribute("five").is_none());
    assert!(element.get_attribute("dc:one").is_none());
    assert!(element.get_attribute("two").is_none());

    assert!(element.get_attribute_ns(common::DC_NS, "three").is_none());
    assert!(element.get_attribute_ns(common::XMLNS_NS, "two").is_none());
}
