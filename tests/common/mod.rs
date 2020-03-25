#![allow(dead_code)]

use std::fmt::Display;
use xml_dom::level2::convert::*;
use xml_dom::level2::*;

pub const DC_NS: &str = "http://purl.org/dc/elements/1.1/";
pub const RDF_NS: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
pub const XML_NS_URI: &str = "http://www.w3.org/XML/1998/namespace";
pub const XMLNS_NS: &str = "http://www.w3.org/2000/xmlns/";

pub fn create_empty_rdf_document() -> RefNode {
    let implementation = get_implementation();
    implementation
        .create_document(Some(RDF_NS), Some("rdf:RDF"), None)
        .unwrap()
}

// <rdf:RDF
//   xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
//   xmlns:dc="http://purl.org/dc/elements/1.1/">
//
//   <rdf:Description rdf:about="http://media.example.com/audio/guide.ra" id="main">
//
//     <dc:creator>Rose Bush</dc:creator>
//     <dc:title xml:id="title">A Guide to Growing Roses</dc:title>
//     <dc:Description id="description">Describes process for planting &#38; nurturing different kinds of rose bushes.</dc:Description>
//     <dc:date>2001-01-20</dc:date>
//
//   </rdf:Description>
// </rdf:RDF>

pub fn create_example_rdf_document() -> RefNode {
    create_example_rdf_document_options(ProcessingOptions::default())
}

#[allow(unused_must_use)]
pub fn create_example_rdf_document_options(options: ProcessingOptions) -> RefNode {
    let implementation = get_implementation();
    let mut document_node = implementation
        .create_document_with_options(Some(RDF_NS), Some("rdf:RDF"), None, options)
        .unwrap();
    let document = as_document_mut(&mut document_node).unwrap();
    let mut root_node = document.document_element().unwrap();
    let root_element = as_element_mut(&mut root_node).unwrap();
    root_element.set_attribute_ns(XMLNS_NS, "xmlns:rdf", RDF_NS);
    root_element.set_attribute("id", "main");

    let mut description_node = document
        .create_element_ns(RDF_NS, "rdf:Description")
        .unwrap();
    let description_element = as_element_mut(&mut description_node).unwrap();
    root_element.set_attribute_ns(
        RDF_NS,
        "rdf:about",
        "http://media.example.com/audio/guide.ra",
    );
    root_element.set_attribute_ns(XMLNS_NS, "xmlns:rdf", RDF_NS);

    description_element.append_child(create_element_with(
        document,
        DC_NS,
        "dc:creator",
        "Rose Bush",
    ));

    let mut new_element =
        create_element_with(document, DC_NS, "dc:title", "A Guide to Growing Roses");
    new_element.set_attribute_ns(XML_NS_URI, "xml:id", "title");
    description_element.append_child(new_element);

    let mut new_element = create_element_with(
        document,
        DC_NS,
        "dc:Description",
        "Describes process for planting & nurturing different kinds of rose bushes.",
    );
    new_element.set_attribute("id", "description");
    description_element.append_child(new_element);

    description_element.append_child(create_element_with(
        document,
        DC_NS,
        "dc:date",
        "2001-01-20",
    ));

    root_element.append_child(description_node);
    document_node
}

#[allow(unused_must_use)]
pub fn create_element_with(document: RefDocument, ns: &str, qn: &str, content: &str) -> RefNode {
    let mut node = document.create_element_ns(ns, qn).unwrap();
    let element = as_element_mut(&mut node).unwrap();
    element.append_child(document.create_text_node(content));
    node
}

#[inline]
pub fn sub_test(primary: &str, secondary: &str) {
    println!("**[{}]** sub-case: `{}`", primary, secondary);
}

#[inline]
pub fn sub_test_result<T: Display>(primary: &str, secondary: &str, result: T) {
    println!("**[{}]** sub-case: `{}` -> {}", primary, secondary, result);
}

#[inline]
pub fn sub_test_error<T: Display>(primary: &str, secondary: &str, result: T) {
    println!(
        "**[{}]** sub-case: `{}` ERROR {}",
        primary, secondary, result
    );
}
