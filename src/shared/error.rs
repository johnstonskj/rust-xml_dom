/*!
Provides a common `Error` and `Result` type and a set of common error messages.
*/

use std::fmt::{Display, Formatter};
use std::result::Result as StdResult;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Corresponds to the DOM `DomException` type.
///
/// # Specification
///
/// DOM operations only raise exceptions in "exceptional" circumstances, i.e., when an operation is
/// impossible to perform (either for logical reasons, because data is lost, or because the
/// implementation has become unstable). In general, DOM methods return specific error values in
/// ordinary processing situation, such as out-of-bound errors when using `NodeList`.
///
#[derive(Clone, Debug, PartialEq)]
#[repr(u16)]
pub enum Error {
    /// If index or size is negative, or greater than the allowed value
    IndexSize = 1,
    /// If the specified range of text does not fit into a DOMString
    StringSize,
    /// If any node is inserted somewhere it doesn't belong
    HierarchyRequest,
    /// If a node is used in a different document than the one that created it (that doesn't
    /// support it)
    WrongDocument,
    /// If an invalid or illegal character is specified, such as in a name. See production 2 in the
    /// XML specification for the definition of a legal character, and production 5 for the
    /// definition of a legal name character.
    InvalidCharacter,
    /// If data is specified for a node which does not support data
    NoDataAllowed,
    /// If an attempt is made to modify an object where modifications are not allowed
    NoModificationAllowed,
    /// If an attempt is made to reference a node in a context where it does not exist
    NotFound,
    /// If the implementation does not support the requested type of object or operation
    NotSupported,
    /// If an attempt is made to add an attribute that is already in use elsewhere
    InUseAttribute,
    /// If an attempt is made to use an object that is not, or is no longer, usable (introduced in DOM Level 2)
    InvalidState,
    /// If an invalid or illegal string is specified (introduced in DOM Level 2)
    Syntax,
    /// If an attempt is made to modify the type of the underlying object (introduced in DOM
    /// Level 2)
    InvalidModification,
    /// If an attempt is made to create or change an object in a way which is incorrect with
    /// regard to namespaces (introduced in DOM Level 2)
    Namespace,
    /// If a parameter or an operation is not supported by the underlying object (introduced in
    /// DOM Level 2)
    InvalidAccess,
}

///
/// This standard `Result` structure is used wherever an IDL function is marked as throwing
/// exceptions.
///
pub type Result<T> = StdResult<T, Error>;

// ------------------------------------------------------------------------------------------------
// Internal Log/Error Messages
// ------------------------------------------------------------------------------------------------

///
/// Error message: "The node `self` is not of the type expected by this method."
///
pub(crate) const MSG_INVALID_NODE_TYPE: &str =
    "The node `self` is not of the type expected by this method.";
///
/// Error message: "This node's extension does not match it's node type."
///
pub(crate) const MSG_INVALID_EXTENSION: &str =
    "This node's extension does not match it's node type.";
///
/// Error message: "The provided value could not be parsed into a `Name`."
///
pub(crate) const MSG_INVALID_NAME: &str = "The provided value could not be parsed into a `Name`.";
///
/// Error message: "This node is missing a `parent_node` value."
///
pub(crate) const MSG_NO_PARENT_NODE: &str = "This node is missing a `parent_node` value.";
///
/// Error message: "Cannot append or insert a child node created in a different document."
///
#[allow(dead_code)]
pub(crate) const MSG_WRONG_DOCUMENT: &str =
    "Cannot append or insert a child node created in a different document.";
///
/// Error message: "Either `offset` or `count` invalid for string operation."
///
pub(crate) const MSG_INDEX_ERROR: &str = "Either `offset` or `count` invalid for string operation.";
///
/// Error message: "Could not upgrade a weak reference."
///
pub(crate) const MSG_WEAK_REF: &str = "Could not upgrade a weak reference.";
///
/// Error message: "Violation of `xml:id` §4, attempt to insert duplicate ID value."
///
pub(crate) const MSG_DUPLICATE_ID: &str =
    "Violation of `xml:id` §4, attempt to insert duplicate ID value.";

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Error::IndexSize => "Either `index` or `size` is negative, or greater than the allowed value",
            Error::StringSize => "The specified range of text does not fit into a DOMString",
            Error::HierarchyRequest => "An attempt insert a node somewhere it doesn't belong",
            Error::WrongDocument => "An attempt to use a node in a different document than the one that created it",
            Error::InvalidCharacter => "An invalid or illegal character was specified, such as in a name",
            Error::NoDataAllowed => "An attempt to add data for a node which does not support data",
            Error::NoModificationAllowed => "An attempt is made to modify an object where modifications are not allowed",
            Error::NotFound => "An attempt is made to reference a node in a context where it does not exist",
            Error::NotSupported => "The implementation does not support the requested type of object or operation",
            Error::InUseAttribute => "An attempt was made to add an attribute that is already in use elsewhere",
            Error::InvalidState => "An attempt is made to use an object that is not, or is no longer, usable",
            Error::Syntax => "An invalid or illegal string was specified",
            Error::InvalidModification => "An attempt was made to modify the type of the underlying object",
            Error::Namespace => "An attempt was made to create or change an object in a way which is incorrect with regard to namespaces",
            Error::InvalidAccess => "A parameter or an operation is not supported by the underlying object",
        })
    }
}

impl std::error::Error for Error {}

impl<T> Into<Result<T>> for Error {
    fn into(self) -> Result<T> {
        Err(self)
    }
}
