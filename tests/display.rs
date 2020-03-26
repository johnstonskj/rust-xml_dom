use xml_dom::level2::convert::{
    as_attribute_mut, as_document, as_document_fragment_mut, as_element_mut,
};
use xml_dom::level2::ext::convert::as_document_decl_mut;
use xml_dom::level2::ext::dom_impl as ext_dom_impl;
use xml_dom::level2::ext::{XmlDecl, XmlVersion};
use xml_dom::level2::get_implementation;

pub mod common;

#[test]
fn test_display_element() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();

    common::sub_test("test_display_element", "element_only");
    let mut test_node = document.create_element("test").unwrap();
    let result = format!("{}", test_node);
    assert_eq!(result, "<test></test>");

    common::sub_test("test_display_element", "element_with_attribute");
    {
        let element = as_element_mut(&mut test_node).unwrap();
        let attribute_node = document.create_attribute_with("test", "data").unwrap();
        assert!(element.set_attribute_node(attribute_node).is_ok());
    }
    let result = format!("{}", test_node);
    assert_eq!(result, "<test test=\"data\"></test>");

    common::sub_test(
        "test_display_element",
        "element_with_attribute_and_children",
    );
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
fn test_display_entity_reference() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();

    let test_node = document.create_entity_reference("amp").unwrap();

    let result = format!("{}", test_node);
    assert_eq!(result, "&amp;");
}

#[test]
fn test_display_processing_instruction() {
    let document_node = common::create_empty_rdf_document();
    let document = as_document(&document_node).unwrap();

    let test_node = document.create_processing_instruction("foo", None).unwrap();

    let result = format!("{}", test_node);
    assert_eq!(result, "<?foo?>");

    let test_node = document
        .create_processing_instruction("foo", Some("version=\"1.0\""))
        .unwrap();

    let result = format!("{}", test_node);
    assert_eq!(result, "<?foo version=\"1.0\"?>");
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
        .create_document(
            Some("http://www.w3.org/1999/xhtml"),
            Some("html"),
            Some(document_type),
        )
        .unwrap();

    let result = format!("{}", test_node);
    assert_eq!(result, "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" SYSTEM \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\"><html></html>");
}

#[test]
fn test_display_document_with_decl() {
    let implementation = get_implementation();
    let document_type = implementation
        .create_document_type(
            "html",
            Some("-//W3C//DTD XHTML 1.0 Transitional//EN"),
            Some("http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd"),
        )
        .unwrap();
    let mut test_node = implementation
        .create_document(
            Some("http://www.w3.org/1999/xhtml"),
            Some("html"),
            Some(document_type),
        )
        .unwrap();

    let mut_document = as_document_decl_mut(&mut test_node).unwrap();
    let xml_decl = XmlDecl::new(XmlVersion::V11, Some("UTF-8".to_string()), None);
    let result = mut_document.set_xml_declaration(xml_decl);
    assert!(result.is_ok());

    let result = format!("{}", test_node);
    assert_eq!(result, "<?xml version=\"1.1\" encoding=\"UTF-8\"?><!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" SYSTEM \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\"><html></html>");
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
fn test_display_document_fragment() {
    let implementation = get_implementation();
    let mut document_node = implementation
        .create_document(Some(common::RDF_NS), Some("rdf:RDF"), None)
        .unwrap();
    let document = as_document(&mut document_node).unwrap();

    let mut test_node = document.create_document_fragment().unwrap();
    let mut_fragment = as_document_fragment_mut(&mut test_node).unwrap();

    for name in vec!["one", "two", "three"] {
        let node = document.create_element(name).unwrap();
        let _safe_to_ignore = mut_fragment.append_child(node).unwrap();
    }

    let result = format!("{}", test_node);
    assert_eq!(
        result,
        "<![CDATA[#document-fragment <one></one><two></two><three></three>]]>"
    );
}
#[test]
fn test_display_entity() {
    let implementation = get_implementation();
    let document_node = implementation
        .create_document(Some(common::RDF_NS), Some("rdf:RDF"), None)
        .unwrap();

    let test_node =
        ext_dom_impl::create_internal_entity(document_node.clone(), "name", "My Name").unwrap();

    let result = format!("{}", test_node);
    assert_eq!(result, "<!ENTITY name \"My Name\">");

    common::sub_test(
        "test_display_entity",
        "create_entity(node, name, Some, None)",
    );
    let test_node =
        ext_dom_impl::create_entity(document_node.clone(), "name", Some("file-name.xml"), None)
            .unwrap();

    let result = format!("{}", test_node);
    assert_eq!(result, "<!ENTITY name PUBLIC \"file-name.xml\">");

    common::sub_test(
        "test_display_entity",
        "create_entity(node, name, None, Some)",
    );
    let test_node =
        ext_dom_impl::create_entity(document_node.clone(), "name", None, Some("file-name.xml"))
            .unwrap();

    let result = format!("{}", test_node);
    assert_eq!(result, "<!ENTITY name SYSTEM \"file-name.xml\">");

    common::sub_test(
        "test_display_entity",
        "create_entity(node, name, Some, Some)",
    );
    let test_node = ext_dom_impl::create_entity(
        document_node.clone(),
        "name",
        Some("foo-bar"),
        Some("file-name.xml"),
    )
    .unwrap();

    let result = format!("{}", test_node);
    assert_eq!(
        result,
        "<!ENTITY name PUBLIC \"foo-bar\" \"file-name.xml\">"
    );

    // ------------------------------------------------------------
    /*
            let mut test_node = dom_impl::create_entity(
                document_node.clone(),
                "name",
                Some("foo-bar"),
                Some("file-name.xml"),
            )
            .unwrap();
            {
                if let Extension::Entity {
                    i_notation_name, ..
                } = &mut test_node.i_extension
                {
                    *i_notation_name = Some("GIF".to_string());
                }
            }

            let result = format!("{}", test_node);
            assert_eq!(
                result,
                "<!ENTITY name PUBLIC \"foo-bar\" \"file-name.xml\" GIF>"
            );
    */
}

#[test]
fn test_display_notation() {
    let implementation = get_implementation();
    let document_node = implementation
        .create_document(Some(common::RDF_NS), Some("rdf:RDF"), None)
        .unwrap();

    let test_node =
        ext_dom_impl::create_notation(document_node.clone(), "name", Some("file-name.xml"), None)
            .unwrap();

    let result = format!("{}", test_node);
    assert_eq!(result, "<!NOTATION name PUBLIC \"file-name.xml\">");

    // ------------------------------------------------------------

    let test_node =
        ext_dom_impl::create_notation(document_node.clone(), "name", None, Some("file-name.xml"))
            .unwrap();

    let result = format!("{}", test_node);
    assert_eq!(result, "<!NOTATION name SYSTEM \"file-name.xml\">");

    // ------------------------------------------------------------

    let test_node = ext_dom_impl::create_notation(
        document_node.clone(),
        "name",
        Some("foo-bar"),
        Some("file-name.xml"),
    )
    .unwrap();

    let result = format!("{}", test_node);
    assert_eq!(
        result,
        "<!NOTATION name PUBLIC \"foo-bar\" \"file-name.xml\">"
    );
}
