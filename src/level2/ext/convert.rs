/*!
Provides safe `RefNode` conversion functions, see [`crate::level2::convert`](../convert/index.html) for
more details.
*/

use crate::level2::ext::namespaced::MutNamespaced;
use crate::level2::ext::traits::*;
use crate::level2::node_impl::*;
use crate::level2::traits::NodeType;
use crate::shared::error::{Error, Result, MSG_INVALID_NODE_TYPE};

use crate::{make_is_as_functions, make_ref_type};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

make_ref_type!(RefDocumentDecl, MutRefDocumentDecl, DocumentDecl);

make_ref_type!(RefNamespaced, Namespaced);
pub(crate) type MutRefNamespaced<'a> = &'a mut dyn MutNamespaced<NodeRef = RefNode>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

make_is_as_functions!(
    is_document_decl,
    NodeType::Document,
    as_document_decl,
    RefDocumentDecl,
    as_document_decl_mut,
    MutRefDocumentDecl
);

make_is_as_functions!(
    is_element_namespaced,
    NodeType::Element,
    as_element_namespaced,
    RefNamespaced
);

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
