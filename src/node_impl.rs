use crate::name::Name;
use crate::options::ProcessingOptions;
use crate::rc_cell::{RcRefCell, WeakRefCell};
use crate::traits::{Node, NodeType};
use crate::{text, XmlDecl};
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
pub(crate) enum Extension {
    None,
    Document {
        i_xml_declaration: Option<XmlDecl>,
        i_document_element: Option<RefNode>,
        i_document_type: Option<RefNode>,
        i_id_map: HashMap<String, WeakRefNode>,
        i_options: ProcessingOptions,
    },
    DocumentType {
        i_entities: HashMap<Name, RefNode>,
        i_notations: HashMap<Name, RefNode>,
        i_public_id: Option<String>,
        i_system_id: Option<String>,
        i_internal_subset: Option<String>,
    },
    Element {
        i_attributes: HashMap<Name, RefNode>,
        i_namespaces: HashMap<Option<String>, String>,
    },
    Entity {
        i_public_id: Option<String>,
        i_system_id: Option<String>,
        i_notation_name: Option<String>,
    },
    Notation {
        i_public_id: Option<String>,
        i_system_id: Option<String>,
    },
}

///
/// Internal container for DOM tree node data and state.
///
/// Note that while the fields are crate-only visible the struct itself MUST be public.
///
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct NodeImpl {
    pub(crate) i_node_type: NodeType,
    pub(crate) i_name: Name,
    pub(crate) i_value: Option<String>,
    pub(crate) i_parent_node: Option<WeakRefNode>,
    pub(crate) i_owner_document: Option<WeakRefNode>,
    pub(crate) i_child_nodes: Vec<RefNode>,
    pub(crate) i_extension: Extension,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl NodeImpl {
    pub(crate) fn new_element(owner_document: WeakRefNode, name: Name) -> Self {
        Self {
            i_node_type: NodeType::Element,
            i_name: name,
            i_value: None,
            i_parent_node: None,
            i_owner_document: Some(owner_document),
            i_child_nodes: vec![],
            i_extension: Extension::Element {
                i_attributes: Default::default(),
                i_namespaces: Default::default(),
            },
        }
    }
    pub(crate) fn new_attribute(
        owner_document: WeakRefNode,
        name: Name,
        value: Option<&str>,
    ) -> Self {
        Self {
            i_node_type: NodeType::Attribute,
            i_name: name,
            i_value: value.map(text::escape),
            i_parent_node: None,
            i_owner_document: Some(owner_document),
            i_child_nodes: vec![],
            i_extension: Extension::None,
        }
    }
    pub(crate) fn new_text(owner_document: WeakRefNode, data: &str) -> Self {
        Self {
            i_node_type: NodeType::Text,
            i_name: Name::for_text(),
            i_value: Some(text::escape(data)),
            i_parent_node: None,
            i_owner_document: Some(owner_document),
            i_child_nodes: vec![],
            i_extension: Extension::None,
        }
    }
    pub(crate) fn new_cdata(owner_document: WeakRefNode, data: &str) -> Self {
        Self {
            i_node_type: NodeType::CData,
            i_name: Name::for_cdata(),
            i_value: Some(text::escape(data)),
            i_parent_node: None,
            i_owner_document: Some(owner_document),
            i_child_nodes: vec![],
            i_extension: Extension::None,
        }
    }
    pub(crate) fn new_processing_instruction(
        owner_document: WeakRefNode,
        target: Name,
        data: Option<&str>,
    ) -> Self {
        Self {
            i_node_type: NodeType::ProcessingInstruction,
            i_name: target,
            i_value: data.map(String::from),
            i_parent_node: None,
            i_owner_document: Some(owner_document),
            i_child_nodes: vec![],
            i_extension: Extension::None,
        }
    }
    pub(crate) fn new_comment(owner_document: WeakRefNode, data: &str) -> Self {
        Self {
            i_node_type: NodeType::Comment,
            i_name: Name::for_comment(),
            i_value: Some(text::escape(data)),
            i_parent_node: None,
            i_owner_document: Some(owner_document),
            i_child_nodes: vec![],
            i_extension: Extension::None,
        }
    }
    pub(crate) fn new_document(
        name: Name,
        doc_type: Option<RefNode>,
        options: ProcessingOptions,
    ) -> Self {
        Self {
            i_node_type: NodeType::Document,
            i_name: name,
            i_value: None,
            i_parent_node: None,
            i_owner_document: None,
            i_child_nodes: vec![],
            i_extension: Extension::Document {
                i_xml_declaration: None,
                i_document_element: None,
                i_document_type: doc_type,
                i_id_map: Default::default(),
                i_options: options,
            },
        }
    }
    pub(crate) fn new_document_fragment(owner_document: WeakRefNode) -> Self {
        Self {
            i_node_type: NodeType::DocumentFragment,
            i_name: Name::for_document_fragment(),
            i_value: None,
            i_parent_node: None,
            i_owner_document: Some(owner_document),
            i_child_nodes: vec![],
            i_extension: Extension::None,
        }
    }
    pub(crate) fn new_document_type(
        owner_document: Option<WeakRefNode>,
        name: Name,
        public_id: Option<&str>,
        system_id: Option<&str>,
    ) -> Self {
        Self {
            i_node_type: NodeType::DocumentType,
            i_name: name,
            i_value: None,
            i_parent_node: owner_document.clone(),
            i_owner_document: owner_document,
            i_child_nodes: vec![],
            i_extension: Extension::DocumentType {
                i_entities: Default::default(),
                i_notations: Default::default(),
                i_public_id: public_id.map(String::from),
                i_system_id: system_id.map(String::from),
                i_internal_subset: None,
            },
        }
    }
    pub(crate) fn new_entity_reference(owner_document: WeakRefNode, name: Name) -> Self {
        Self {
            i_node_type: NodeType::EntityReference,
            i_name: name,
            i_value: None,
            i_parent_node: None,
            i_owner_document: Some(owner_document),
            i_child_nodes: vec![],
            i_extension: Extension::None,
        }
    }
    pub(crate) fn new_entity(
        owner_document: Option<WeakRefNode>,
        notation_name: Name,
        public_id: Option<&str>,
        system_id: Option<&str>,
    ) -> Self {
        Self {
            i_node_type: NodeType::Entity,
            i_name: notation_name,
            i_value: None,
            i_parent_node: None,
            i_owner_document: owner_document,
            i_child_nodes: vec![],
            i_extension: Extension::Entity {
                i_public_id: public_id.map(String::from),
                i_system_id: system_id.map(String::from),
                i_notation_name: None,
            },
        }
    }
    pub(crate) fn new_internal_entity(
        owner_document: Option<WeakRefNode>,
        notation_name: Name,
        value: &str,
    ) -> Self {
        Self {
            i_node_type: NodeType::Entity,
            i_name: notation_name,
            i_value: Some(value.to_string()),
            i_parent_node: None,
            i_owner_document: owner_document,
            i_child_nodes: vec![],
            i_extension: Extension::Entity {
                i_public_id: None,
                i_system_id: None,
                i_notation_name: None,
            },
        }
    }
    pub(crate) fn new_notation(
        owner_document: Option<WeakRefNode>,
        notation_name: Name,
        public_id: Option<&str>,
        system_id: Option<&str>,
    ) -> Self {
        Self {
            i_node_type: NodeType::Notation,
            i_name: notation_name,
            i_value: None,
            i_parent_node: None,
            i_owner_document: owner_document,
            i_child_nodes: vec![],
            i_extension: Extension::Notation {
                i_public_id: public_id.map(String::from),
                i_system_id: system_id.map(String::from),
            },
        }
    }
    pub(crate) fn clone_node(&self, deep: bool) -> Self {
        let extension = match &self.i_extension {
            Extension::None => Extension::None,
            Extension::Document {
                i_xml_declaration,
                i_document_element,
                i_document_type,
                i_id_map,
                i_options,
            } => Extension::Document {
                i_xml_declaration: i_xml_declaration.clone(),
                i_document_element: i_document_element.clone(),
                i_document_type: i_document_type.clone(),
                i_id_map: i_id_map.clone(),
                i_options: i_options.clone(),
            },
            Extension::DocumentType {
                i_entities,
                i_notations,
                i_public_id,
                i_system_id,
                i_internal_subset,
            } => Extension::DocumentType {
                i_entities: i_entities.clone(),
                i_notations: i_notations.clone(),
                i_public_id: i_public_id.clone(),
                i_system_id: i_system_id.clone(),
                i_internal_subset: i_internal_subset.clone(),
            },
            Extension::Element {
                i_attributes,
                i_namespaces,
            } => Extension::Element {
                i_attributes: i_attributes.clone(),
                i_namespaces: i_namespaces.clone(),
            },
            entity @ Extension::Entity { .. } => entity.clone(),
            notation @ Extension::Notation { .. } => notation.clone(),
        };
        Self {
            i_node_type: self.i_node_type.clone(),
            i_name: self.i_name.clone(),
            i_value: self.i_value.clone(),
            i_parent_node: None,
            i_owner_document: self.i_owner_document.clone(),
            i_child_nodes: if deep {
                self.i_child_nodes
                    .iter()
                    .map(|node| node.clone_node(deep).unwrap())
                    .collect()
            } else {
                vec![]
            },
            i_extension: extension,
        }
    }
}
