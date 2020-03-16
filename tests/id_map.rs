use xml_dom::convert::{as_document, as_element};
use xml_dom::ProcessingOptions;

mod common;

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
        assert_eq!(ref_element.name().local_name(), &"description".to_string());
        assert_eq!(ref_element.name().prefix(), &Some("dc".to_string()));
    }

    let element = ref_document.get_element_by_id("unknown");
    assert!(element.is_none());

    let element = ref_document.get_element_by_id("");
    assert!(element.is_none());
}
