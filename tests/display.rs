use xml_dom::convert::{as_attribute_mut, as_document, as_element_mut};
use xml_dom::get_implementation;

mod common;

#[test]
fn test_display_element() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();
    let mut test_node = document.create_element("test").unwrap();

    let result = format!("{}", test_node);
    assert_eq!(result, "<test></test>");

    {
        let element = as_element_mut(&mut test_node).unwrap();
        let attribute_node = document.create_attribute_with("test", "data").unwrap();
        assert!(element.set_attribute_node(attribute_node).is_ok());
    }
    let result = format!("{}", test_node);
    assert_eq!(result, "<test test=\"data\"></test>");

    {
        let element = as_element_mut(&mut test_node).unwrap();
        let attribute_node = document
            .create_attribute_ns(common::DC_NS, "dc:creator")
            .unwrap();
        assert!(element.set_attribute_node(attribute_node).is_ok());
    }
    let result = format!("{}", test_node);
    assert_eq!(result.len(), 39);
    assert!(result.starts_with("<test"));
    assert!(result.contains(" test=\"data\""));
    assert!(result.contains(" dc:creator=\"\""));
    assert!(result.ends_with("></test>"));
}

#[test]
fn test_display_attribute() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();

    let mut test_node = document.create_attribute("test").unwrap();
    let attribute = as_attribute_mut(&mut test_node).unwrap();
    assert!(attribute.set_value("test-data").is_ok());
    let result = format!("{}", test_node);
    assert_eq!(result, "test=\"test-data\"");

    let mut test_node = document
        .create_attribute_ns(common::DC_NS, "dc:creator")
        .unwrap();
    let attribute = as_attribute_mut(&mut test_node).unwrap();
    assert!(attribute.set_value("Rose Bush").is_ok());
    let result = format!("{}", test_node);
    assert_eq!(result, "dc:creator=\"Rose Bush\"");
}

#[test]
fn test_display_text() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();

    let test_node = document.create_text_node("this is textual test data");

    let result = format!("{}", test_node);
    assert_eq!(result, "this is textual test data");
}

#[test]
fn test_display_cdata() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();

    let test_node = document
        .create_cdata_section("this is textual test data")
        .unwrap();

    let result = format!("{}", test_node);
    assert_eq!(result, "<![CDATA[ this is textual test data ]]>");
}

#[test]
#[ignore]
fn test_display_entity_reference() {
    unimplemented!()
}

#[test]
#[ignore]
fn test_display_entity() {
    unimplemented!()
}

#[test]
fn test_display_processing_instruction() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();

    let test_node = document.create_processing_instruction("xml", None).unwrap();

    let result = format!("{}", test_node);
    assert_eq!(result, "<?xml>");

    let test_node = document
        .create_processing_instruction("xml", Some("version=\"1.0\""))
        .unwrap();

    let result = format!("{}", test_node);
    assert_eq!(result, "<?xml version=\"1.0\">");
}

#[test]
fn test_display_comment() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();

    let test_node = document.create_comment("this is textual test data");

    let result = format!("{}", test_node);
    assert_eq!(result, "<!--this is textual test data-->");
}

#[test]
fn test_display_document() {
    let implementation = get_implementation();
    let document_type = implementation
        .create_document_type(
            "html",
            Some("-//W3C//DTD XHTML 1.0 Transitional//EN"),
            Some("http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd"),
        )
        .unwrap();
    let test_node = implementation
        .create_document("http://www.w3.org/1999/xhtml", "html", Some(document_type))
        .unwrap();

    let result = format!("{}", test_node);
    assert_eq!(result, "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" SYSTEM \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\"><html></html>");
}

#[test]
fn test_display_document_type() {
    let implementation = get_implementation();
    let test_node = implementation
        .create_document_type(
            "html",
            Some("-//W3C//DTD XHTML 1.0 Transitional//EN"),
            Some("http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd"),
        )
        .unwrap();

    let result = format!("{}", test_node);
    assert_eq!(result, "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" SYSTEM \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">");
}

#[test]
#[ignore]
fn test_display_document_fragment() {
    unimplemented!()
}

#[test]
#[ignore]
fn test_display_notation() {
    unimplemented!()
}
