use xml_dom::convert::{
    as_cdata_section, as_cdata_section_mut, as_document, as_document_mut, as_element,
    as_element_mut, as_text, as_text_mut,
};
use xml_dom::Error;

pub mod common;

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
fn test_text_substring() {
    let mut document_node = common::create_empty_rdf_document();
    let document = as_document_mut(&mut document_node).unwrap();

    let mut root_node = document.document_element().unwrap();
    let root_element = as_element_mut(&mut root_node).unwrap();

    let text_node = document.create_text_node("Hello cruel world!");
    let text_node = root_element.append_child(text_node);
    assert!(text_node.is_ok());

    let children = root_element.child_nodes();
    assert_eq!(children.len(), 1);

    let mut text_node = children.get(0).unwrap().clone();
    let text = as_text_mut(&mut text_node).unwrap();
    assert_eq!(text.data(), Some("Hello cruel world!".to_string()));
    assert_eq!(text.substring(0, 0), Ok("".to_string()));
    assert_eq!(text.substring(6, 5), Ok("cruel".to_string()));
    assert_eq!(text.substring(12, 10), Ok("world!".to_string()));
    assert_eq!(text.substring(20, 5), Err(Error::IndexSize));
}

#[test]
fn test_text_insert() {
    let mut document_node = common::create_empty_rdf_document();
    let document = as_document_mut(&mut document_node).unwrap();

    let mut root_node = document.document_element().unwrap();
    let root_element = as_element_mut(&mut root_node).unwrap();

    let text_node = document.create_text_node("Hello cruel world!");
    let text_node = root_element.append_child(text_node);
    assert!(text_node.is_ok());

    let children = root_element.child_nodes();
    assert_eq!(children.len(), 1);

    let mut text_node = children.get(0).unwrap().clone();
    let text = as_text_mut(&mut text_node).unwrap();
    assert_eq!(text.data(), Some("Hello cruel world!".to_string()));

    let mut text_node = children.get(0).unwrap().clone();
    let text = as_text_mut(&mut text_node).unwrap();
    let result = text.insert(6, "my ");
    assert!(result.is_ok());
    assert_eq!(text.data(), Some("Hello my cruel world!".to_string()));

    let result = text.insert(30, "my ");
    assert!(result.is_err());
    assert_eq!(text.data(), Some("Hello my cruel world!".to_string()));

    let result = text.insert(0, "¡");
    assert!(result.is_ok());
    assert_eq!(text.data(), Some("¡Hello my cruel world!".to_string()));
}

#[test]
fn test_text_replace() {
    let mut document_node = common::create_empty_rdf_document();
    let document = as_document_mut(&mut document_node).unwrap();

    let mut root_node = document.document_element().unwrap();
    let root_element = as_element_mut(&mut root_node).unwrap();

    let text_node = document.create_text_node("Hello cruel world!");
    let text_node = root_element.append_child(text_node);
    assert!(text_node.is_ok());

    let children = root_element.child_nodes();
    assert_eq!(children.len(), 1);

    let mut text_node = children.get(0).unwrap().clone();
    let text = as_text_mut(&mut text_node).unwrap();
    assert_eq!(text.data(), Some("Hello cruel world!".to_string()));

    let mut text_node = children.get(0).unwrap().clone();
    let text = as_text_mut(&mut text_node).unwrap();
    let result = text.replace(6, 6, "my happy ");
    assert!(result.is_ok());
    assert_eq!(text.data(), Some("Hello my happy world!".to_string()));

    let result = text.replace(0, 0, "my happy ");
    assert!(result.is_ok());
    assert_eq!(
        text.data(),
        Some("my happy Hello my happy world!".to_string())
    );

    let result = text.replace(15, 30, "world");
    assert!(result.is_ok());
    assert_eq!(text.data(), Some("my happy Hello world".to_string()));
}

#[test]
fn test_text_delete() {
    let mut document_node = common::create_empty_rdf_document();
    let document = as_document_mut(&mut document_node).unwrap();

    let mut root_node = document.document_element().unwrap();
    let root_element = as_element_mut(&mut root_node).unwrap();

    let text_node = document.create_text_node("Hello cruel world!");
    let text_node = root_element.append_child(text_node);
    assert!(text_node.is_ok());

    let children = root_element.child_nodes();
    assert_eq!(children.len(), 1);

    let mut text_node = children.get(0).unwrap().clone();
    let text = as_text_mut(&mut text_node).unwrap();
    assert_eq!(text.data(), Some("Hello cruel world!".to_string()));

    let mut text_node = children.get(0).unwrap().clone();
    let text = as_text_mut(&mut text_node).unwrap();
    let result = text.delete(6, 6);
    assert!(result.is_ok());
    assert_eq!(text.data(), Some("Hello world!".to_string()));

    let result = text.delete(0, 0);
    assert!(result.is_ok());
    assert_eq!(text.data(), Some("Hello world!".to_string()));

    let result = text.delete(6, 0);
    assert!(result.is_ok());
    assert_eq!(text.data(), Some("Hello world!".to_string()));

    let result = text.delete(20, 6);
    assert!(result.is_err());
    assert_eq!(text.data(), Some("Hello world!".to_string()));

    let result = text.delete(6, 20);
    assert!(result.is_ok());
    assert_eq!(text.data(), Some("Hello ".to_string()));
}

#[test]
fn test_cdata_split() {
    let mut document_node = common::create_empty_rdf_document();
    let document = as_document_mut(&mut document_node).unwrap();

    let mut root_node = document.document_element().unwrap();
    let root_element = as_element_mut(&mut root_node).unwrap();

    let text_node = document.create_cdata_section("onetwo").unwrap();
    let _ignore = root_element.append_child(text_node);

    let children = root_element.child_nodes();
    assert_eq!(children.len(), 1);

    let mut text_node = children.get(0).unwrap().clone();
    let cdata = as_cdata_section_mut(&mut text_node).unwrap();
    let result = cdata.split(3);
    assert!(result.is_ok());

    let expected = vec!["one", "two"];
    for (index, child_node) in root_element.child_nodes().iter().enumerate() {
        // The following also ensures `node_type == NodeType::CData`
        let text = as_cdata_section(&child_node).unwrap();
        assert_eq!(text.data().unwrap(), expected[index].to_string());
    }
}

#[test]
fn test_text_split() {
    let mut document_node = common::create_empty_rdf_document();
    let document = as_document_mut(&mut document_node).unwrap();

    let mut root_node = document.document_element().unwrap();
    let root_element = as_element_mut(&mut root_node).unwrap();

    for content in vec!["onetwo", "threefour", "fivesix"] {
        let text_node = document.create_text_node(content);
        let _ignore = root_element.append_child(text_node);
    }

    let children = root_element.child_nodes();
    assert_eq!(children.len(), 3);

    let mut text_node = children.get(0).unwrap().clone();
    let text = as_text_mut(&mut text_node).unwrap();
    let result = text.split(3);
    assert!(result.is_ok());

    let children = root_element.child_nodes();
    assert_eq!(children.len(), 4);

    let mut text_node = children.get(2).unwrap().clone();
    let text = as_text_mut(&mut text_node).unwrap();
    let result = text.split(0);
    assert!(result.is_ok());

    let children = root_element.child_nodes();
    assert_eq!(children.len(), 5);

    let mut text_node = children.get(4).unwrap().clone();
    let text = as_text_mut(&mut text_node).unwrap();
    let result = text.split(9);
    assert!(result.is_ok());

    let children = root_element.child_nodes();
    assert_eq!(children.len(), 6);

    let expected = vec!["one", "two", "", "threefour", "fivesix", ""];
    for (index, child_node) in root_element.child_nodes().iter().enumerate() {
        // The following also ensures `node_type == NodeType::Text`
        let text = as_text(&child_node).unwrap();
        assert_eq!(text.data().unwrap(), expected[index].to_string());
    }
}
