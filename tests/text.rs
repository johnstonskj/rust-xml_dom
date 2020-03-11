use xml_dom::convert::{as_document, as_element, as_text};

mod common;

#[test]
fn test_text_node_contents() {
    //
    // Note, this test character escaping, "&" should be "&#38;" in the tree.
    //
    let text_values = vec![
        "Rose Bush",
        "A Guide to Growing Roses",
        "Describes process for planting &#38; nurturing different kinds of rose bushes.",
        "2001-01-20",
    ];
    let root_node = common::create_example_rdf_document();
    let document = as_document(&root_node).unwrap();

    let document_element_node = document.document_element().unwrap();
    let root_element = as_element(&document_element_node).unwrap();

    let description_node = root_element.first_child().unwrap();
    let description_element = as_element(&description_node).unwrap();

    for (index, child) in description_element.child_nodes().iter().enumerate() {
        let child_element = as_element(&child).unwrap();
        let children = child_element.child_nodes();
        assert_eq!(children.len(), 1);
        let text = children.first().unwrap();
        let text = as_text(&text).unwrap();
        assert_eq!(text.node_value().unwrap(), text_values[index].to_string());
    }
}

#[test]
#[ignore]
fn test_text_modification() {}

#[test]
#[ignore]
fn test_cdata_split() {}

#[test]
#[ignore]
fn test_text_split() {}
