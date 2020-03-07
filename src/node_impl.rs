use self::super::name::Name;
use self::super::rc_cell::*;
use self::super::traits::NodeType;
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Internal DOM tree node owned reference.
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
pub type WeakRefNode = WeakRefCell<NodeImpl>;

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
