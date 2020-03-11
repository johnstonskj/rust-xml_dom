use crate::name::Name;
use crate::rc_cell::{RcRefCell, WeakRefCell};
use crate::text;
use crate::traits::NodeType;
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
    // For Elements
    pub(crate) i_namespaces: HashMap<Option<String>, String>,
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
            i_namespaces: Default::default(),
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub(crate) fn new_attribute(name: Name, value: Option<&str>) -> Self {
        Self {
            i_node_type: NodeType::Attribute,
            i_name: name,
            i_value: value.map(text::escape),
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_namespaces: Default::default(),
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub(crate) fn new_text(data: &str) -> Self {
        Self {
            i_node_type: NodeType::Text,
            i_name: Name::for_text(),
            i_value: Some(text::escape(data)),
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_namespaces: Default::default(),
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub(crate) fn new_cdata(data: &str) -> Self {
        Self {
            i_node_type: NodeType::CData,
            i_name: Name::for_cdata(),
            i_value: Some(text::escape(data)),
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_namespaces: Default::default(),
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub(crate) fn new_processing_instruction(target: Name, data: Option<&str>) -> Self {
        Self {
            i_node_type: NodeType::ProcessingInstruction,
            i_name: target,
            i_value: data.map(text::escape),
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_namespaces: Default::default(),
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub(crate) fn new_comment(data: &str) -> Self {
        Self {
            i_node_type: NodeType::Comment,
            i_name: Name::for_comment(),
            i_value: Some(text::escape(data)),
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_namespaces: Default::default(),
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
            i_namespaces: Default::default(),
            i_document_element: None,
            i_document_type: doc_type,
        }
    }
    pub(crate) fn new_document_type(name: Name, public_id: &str, system_id: &str) -> Self {
        let public_id = Self::new_attribute(Name::for_public_id(), Some(public_id));
        let system_id = Self::new_attribute(Name::for_system_id(), Some(system_id));
        let mut doc_type = Self {
            i_node_type: NodeType::DocumentType,
            i_name: name,
            i_value: None,
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_namespaces: Default::default(),
            i_document_element: None,
            i_document_type: None,
        };
        let _unused = doc_type
            .i_attributes
            .insert(public_id.i_name.clone(), RefNode::new(public_id));
        let _unused = doc_type
            .i_attributes
            .insert(system_id.i_name.clone(), RefNode::new(system_id));
        doc_type
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_impl::NodeImpl;
    use std::str::FromStr;

    #[test]
    fn test_escaping() {
        let name = Name::from_str("test").unwrap();
        let attribute = NodeImpl::new_attribute(name, Some("hello <\"world\"> & 'everyone' in it"));
        assert_eq!(
            attribute.i_value,
            Some("hello &#60;&#34;world&#34;&#62; &#38; &#39;everyone&#39; in it".to_string())
        )
    }

    #[test]
    fn test_doctype_structure() {
        let name = Name::from_str("html").unwrap();
        let doc_type = NodeImpl::new_document_type(
            name,
            "-//W3C//DTD XHTML 1.0 Transitional//EN",
            "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd",
        );
        assert_eq!(doc_type.i_attributes.len(), 2);
    }
}
