use xml_dom::convert::*;
use xml_dom::*;

#[test]
fn test_is_child_allowed() {
    //
    // This logic is shared by append, insert, and replace, so we only test once.
    //
    let test_matrix: Vec<(NodeType, Vec<NodeType>)> = vec![
        (
            NodeType::Element,
            vec![
                NodeType::Element,
                NodeType::Text,
                NodeType::Comment,
                NodeType::ProcessingInstruction,
                NodeType::CData,
                NodeType::EntityReference,
            ],
        ),
        (
            NodeType::Attribute,
            vec![NodeType::Text, NodeType::EntityReference],
        ),
        (NodeType::Text, vec![]),
        (NodeType::CData, vec![]),
        (
            NodeType::EntityReference,
            vec![
                NodeType::Element,
                NodeType::Text,
                NodeType::Comment,
                NodeType::ProcessingInstruction,
                NodeType::CData,
                NodeType::EntityReference,
            ],
        ),
        (
            NodeType::Entity,
            vec![
                NodeType::Element,
                NodeType::Text,
                NodeType::Comment,
                NodeType::ProcessingInstruction,
                NodeType::CData,
                NodeType::EntityReference,
            ],
        ),
        (NodeType::ProcessingInstruction, vec![]),
        (NodeType::Comment, vec![]),
        (NodeType::DocumentType, vec![]),
        (
            NodeType::DocumentFragment,
            vec![
                NodeType::Element,
                NodeType::Text,
                NodeType::Comment,
                NodeType::ProcessingInstruction,
                NodeType::CData,
                NodeType::EntityReference,
            ],
        ),
        (NodeType::Notation, vec![]),
    ];

    let implementation = get_implementation();
    let document_node = implementation
        .create_document("http://www.w3.org/1999/xhtml", "html", None)
        .unwrap();

    for (parent_type, allowed_child_types) in test_matrix {
        test_parent(document_node.clone(), parent_type, &allowed_child_types);
    }
}

#[test]
#[ignore]
fn test_append_child_node() {}

#[test]
#[ignore]
fn test_insert_child_node() {}

#[test]
#[ignore]
fn test_replace_child_node() {}

#[test]
#[ignore]
fn test_remove_child_node() {}

#[test]
fn test_next_sibling() {
    //
    // Setup the tree
    //
    let mut document = make_document_node();
    let mut root_node = make_element_node(&mut document, "element");
    {
        let root_element = as_element_mut(&mut root_node).unwrap();

        for index in 1..6 {
            let child_node = make_element_node(&mut document, &format!("child-{}", index));
            let _ignore = root_element.append_child(child_node.clone());
        }
    }

    {
        assert_eq!(root_node.child_nodes().len(), 5);
    }

    //
    // Ask for siblings
    //
    let ref_root = as_element(&root_node).unwrap();
    let child_nodes = ref_root.child_nodes();
    let mid_node = child_nodes.get(2).unwrap();
    let ref_mid = as_element(mid_node).unwrap();
    assert_eq!(ref_mid.name().to_string(), "child-3".to_string());

    let next_node = ref_mid.next_sibling().unwrap();
    let ref_next = as_element(&next_node).unwrap();
    assert_eq!(ref_next.name().to_string(), "child-4".to_string());

    let last_node = ref_next.next_sibling().unwrap();
    let ref_last = as_element(&last_node).unwrap();
    assert_eq!(ref_last.name().to_string(), "child-5".to_string());

    let no_node = ref_last.next_sibling();
    assert!(no_node.is_none());
}

#[test]
fn test_previous_sibling() {
    let mut document = make_document_node();
    //
    // Setup the tree
    //
    let mut root_node = make_element_node(&mut document, "element");
    {
        let root_element = as_element_mut(&mut root_node).unwrap();

        for index in 1..6 {
            let child_node = make_element_node(&mut document, &format!("child-{}", index));
            let _ignore = root_element.append_child(child_node.clone());
        }
    }

    {
        assert_eq!(root_node.child_nodes().len(), 5);
    }

    //
    // Ask for siblings
    //
    let ref_root = as_element(&root_node).unwrap();
    let child_nodes = ref_root.child_nodes();
    let mid_node = child_nodes.get(2).unwrap();
    let ref_mid = as_element(mid_node).unwrap();
    assert_eq!(ref_mid.name().to_string(), "child-3".to_string());

    let previous_node = ref_mid.previous_sibling().unwrap();
    let ref_previous = as_element(&previous_node).unwrap();
    assert_eq!(ref_previous.name().to_string(), "child-2".to_string());

    let first_node = ref_previous.previous_sibling().unwrap();
    let ref_first = as_element(&first_node).unwrap();
    assert_eq!(ref_first.name().to_string(), "child-1".to_string());

    let no_node = ref_first.previous_sibling();
    assert!(no_node.is_none());
}

#[test]
#[ignore]
fn test_clone_node() {
    unimplemented!()
}

#[test]
#[ignore]
fn test_normalize() {
    unimplemented!()
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn make_node(document: RefNode, node_type: NodeType, prefix: &str) -> RefNode {
    let named = |s| format!("{}-{}", prefix, s);
    match node_type {
        NodeType::Element => document.create_element(&named("element")).unwrap(),
        NodeType::Attribute => document.create_attribute(&named("attribute")).unwrap(),
        NodeType::Text => document.create_text_node(&named("text")),
        NodeType::CData => document.create_cdata_section(&named("cdata")).unwrap(),
        NodeType::EntityReference => document.create_entity_reference(&named("text")).unwrap(),
        NodeType::ProcessingInstruction => document
            .create_processing_instruction(&named("pi"), None)
            .unwrap(),
        NodeType::Comment => document.create_comment(&named("comment")),
        NodeType::Document => document,
        NodeType::DocumentType => {
            let implementation = get_implementation();
            implementation
                .create_document_type(
                    "html",
                    Some("-//W3C//DTD XHTML 1.0 Transitional//EN"),
                    Some("http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd"),
                )
                .unwrap()
        }
        NodeType::DocumentFragment => document.create_document_fragment().unwrap(),
        // Created by dom_impl not Document
        NodeType::Entity => {
            dom_impl::create_internal_entity(document, &named("entity"), "some value").unwrap()
        }
        NodeType::Notation => {
            dom_impl::create_notation(document, &named("notation"), Some("file-name.xml"), None)
                .unwrap()
        }
    }
}

const ALL_CHILDREN: [NodeType; 12] = [
    NodeType::Element,
    NodeType::Attribute,
    NodeType::Text,
    NodeType::CData,
    NodeType::EntityReference,
    NodeType::Entity,
    NodeType::ProcessingInstruction,
    NodeType::Comment,
    NodeType::Document,
    NodeType::DocumentType,
    NodeType::DocumentFragment,
    NodeType::Notation,
];

fn test_parent(document: RefNode, parent_type: NodeType, allowed: &Vec<NodeType>) {
    let mut parent_node = make_node(document.clone(), parent_type.clone(), "parent");
    for child_type in ALL_CHILDREN.iter() {
        println!(
            "{:?}.append_child({:?}) -> {}?",
            parent_type,
            child_type,
            allowed.contains(&child_type)
        );
        let child_node = make_node(document.clone(), child_type.clone(), "child");
        assert_eq!(
            parent_node.append_child(child_node).is_ok(),
            allowed.contains(&child_type)
        );
    }
}

fn make_document_node() -> RefNode {
    get_implementation()
        .create_document("http://example.org/", "root", None)
        .unwrap()
}

fn make_element_node(document: &mut RefNode, name: &str) -> RefNode {
    let document = as_document_mut(document).unwrap();
    let element = document.create_element(name).unwrap();
    let mut document_element = document.document_element().unwrap();
    let document_element = as_element_mut(&mut document_element).unwrap();
    let result = document_element.append_child(element.clone());
    assert!(result.is_ok());
    element
}
