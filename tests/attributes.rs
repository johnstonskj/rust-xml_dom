use xml_dom::level2::convert::{as_attribute, as_attribute_mut, as_document, as_element_mut};
use xml_dom::level2::*;
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

#[test]
fn test_escaping() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();

    let attribute = document
        .create_attribute_with("test", "hello <\"world\"> & 'everyone' in it")
        .unwrap();
    assert_eq!(
        attribute.value(),
        Some("hello &#60;&#34;world&#34;&#62; &#38; &#39;everyone&#39; in it".to_string())
    )
}

#[test]
fn test_set_attribute() {
    //
    // From `Element::setAttribute()`:
    //
    // If an attribute with that name is already present in the element, its value is changed to
    // be that of the value parameter.
    //
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let mut element_node = document.document_element().unwrap();
    let element = as_element_mut(&mut element_node).unwrap();
    assert_eq!(element.attributes().len(), 0);

    // Create a simple attribute
    assert!(element.set_attribute("test-1", "1").is_ok());
    assert_eq!(element.attributes().len(), 1);
    assert_eq!(element.get_attribute("test-1").unwrap(), "1");

    // Create a second simple attribute
    assert!(element.set_attribute("test-2", "2").is_ok());
    assert_eq!(element.attributes().len(), 2);
    assert_eq!(element.get_attribute("test-2").unwrap(), "2");
    assert_eq!(element.get_attribute("test-1").unwrap(), "1");

    // Overwrite the first attribute
    assert!(element.set_attribute("test-1", "one").is_ok());
    assert_eq!(element.attributes().len(), 2);
    assert_eq!(element.get_attribute("test-1").unwrap(), "one");
    assert_eq!(element.get_attribute("test-2").unwrap(), "2");
}

#[test]
fn test_model_parent_owner() {
    //
    // From `Attr`:
    //
    // `Attr` objects inherit the `Node` interface, but since they are not actually child nodes of
    // the element they describe, the DOM does not consider them part of the document tree. Thus,
    // the `Node` attributes `parentNode`, `previousSibling`, and `nextSibling` have a `null` value
    // for `Attr` objects.
    //
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let attribute_node = document.create_attribute("test-1").unwrap();
    let attribute = as_attribute(&attribute_node).unwrap();
    assert!(attribute.owner_document().is_some());
    assert!(attribute.owner_element().is_none());
    assert!(attribute.parent_node().is_none());
    assert!(attribute.node_value().is_none());
    assert!(attribute.specified());
    assert!(!attribute.has_child_nodes());
    assert!(!attribute.has_attributes());

    let mut element_node = document.document_element().unwrap();
    let element = as_element_mut(&mut element_node).unwrap();
    assert!(element.set_attribute_node(attribute_node.clone()).is_ok());

    assert!(attribute.owner_document().is_some());
    assert!(attribute.owner_element().is_some()); // now assigned
    assert!(attribute.parent_node().is_none());
    assert!(attribute.node_value().is_none());
    assert!(attribute.specified());
    assert!(!attribute.has_child_nodes());
    assert!(!attribute.has_attributes());
}

#[test]
fn test_model_siblings() {
    //
    // From `Attr`:
    //
    // `Attr` objects inherit the `Node` interface, but since they are not actually child nodes of
    // the element they describe, the DOM does not consider them part of the document tree. Thus,
    // the `Node` attributes `parentNode`, `previousSibling`, and `nextSibling` have a `null` value
    // for `Attr` objects.
    //
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let mut element_node = document.document_element().unwrap();
    let element = as_element_mut(&mut element_node).unwrap();

    for n in 1..6 {
        assert!(element
            .set_attribute(&format!("test-{}", n), &n.to_string())
            .is_ok());
    }

    for n in 1..6 {
        let attribute_node = element.get_attribute_node(&format!("test-{}", n)).unwrap();
        let attribute = as_attribute(&attribute_node).unwrap();
        assert!(attribute.previous_sibling().is_none());
        assert!(attribute.next_sibling().is_none());
    }
}

#[test]
fn test_normalization_new_lines() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let mut element_node = document.document_element().unwrap();
    let element = as_element_mut(&mut element_node).unwrap();

    element.set_attribute("test", "hello\nworld").unwrap();
    assert_eq!(
        element.get_attribute("test"),
        Some("hello world".to_string())
    );

    element.set_attribute("test", "hello\u{0A}world").unwrap();
    assert_eq!(
        element.get_attribute("test"),
        Some("hello world".to_string())
    );

    element.set_attribute("test", "hello\u{0D}world").unwrap();
    assert_eq!(
        element.get_attribute("test"),
        Some("hello world".to_string())
    );

    element
        .set_attribute("test", "hello\u{0D}\u{0A}world")
        .unwrap();
    assert_eq!(
        element.get_attribute("test"),
        Some("hello world".to_string())
    );

    element
        .set_attribute("test", "hello\u{0D}\u{85}world")
        .unwrap();
    assert_eq!(
        element.get_attribute("test"),
        Some("hello world".to_string())
    );

    element.set_attribute("test", "hello\u{85}world").unwrap();
    assert_eq!(
        element.get_attribute("test"),
        Some("hello world".to_string())
    );

    element.set_attribute("test", "hello\u{2028}world").unwrap();
    assert_eq!(
        element.get_attribute("test"),
        Some("hello world".to_string())
    );
}

#[test]
fn test_normalization_whitespace() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let mut element_node = document.document_element().unwrap();
    let element = as_element_mut(&mut element_node).unwrap();

    element
        .set_attribute("test", "hello\u{09}\u{0A}\u{0D}world")
        .unwrap();
    assert_eq!(
        element.get_attribute("test"),
        Some("hello   world".to_string())
    );
}

#[test]
fn test_normalization_character_entities() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let mut element_node = document.document_element().unwrap();
    let element = as_element_mut(&mut element_node).unwrap();

    element.set_attribute("test", "hello&#xA7;world").unwrap();
    assert_eq!(
        element.get_attribute("test"),
        Some("hello§world".to_string())
    );

    element.set_attribute("test", "hello&#49;world").unwrap();
    assert_eq!(
        element.get_attribute("test"),
        Some("hello1world".to_string())
    );
}
