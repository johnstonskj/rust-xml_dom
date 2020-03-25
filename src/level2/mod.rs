/*!
Implementation for DOM Core Level 2.
*/

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

pub mod convert;

pub mod dom_impl;
pub use dom_impl::get_implementation;

mod node_impl;
pub use node_impl::RefNode;

mod traits;
pub use traits::*;

mod trait_impls;
pub use trait_impls::*;

// ------------------------------------------------------------------------------------------------
// Extended
// ------------------------------------------------------------------------------------------------

mod namespaced;
pub use namespaced::{NamespacePrefix, Namespaced};

mod options;
pub use options::ProcessingOptions;

// ------------------------------------------------------------------------------------------------
// Re-Export
// ------------------------------------------------------------------------------------------------

pub use crate::shared::error::{Error, Result};

pub use crate::shared::name::*;

pub use crate::shared::decl::{XmlDecl, XmlVersion};
