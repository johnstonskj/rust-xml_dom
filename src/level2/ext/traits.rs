use crate::level2::ext::decl::XmlDecl;
use crate::level2::ext::namespaced::NamespacePrefix;
use crate::level2::ext::options::ProcessingOptions;
use crate::level2::traits as base;
use crate::shared::error::Result;

// ------------------------------------------------------------------------------------------------
// Public Traits
// ------------------------------------------------------------------------------------------------

///
/// This interface extends the DOM standard `Document` and allows the setting, and retrieval,
/// of the XML declaration from the document prolog.
///
/// # Specification
///
/// From XML 1.1 [§2.8 Prolog and Document Type Declaration](https://www.w3.org/TR/xml11/#sec-prolog-dtd)
/// -- Definition: XML 1.1 documents **must** begin with an **XML declaration** which specifies the
/// version of XML being used.
///
/// From XML 1.0 -- Definition: XML documents **should** begin with an **XML declaration** which
/// specifies the version of XML being used.
///
pub trait DocumentDecl: base::Document {
    ///
    /// Retrieve the current XML declaration, if set.
    ///
    fn xml_declaration(&self) -> Option<XmlDecl>;
    ///
    /// Set the current XML declaration for this document.
    ///
    /// Note that it is not possible to unset (set to `None`) this value.
    ///
    fn set_xml_declaration(&mut self, xml_decl: XmlDecl) -> Result<()>;
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `DOMImplementation` interface.
///
/// The instance used to create a document can be retrieved using the `implementation` method on
/// [`Document`](../trait.Document.html). To fetch an implementation to create a document iun the
/// first place use the function [`get_implementation`](../dom_impl/fn.get_implementation.html).
///
/// # Specification
///
/// The `DOMImplementation` interface provides a number of methods for performing operations that
/// are independent of any particular instance of the document object model.
///
pub trait DOMImplementation: base::DOMImplementation {
    ///
    /// Extension to the standard DOM `create_document` method that takes an options structure to
    /// control the processing of nodes. See the documentation for [`ProcessingOptions`](options/struct.ProcessingOptions.html)
    /// examples.
    ///
    /// * `options` of type `ProcessingOptions`: the options to be set for this document.
    ///
    fn create_document_with_options(
        &self,
        namespace_uri: Option<&str>,
        qualified_name: Option<&str>,
        doc_type: Option<Self::NodeRef>,
        options: ProcessingOptions,
    ) -> Result<Self::NodeRef>;
}

// ------------------------------------------------------------------------------------------------

///
/// An extended interface that provides access to namespace information for elements, including
/// the resolving of prefixes and namespaces in the hierarchy of the document.
///
/// The abstraction is of a hash map for each element that maps prefixes to namespace URIs. A
/// prefix is of type `Option<String>` so that the un-prefixed namespace can be represented as
/// the prefix `None`. URIs are simply stored as `String`s.
///
/// So, given the following XML:
///
/// ```xml
/// <element
///   xmlns="example.org/schema/common"
///   xmlns:p="example.org/schema/product"
///   xmlns:o="example.org/schema/order">
/// </element>
/// ```
///
/// we would get the following hash:
///
/// ```rust,ignore
/// {
///     Some(
///         "o",
///     ): "example.org/schema/order",
///     None: "example.org/schema/common",
///     Some(
///         "p",
///     ): "example.org/schema/product",
/// }
/// ```
///
pub trait Namespaced: base::Element {
    ///
    /// Returns `true` if this, and only this, element has a URI mapping for the provided `prefix`,
    /// `false` otherwise.
    ///
    fn contains_mapping(&self, prefix: Option<&str>) -> bool;
    ///
    /// Returns the namespace URI associated with the provided `prefix`, `None` if the prefix is not
    /// mapped to a URI for this, and only this, element.
    ///
    fn get_namespace(&self, prefix: Option<&str>) -> Option<String>;
    ///
    /// Returns the namespace URI associated with the provided `prefix` for this element by looking
    /// up the DOM tree through `parent_node` links. Returns `None` if the prefix is not mapped to a
    /// URI on this, or any parent, element.
    ///
    fn resolve_namespace(&self, prefix: Option<&str>) -> Option<String>;

    ///
    /// Returns `true` if this, and only this, element has a URI mapping for the provided
    /// `namespace_uri`, `false` otherwise.
    ///
    fn contains_mapped_namespace(&self, namespace_uri: &str) -> bool;
    ///
    /// Returns the prefix associated with the provided `namespace_uri`, `None` if the namespace
    /// URI is not mapped with a prefix for this, and only this, element.
    ///
    fn get_prefix(&self, namespace_uri: &str) -> NamespacePrefix;
    ///
    /// Returns the prefix associated with the provided `namespace_uri` for this element by looking
    /// up the DOM tree through `parent_node` links. Returns `None` if the namespace is not mapped
    /// with a prefix for this, or any parent, element.
    ///
    fn resolve_prefix(&self, namespace_uri: &str) -> NamespacePrefix;
}
