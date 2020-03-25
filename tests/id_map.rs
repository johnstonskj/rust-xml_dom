use xml_dom::level2::convert::{as_document, as_element, as_element_mut};
use xml_dom::level2::*;

pub mod common;

#[test]
fn test_get_element_by_id_strict() {
    let document = common::create_example_rdf_document();
    let ref_document = as_document(&document).unwrap();

    let element = ref_document.get_element_by_id("title");
    assert!(element.is_some());
    {
        let element = element.unwrap();
        let ref_element = as_element(&element).unwrap();
        assert_eq!(ref_element.name().local_name(), &"title".to_string());
        assert_eq!(ref_element.name().prefix(), &Some("dc".to_string()));
    }

    let element = ref_document.get_element_by_id("description");
    assert!(element.is_none());

    let element = ref_document.get_element_by_id("unknown");
    assert!(element.is_none());

    let element = ref_document.get_element_by_id("");
    assert!(element.is_none());
}

#[test]
fn test_get_element_by_id_lax() {
    let mut options = ProcessingOptions::default();
    options.set_assume_ids();
    let document = common::create_example_rdf_document_options(options);
    let ref_document = as_document(&document).unwrap();

    let element = ref_document.get_element_by_id("title");
    assert!(element.is_some());
    {
        let element = element.unwrap();
        let ref_element = as_element(&element).unwrap();
        assert_eq!(ref_element.name().local_name(), &"title".to_string());
        assert_eq!(ref_element.name().prefix(), &Some("dc".to_string()));
    }

    let element = ref_document.get_element_by_id("description");
    assert!(element.is_some());
    {
        let element = element.unwrap();
        let ref_element = as_element(&element).unwrap();
        assert_eq!(ref_element.name().local_name(), &"Description".to_string());
        assert_eq!(ref_element.name().prefix(), &Some("dc".to_string()));
    }

    let element = ref_document.get_element_by_id("unknown");
    assert!(element.is_none());

    let element = ref_document.get_element_by_id("");
    assert!(element.is_none());
}

#[test]
#[allow(unused_must_use)]
fn test_no_duplicates() {
    let document = common::create_empty_rdf_document();
    let ref_document = as_document(&document).unwrap();
    let mut root_node = ref_document.document_element().unwrap();
    let root_element = as_element_mut(&mut root_node).unwrap();

    let mut new_element = common::create_element_with(
        ref_document,
        common::DC_NS,
        "dc:title",
        "A Guide to Growing Roses",
    );
    new_element.set_attribute_ns(common::XML_NS_URI, "xml:id", "title");
    root_element.append_child(new_element);

    let mut new_element = common::create_element_with(
        ref_document,
        common::DC_NS,
        "dc:title-2",
        "Another Guide to Growing Roses",
    );
    let result = new_element.set_attribute_ns(common::XML_NS_URI, "xml:id", "title");
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), Error::Syntax);
}
