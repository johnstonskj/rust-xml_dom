use xml_dom::convert::*;
use xml_dom::*;

mod common;

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
fn test_insert_child_node() {
    let document_node = make_sibling_document();
    let ref_document = as_document(&document_node).unwrap();

    let mut root_node = ref_document.document_element().unwrap();
    let mut_root = as_element_mut(&mut root_node).unwrap();
    let child_nodes = mut_root.child_nodes();
    compare_node_names(
        &child_nodes,
        &["child-1", "child-2", "child-3", "child-4", "child-5"],
    );

    {
        common::sub_test("test_insert_child_node", "insert_before(_, mid_node)");
        let mid_node = child_nodes.get(2).unwrap();
        let new_child_node = ref_document.create_element("inserted-1").unwrap();
        let result = mut_root.insert_before(new_child_node, Some(mid_node.clone()));
        assert!(result.is_ok());
        let new_node = result.unwrap();
        assert!(new_node.parent_node().is_some());
        assert!(new_node.owner_document().is_some());
        compare_node_names(
            &mut_root.child_nodes(),
            &[
                "child-1",
                "child-2",
                "inserted-1",
                "child-3",
                "child-4",
                "child-5",
            ],
        );
    }

    {
        common::sub_test("test_insert_child_node", "insert_before(_, first_node)");
        let first_node = child_nodes.first().unwrap();
        let new_child_node = ref_document.create_element("inserted-2").unwrap();
        let result = mut_root.insert_before(new_child_node, Some(first_node.clone()));
        assert!(result.is_ok());
        let new_node = result.unwrap();
        assert!(new_node.parent_node().is_some());
        assert!(new_node.owner_document().is_some());
        compare_node_names(
            &mut_root.child_nodes(),
            &[
                "inserted-2",
                "child-1",
                "child-2",
                "inserted-1",
                "child-3",
                "child-4",
                "child-5",
            ],
        );
    }

    {
        common::sub_test("test_insert_child_node", "insert_before(_, last_node)");
        let new_child_node = ref_document.create_element("inserted-3").unwrap();
        let result = mut_root.insert_before(new_child_node, None);
        assert!(result.is_ok());
        let new_node = result.unwrap();
        assert!(new_node.parent_node().is_some());
        assert!(new_node.owner_document().is_some());
        compare_node_names(
            &mut_root.child_nodes(),
            &[
                "inserted-2",
                "child-1",
                "child-2",
                "inserted-1",
                "child-3",
                "child-4",
                "child-5",
                "inserted-3",
            ],
        );
    }

    {
        common::sub_test("test_insert_child_node", "insert_before(_, not_a_child)");
        let not_a_child = ref_document.create_element("not-a-child").unwrap();
        let new_child_node = ref_document.create_element("not-inserted").unwrap();
        let result = mut_root.insert_before(new_child_node, Some(not_a_child));
        assert!(result.is_err());
    }
}

#[test]
fn test_replace_child_node() {
    let document_node = make_sibling_document();
    let ref_document = as_document(&document_node).unwrap();

    let mut root_node = ref_document.document_element().unwrap();
    let mut_root = as_element_mut(&mut root_node).unwrap();
    let child_nodes = mut_root.child_nodes();
    compare_node_names(
        &child_nodes,
        &["child-1", "child-2", "child-3", "child-4", "child-5"],
    );

    {
        common::sub_test("test_replace_child_node", "remove_child(mid_node)");
        let mid_node = child_nodes.get(2).unwrap();
        let new_child_node = ref_document.create_element("inserted-1").unwrap();
        let result = mut_root.replace_child(new_child_node, mid_node.clone());
        assert!(result.is_ok());
        compare_node_names(
            &mut_root.child_nodes(),
            &["child-1", "child-2", "inserted-1", "child-4", "child-5"],
        );
    }

    {
        common::sub_test("test_replace_child_node", "remove_child(first_node)");
        let first_node = child_nodes.first().unwrap();
        let new_child_node = ref_document.create_element("inserted-2").unwrap();
        let result = mut_root.replace_child(new_child_node, first_node.clone());
        assert!(result.is_ok());
        compare_node_names(
            &mut_root.child_nodes(),
            &["inserted-2", "child-2", "inserted-1", "child-4", "child-5"],
        );
    }

    {
        common::sub_test("test_replace_child_node", "remove_child(last_node)");
        let last_node = child_nodes.last().unwrap();
        let new_child_node = ref_document.create_element("inserted-3").unwrap();
        let result = mut_root.replace_child(new_child_node, last_node.clone());
        assert!(result.is_ok());
        compare_node_names(
            &mut_root.child_nodes(),
            &[
                "inserted-2",
                "child-2",
                "inserted-1",
                "child-4",
                "inserted-3",
            ],
        );
    }

    {
        common::sub_test("test_replace_child_node", "remove_child(not_a_child)");
        let not_a_child = ref_document.create_element("not-a-child").unwrap();
        let new_child_node = ref_document.create_element("not-inserted").unwrap();
        let result = mut_root.replace_child(new_child_node, not_a_child);
        assert!(result.is_err());
    }
}

#[test]
fn test_remove_child_node() {
    let document_node = make_sibling_document();
    let ref_document = as_document(&document_node).unwrap();

    let mut root_node = ref_document.document_element().unwrap();
    let mut_root = as_element_mut(&mut root_node).unwrap();
    let child_nodes = mut_root.child_nodes();
    compare_node_names(
        &child_nodes,
        &["child-1", "child-2", "child-3", "child-4", "child-5"],
    );

    {
        common::sub_test("test_remove_child_node", "remove_child(mid_node)");
        let mid_node = child_nodes.get(2).unwrap();
        let result = mut_root.remove_child(mid_node.clone());
        assert!(result.is_ok());
        compare_node_names(
            &mut_root.child_nodes(),
            &["child-1", "child-2", "child-4", "child-5"],
        );
    }

    {
        common::sub_test("test_remove_child_node", "remove_child(first_node)");
        let first_node = child_nodes.first().unwrap();
        let result = mut_root.remove_child(first_node.clone());
        assert!(result.is_ok());
        compare_node_names(&mut_root.child_nodes(), &["child-2", "child-4", "child-5"]);
    }

    {
        common::sub_test("test_remove_child_node", "remove_child(last_node)");
        let last_node = child_nodes.last().unwrap();
        let result = mut_root.remove_child(last_node.clone());
        assert!(result.is_ok());
        compare_node_names(&mut_root.child_nodes(), &["child-2", "child-4"]);
    }

    {
        common::sub_test("test_remove_child_node", "remove_child(not_a_child)");
        let not_a_child = ref_document.create_element("not-a-child").unwrap();
        let result = mut_root.remove_child(not_a_child);
        assert!(result.is_err());
    }
}

#[test]
fn test_next_sibling() {
    let document_node = make_sibling_document();
    let ref_document = as_document(&document_node).unwrap();
    let root_node = ref_document.document_element().unwrap();
    let ref_root = as_element(&root_node).unwrap();
    let child_nodes = ref_root.child_nodes();

    //
    // Ask for siblings
    //
    let mid_node = child_nodes.get(2).unwrap();
    let ref_mid = as_element(mid_node).unwrap();
    assert_eq!(ref_mid.name().to_string(), "child-3".to_string());

    common::sub_test("test_next_sibling", "next_sibling() 1");
    let next_node = ref_mid.next_sibling().unwrap();
    let ref_next = as_element(&next_node).unwrap();
    assert_eq!(ref_next.name().to_string(), "child-4".to_string());

    common::sub_test("test_next_sibling", "next_sibling() 2");
    let last_node = ref_next.next_sibling().unwrap();
    let ref_last = as_element(&last_node).unwrap();
    assert_eq!(ref_last.name().to_string(), "child-5".to_string());

    common::sub_test("test_next_sibling", "next_sibling() 3");
    let no_node = ref_last.next_sibling();
    assert!(no_node.is_none());
}

#[test]
fn test_previous_sibling() {
    let document_node = make_sibling_document();
    let ref_document = as_document(&document_node).unwrap();
    let root_node = ref_document.document_element().unwrap();
    let ref_root = as_element(&root_node).unwrap();
    let child_nodes = ref_root.child_nodes();

    //
    // Ask for siblings
    //
    let mid_node = child_nodes.get(2).unwrap();
    let ref_mid = as_element(mid_node).unwrap();
    assert_eq!(ref_mid.name().to_string(), "child-3".to_string());

    common::sub_test("test_previous_sibling", "previous_sibling() 1");
    let previous_node = ref_mid.previous_sibling().unwrap();
    let ref_previous = as_element(&previous_node).unwrap();
    assert_eq!(ref_previous.name().to_string(), "child-2".to_string());

    common::sub_test("test_previous_sibling", "previous_sibling() 2");
    let first_node = ref_previous.previous_sibling().unwrap();
    let ref_first = as_element(&first_node).unwrap();
    assert_eq!(ref_first.name().to_string(), "child-1".to_string());

    common::sub_test("test_previous_sibling", "previous_sibling() 3");
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
        common::sub_test(
            "test_is_child_allowed",
            &format!(
                "{:?}.append_child({:?}) -> {}?",
                parent_type,
                child_type,
                allowed.contains(&child_type)
            ),
        );
        let child_node = make_node(document.clone(), child_type.clone(), "child");
        assert_eq!(
            parent_node.append_child(child_node).is_ok(),
            allowed.contains(&child_type)
        );
    }
}

fn append_element_node(parent_node: &mut RefNode, name: &str) -> RefNode {
    let mut_parent = as_element_mut(parent_node).unwrap();

    let mut document_node = mut_parent.owner_document().unwrap();
    let mut_document = as_document_mut(&mut document_node).unwrap();
    let new_element_node = mut_document.create_element(name).unwrap();

    let result = mut_parent.append_child(new_element_node.clone());
    assert!(result.is_ok());
    new_element_node
}

fn make_sibling_document() -> RefNode {
    let document_node = get_implementation()
        .create_document("http://example.org/", "root", None)
        .unwrap();
    let ref_document = as_document(&document_node).unwrap();
    let mut root_node = ref_document.document_element().unwrap();
    {
        for index in 1..6 {
            let _safe_to_ignore = append_element_node(&mut root_node, &format!("child-{}", index));
        }
    }

    {
        assert_eq!(root_node.child_nodes().len(), 5);
    }
    document_node
}

fn compare_node_names(nodes: &Vec<RefNode>, expected_names: &[&str]) {
    let names: Vec<String> = nodes.iter().map(|n| n.name().to_string()).collect();
    let expected_names: Vec<String> = expected_names.iter().map(|s| String::from(*s)).collect();
    assert_eq!(names, expected_names);
}
