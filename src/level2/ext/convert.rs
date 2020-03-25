/*!
Provides safe `RefNode` conversion functions, see [`crate::level2::convert`](../convert/index.html) for
more details.
*/

use crate::level2::ext::namespaced::MutNamespaced;
use crate::level2::ext::traits::*;
use crate::level2::node_impl::*;
use crate::level2::traits::NodeType;
use crate::shared::error::{Error, Result, MSG_INVALID_NODE_TYPE};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Type for dynamic trait cast
pub type RefNamespaced<'a> = &'a dyn Namespaced<NodeRef = RefNode>;
pub(crate) type MutRefNamespaced<'a> = &'a mut dyn MutNamespaced<NodeRef = RefNode>;

/// Type for dynamic trait cast
pub type RefDocumentDecl<'a> = &'a dyn DocumentDecl<NodeRef = RefNode>;
/// Type for mutable dynamic trait cast
pub type MutRefDocumentDecl<'a> = &'a mut dyn DocumentDecl<NodeRef = RefNode>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Determines if the specified node is of type `NodeType::Element` and supports the trait
/// `Namespaced`.
///
#[inline]
pub fn is_element_namespaced(ref_node: &RefNode) -> bool {
    ref_node.borrow().i_node_type == NodeType::Element
}

///
/// Safely _cast_ the specified `RefNode` into a  `Namespaced` element.
///
#[inline]
pub fn as_element_namespaced(ref_node: &RefNode) -> Result<RefNamespaced<'_>> {
    if ref_node.borrow().i_node_type == NodeType::Element {
        Ok(ref_node as RefNamespaced<'_>)
    } else {
        warn!("{}", MSG_INVALID_NODE_TYPE);
        Err(Error::InvalidState)
    }
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `Namespaced` element.
///
#[inline]
pub(crate) fn as_element_namespaced_mut(ref_node: &mut RefNode) -> Result<MutRefNamespaced<'_>> {
    if ref_node.borrow().i_node_type == NodeType::Element {
        Ok(ref_node as MutRefNamespaced<'_>)
    } else {
        warn!("{}", MSG_INVALID_NODE_TYPE);
        Err(Error::InvalidState)
    }
}

///
/// Determines if the specified node is of type `NodeType::Document`.
///
#[inline]
pub fn is_document_decl(ref_node: &RefNode) -> bool {
    ref_node.borrow().i_node_type == NodeType::Document
}

///
/// Safely _cast_ the specified `RefNode` into a  `Document`.
///
#[inline]
pub fn as_document_decl(ref_node: &RefNode) -> Result<RefDocumentDecl<'_>> {
    if ref_node.borrow().i_node_type == NodeType::Document {
        Ok(ref_node as RefDocumentDecl<'_>)
    } else {
        warn!("{}", MSG_INVALID_NODE_TYPE);
        Err(Error::InvalidState)
    }
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `Document`.
///
#[inline]
pub fn as_document_decl_mut(ref_node: &mut RefNode) -> Result<MutRefDocumentDecl<'_>> {
    if ref_node.borrow().i_node_type == NodeType::Document {
        Ok(ref_node as MutRefDocumentDecl<'_>)
    } else {
        warn!("{}", MSG_INVALID_NODE_TYPE);
        Err(Error::InvalidState)
    }
}
