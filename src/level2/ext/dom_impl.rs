/*!
This module implements extended capabilities but not specified by the DOM Core.
*/

use crate::level2::dom_impl::Implementation;
use crate::level2::ext::traits::DOMImplementation;
use crate::level2::node_impl::{NodeImpl, RefNode};
use crate::shared::error::Result;
use crate::shared::name::Name;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

const THIS_IMPLEMENTATION: &'static dyn DOMImplementation<NodeRef = RefNode> = &Implementation {};

///
/// Return a reference to an instance of this `DOMImplementation` implementation.
///
/// This function gets around the DOM bootstrap issue, the `implementation` method on the
/// [`Document`](../trait.Document.html) trait requires an instance of `Document`; however, the
/// `create_document` method on `DOMImplementation` requires an instance from `implementation`.
///
/// # Example
///
/// ```rust
/// use xml_dom::level2::get_implementation;
///
/// let implementation = get_implementation();
/// let mut document_node = implementation
///     .create_document(Some("http://www.w3.org/1999/xhtml"), Some("html"), None)
///     .unwrap();
/// ```
///
pub fn get_implementation_ext() -> &'static dyn DOMImplementation<NodeRef = RefNode> {
    THIS_IMPLEMENTATION as &'static dyn DOMImplementation<NodeRef = RefNode>
}

///
/// Required to create instances of the [`Entity`](../trait.Entity.html) extended interface.
///
/// Rather than add a non-standard member to the [`Document`](../trait.Document.html) trait
/// this function takes a `Document` as the first parameter.
///
pub fn create_notation(
    owner_document: RefNode,
    notation_name: &str,
    public_id: Option<&str>,
    system_id: Option<&str>,
) -> Result<RefNode> {
    let name = Name::from_str(notation_name)?;
    let node_impl =
        NodeImpl::new_notation(Some(owner_document.downgrade()), name, public_id, system_id);
    Ok(RefNode::new(node_impl))
}

///
/// Required to create instances of the [`Entity`](../trait.Entity.html) extended interface.
///
/// Rather than add a non-standard member to the [`Document`](../trait.Document.html) trait
/// this function takes a `Document` as the first parameter.
///
pub fn create_entity(
    owner_document: RefNode,
    notation_name: &str,
    public_id: Option<&str>,
    system_id: Option<&str>,
) -> Result<RefNode> {
    let name = Name::from_str(notation_name)?;
    let node_impl =
        NodeImpl::new_entity(Some(owner_document.downgrade()), name, public_id, system_id);
    Ok(RefNode::new(node_impl))
}

///
/// Required to create instances of the [`Notation`](../trait.Notation.html) Extended interface.
///
/// Rather than add a non-standard member to the [`Document`](../trait.Document.html) trait
/// this function takes a `Document` as the first parameter.
///
pub fn create_internal_entity(
    owner_document: RefNode,
    notation_name: &str,
    value: &str,
) -> Result<RefNode> {
    let name = Name::from_str(notation_name)?;
    let node_impl = NodeImpl::new_internal_entity(Some(owner_document.downgrade()), name, value);
    Ok(RefNode::new(node_impl))
}
