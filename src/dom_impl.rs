use crate::node_impl::RefNode;
use crate::traits::DOMImplementation;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
#[derive(Clone, Debug)]
pub(crate) struct Implementation {}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

const THIS_IMPLEMENTATION: &'static dyn DOMImplementation<NodeRef = RefNode> = &Implementation {};

///
/// Return a reference to an instance of this `DOMImplementation` implementation.
///
/// This function gets around the DOM bootstrap issue, the `implementation` method on the
/// [`Document`](trait.Document.html) trait requires an instance of `Document`; however, the
/// `create_document` method on `DOMImplementation` requires an instance from `implementation`.
///
/// # Example
///
/// ```rust
/// use xml_dom::get_implementation;
///
/// let implementation = get_implementation();
/// let mut document_node = implementation
///     .create_document("http://www.w3.org/1999/xhtml", "html", None)
///     .unwrap();
/// ```
///
pub fn get_implementation() -> &'static dyn DOMImplementation<NodeRef = RefNode> {
    THIS_IMPLEMENTATION
}

// ------------------------------------------------------------------------------------------------

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

///
/// Return a string with the vendor/version of the implementation.
///
pub fn get_implementation_version() -> String {
    format!("{}:{}", CRATE_NAME, CRATE_VERSION)
}
