use self::super::name::Name;
use self::super::rc_cell::*;
use self::super::traits::NodeType;
use crate::syntax::{XML_DOCTYPE_PUBLIC, XML_DOCTYPE_SYSTEM};
use crate::Element;
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Opaque DOM tree node reference. This is the type used by this implementation as the concrete
/// type for the `NodeRef` associated type in the  [`Node`](trait.Node.html) trait.
///
/// This is the common response type for DOM actions and can be cast to specific traits either
/// by-hand or using the [`xml_dom::convert`](convert/index.html) module. Also, note that this type
/// supports[`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) and so two nodes
/// can be tested to ensure they are the same.
///
pub type RefNode = RcRefCell<NodeImpl>;

///
/// Internal DOM tree node weak reference.
///
/// This is an opaque reference and can only used when converted into a
/// [`RefNode`](type.RefNode.html).
///
pub(crate) type WeakRefNode = WeakRefCell<NodeImpl>;

// ------------------------------------------------------------------------------------------------

///
/// Internal container for DOM tree node data and state.
///
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct NodeImpl {
    pub(crate) i_node_type: NodeType,
    pub(crate) i_name: Name,
    pub(crate) i_value: Option<String>,
    pub(crate) i_parent_node: Option<WeakRefNode>,
    pub(crate) i_owner_document: Option<WeakRefNode>,
    pub(crate) i_attributes: HashMap<Name, RefNode>,
    pub(crate) i_child_nodes: Vec<RefNode>,
    // for Document
    pub(crate) i_document_element: Option<RefNode>,
    pub(crate) i_document_type: Option<RefNode>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl NodeImpl {
    pub(crate) fn new_element(name: Name) -> Self {
        Self {
            i_node_type: NodeType::Element,
            i_name: name,
            i_value: None,
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub(crate) fn new_attribute(name: Name, value: Option<&str>) -> Self {
        Self {
            i_node_type: NodeType::Attribute,
            i_name: name,
            i_value: value.map(|v| v.to_string()),
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub(crate) fn new_text(data: &str) -> Self {
        Self {
            i_node_type: NodeType::Text,
            i_name: Name::for_text(),
            i_value: Some(data.to_string()),
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub(crate) fn new_cdata(data: &str) -> Self {
        Self {
            i_node_type: NodeType::CData,
            i_name: Name::for_cdata(),
            i_value: Some(data.to_string()),
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub(crate) fn new_processing_instruction(target: Name, data: Option<&str>) -> Self {
        Self {
            i_node_type: NodeType::ProcessingInstruction,
            i_name: target,
            i_value: data.map(|v| v.to_string()),
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub(crate) fn new_comment(data: &str) -> Self {
        Self {
            i_node_type: NodeType::Comment,
            i_name: Name::for_cdata(),
            i_value: Some(data.to_string()),
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub(crate) fn new_document(name: Name, doc_type: Option<RefNode>) -> Self {
        Self {
            i_node_type: NodeType::Document,
            i_name: name,
            i_value: None,
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: doc_type,
        }
    }
    pub(crate) fn new_document_type(name: Name, public_id: &str, system_id: &str) -> Self {
        let new_doc_type = Self {
            i_node_type: NodeType::DocumentType,
            i_name: name,
            i_value: None,
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        };
        let mut ref_node: RefNode = RcRefCell::new(new_doc_type);
        let as_element = &mut ref_node as &mut dyn Element<NodeRef = RefNode>;
        as_element
            .set_attribute(XML_DOCTYPE_PUBLIC, public_id)
            .expect("invalid public ID");
        as_element
            .set_attribute(XML_DOCTYPE_SYSTEM, system_id)
            .expect("invalid system ID");
        ref_node.unwrap()
    }
}
