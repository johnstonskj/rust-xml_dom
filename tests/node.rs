use xml_dom::*;

#[test]
fn test_is_child_allowed() {
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
#[ignore]
fn test_child_node_navigation() {}

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
