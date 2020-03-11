use xml_dom::convert::*;
use xml_dom::*;

// <rdf:RDF
//   xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
//   xmlns:dc="http://purl.org/dc/elements/1.1/">
//
//   <rdf:Description rdf:about="http://media.example.com/audio/guide.ra" id="main">
//
//     <dc:creator>Rose Bush</dc:creator>
//     <dc:title>A Guide to Growing Roses</dc:title>
//     <dc:description>Describes process for planting &#38; nurturing different kinds of rose bushes.</dc:description>
//     <dc:date>2001-01-20</dc:date>
//
//   </rdf:Description>
// </rdf:RDF>
#[allow(unused_must_use)]
pub fn create_rdf_example() -> RefNode {
    const DC_NS: &str = "http://purl.org/dc/elements/1.1/";
    const RDF_NS: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
    const XMLNS_NS: &str = "http://www.w3.org/2000/xmlns/";

    let implementation = get_implementation();
    let mut document_node = implementation
        .create_document(RDF_NS, "rdf:RDF", None)
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
    description_element.append_child(create_element_with(
        document,
        DC_NS,
        "dc:title",
        "A Guide to Growing Roses",
    ));
    description_element.append_child(create_element_with(
        document,
        DC_NS,
        "dc:description",
        "Describes process for planting & nurturing different kinds of rose bushes.",
    ));
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
fn create_element_with(document: RefDocument, ns: &str, qn: &str, content: &str) -> RefNode {
    let mut node = document.create_element_ns(ns, qn).unwrap();
    let element = as_element_mut(&mut node).unwrap();
    element.append_child(document.create_text_node(content));
    node
}
