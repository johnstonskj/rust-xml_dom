/*!
This module contains extensions above and beyond the Level 2 specification.
*/

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

pub mod convert;

pub mod decl;
pub use decl::{XmlDecl, XmlVersion};

pub mod dom_impl;

pub mod options;
pub use options::ProcessingOptions;

pub mod namespaced;
pub use namespaced::NamespacePrefix;

pub(crate) mod traits;
pub use traits::*;

pub(crate) mod trait_impls;
