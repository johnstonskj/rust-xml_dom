use self::super::error::Result;
use self::super::name::Name;
use crate::options::ProcessingOptions;
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Traits
// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `Attr` interface.
///
pub trait Attribute: Node {
    ///
    /// On retrieval, the value of the attribute is returned as a string.
    ///
    /// # Specification
    ///
    /// Character and general entity references are replaced with their values. See also the method
    /// [`getAttribute`](trait.Element.html#tymethod.get_attribute) on the [`Element`](trait.Element.html)
    /// interface.
    ///
    /// On setting, this creates a `Text` node with the unparsed contents of the string. I.e. any
    /// characters that an XML processor would recognize as markup are instead treated as literal
    /// text. See also the method setAttribute on the Element interface.
    ///
    /// **Exceptions on setting**
    ///
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised when the node is readonly.
    ///
    fn value(&self) -> Option<String> {
        Node::node_value(self)
    }
    ///
    /// Set the `value` for the node; see [`value`](#tymethod.value).
    ///
    fn set_value(&mut self, value: &str) -> Result<()> {
        Node::set_node_value(self, value)
    }
    ///
    /// Set the `value` for the node to `None`; see [`value`](#tymethod.value).
    ///
    fn unset_value(&mut self) -> Result<()> {
        Node::unset_node_value(self)
    }
    ///
    /// If this attribute was explicitly given a value in the original document, this is `true`;
    /// otherwise, it is `false`.
    ///
    /// # Specification
    ///
    /// Note that the implementation is in charge of this attribute, not the user. If the user
    /// changes the value of the attribute (even if it ends up having the same value as the default
    /// value) then the specified flag is automatically flipped to true. To re-specify the
    /// attribute as the default value from the DTD, the user must delete the attribute. The
    /// implementation will then make a new attribute available with specified set to false and
    /// the default value (if one exists).
    ///
    /// In summary:
    ///
    /// * If the attribute has an assigned value in the document then specified is `true`, and the
    ///   value is the assigned value.
    /// * If the attribute has no assigned value in the document and has a default value in the
    ///   DTD, then specified is `false`, and the value is the default value in the DTD.
    /// * If the attribute has no assigned value in the document and has a value of `#IMPLIED` in
    ///   the DTD, then the attribute does not appear in the structure model of the document.
    /// * If the `ownerElement` attribute is `null` (i.e. because it was just created or was set to
    ///   `null` by the various removal and cloning operations) specified is `true`.
    ///
    fn specified(&self) -> bool {
        true
    }
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `CDataSection` interface.
///
/// # Specification
///
/// CDATA sections are used to escape blocks of text containing characters that would otherwise be
/// regarded as markup. The only delimiter that is recognized in a CDATA section is the `"]]>"`
/// string that ends the CDATA section. CDATA sections cannot be nested. Their primary purpose is
/// for including material such as XML fragments, without needing to escape all the delimiters.
///
/// The `DOMString` attribute of the `Text` node holds the text that is contained by the CDATA
/// section. Note that this may contain characters that need to be escaped outside of CDATA
/// sections and that, depending on the character encoding ("charset") chosen for serialization,
/// it may be impossible to write out some characters as part of a CDATA section.
///
/// The `CDATASection` interface inherits from the [`CharacterData`](trait.CharacterData.html)
/// interface through the [`Text`](trait.Text.html) interface. Adjacent `CDATASection` nodes are not
/// merged by use of the normalize method of the [`Node`](trait.Node.html) interface.
///
/// **Note:** Because no markup is recognized within a `CDATASection`, character numeric references
/// cannot be used as an escape mechanism when serializing. Therefore, action needs to be taken
/// when serializing a `CDATASection` with a character encoding where some of the contained
/// characters cannot be represented. Failure to do so would not produce well-formed XML.
///
/// One potential solution in the serialization process is to end the CDATA section before the
/// character, output the character using a character reference or entity reference, and open a
/// new CDATA section for any further characters in the text node. Note, however, that some
/// code conversion libraries at the time of writing do not return an error or exception when a
/// character is missing from the encoding, making the task of ensuring that data is not corrupted
/// on serialization more difficult.
///
pub trait CDataSection: Text {}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `CharacterData` interface.
///
/// # Specification
///
/// The `CharacterData` interface extends [`Node`](trait.Node.html) with a set of attributes and
/// methods for accessing character data in the DOM. For clarity this set is defined here rather
/// than on each object that uses these attributes and methods. No DOM objects correspond directly
/// to `CharacterData`, though [`Text`](trait.Text.html) and others do inherit the interface from it.
/// All offsets in this interface start from 0.
///
/// As explained in the `DOMString` interface, text strings in the DOM are represented in UTF-16,
/// i.e. as a sequence of 16-bit units. In the following, the term 16-bit units is used whenever
/// necessary to indicate that indexing on `CharacterData` is done in 16-bit units.
///
pub trait CharacterData: Node {
    ///
    /// The number of 16-bit units that are available through data and the `substringData` method
    /// below. This may have the value zero, i.e., `CharacterData` nodes may be empty.
    ///
    /// **Note:** This implementation drops the `_data` suffix from the methods for clarity.
    ///
    fn length(&self) -> usize {
        match self.data() {
            None => 0,
            Some(s) => s.len(),
        }
    }
    ///
    /// The character data of the node that implements this interface.
    ///
    /// # Specification
    ///
    /// The DOM implementation may not put arbitrary limits on the amount of data that may be
    /// stored in a `CharacterData` node. However, implementation limits may mean that the entirety
    /// of a node's data may not fit into a single `DOMString`. In such cases, the user may call
    /// `substringData` to retrieve the data in appropriately sized pieces.
    ///
    /// **Exceptions on setting**
    ///
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised when the node is readonly.
    ///
    /// **Exceptions on retrieval**
    ///
    /// * `DOMSTRING_SIZE_ERR`: Raised when it would return more characters than fit in a
    ///   `DOMString` variable on the implementation platform.
    ///
    fn data(&self) -> Option<String> {
        Node::node_value(self)
    }
    ///
    /// Set the `data` for the node; see [data()](#tymethod.data).
    ///
    fn set_data(&mut self, data: &str) -> Result<()> {
        Node::set_node_value(self, data)
    }
    ///
    /// Set the `data` for the node to `None`; see [data()](#tymethod.data).
    ///
    fn unset_data(&mut self) -> Result<()> {
        Node::unset_node_value(self)
    }
    ///
    /// Extracts a range of data from the node.
    ///
    /// # Specification
    ///
    /// **Parameters**
    ///
    /// * `offset` of type `unsigned long`: Start offset of substring to extract.
    /// * `count` of type `unsigned long`: The number of 16-bit units to extract.
    ///
    /// **Return Value**
    ///
    /// * `DOMString`: The specified substring. If the sum of `offset` and `count` exceeds the
    ///   `length`, then all 16-bit units to the end of the data are returned.
    ///
    /// **Exceptions**
    ///
    /// * `INDEX_SIZE_ERR`: Raised if the specified `offset` is negative or greater than the
    ///   number of 16-bit units in data, or if the specified `count` is negative.
    /// * `DOMSTRING_SIZE_ERR`: Raised if the specified range of text does not fit into a `DOMString`.
    ///
    fn substring(&self, offset: usize, count: usize) -> Result<String>;
    ///
    /// Append the string to the end of the character data of the node.
    ///
    /// # Specification
    ///
    /// Upon success, data provides access to the concatenation of data and the DOMString specified.
    ///
    /// **Parameters**
    ///
    /// * `arg` of type `DOMString`: The DOMString to append.
    ///
    /// **Exceptions**
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node is readonly.
    ///
    fn append(&mut self, data: &str) -> Result<()>;
    ///
    /// Insert a string at the specified 16-bit unit offset.
    ///
    /// # Specification
    ///
    /// **Parameters**
    ///
    /// * `offset` of type `unsigned long`: The character offset at which to insert.
    /// * `arg` of type `DOMString`: The DOMString to insert.
    ///
    /// **Exceptions**
    ///
    /// * `INDEX_SIZE_ERR`: Raised if the specified `offset` is negative or greater than the number
    ///   of 16-bit units in data.
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node is readonly.
    ///
    fn insert(&mut self, offset: usize, data: &str) -> Result<()>;
    ///
    /// Remove a range of 16-bit units from the node. Upon success, data and length reflect the change.
    ///
    /// # Specification
    ///
    /// **Parameters**
    ///
    /// * `offset` of type `unsigned long`: The offset from which to start removing.
    /// * `count` of type `unsigned long`: The number of 16-bit units to delete. If the sum of
    ///   `offset` and `count` exceeds `length` then all 16-bit units from offset to the end of
    ///   the data are deleted.
    ///
    /// **Exceptions**
    ///
    /// * `INDEX_SIZE_ERR`: Raised if the specified `offset` is negative or greater than the number
    ///   of 16-bit units in data, or if the specified `count` is negative.
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node is readonly.
    ///
    fn delete(&mut self, offset: usize, count: usize) -> Result<()>;
    ///
    /// Replace the characters starting at the specified 16-bit unit offset with the specified string.
    ///
    /// # Specification
    ///
    /// **Parameters**
    ///
    /// * `offset` of type `unsigned long`: The offset from which to start replacing.
    /// * `count` of type `unsigned long`: The number of 16-bit units to replace. If the sum of
    ///   `offset` and `count` exceeds `length`, then all 16-bit units to the end of the data are
    ///   replaced; (i.e., the effect is the same as a remove method call with the same range,
    ///   followed by an append method invocation).
    /// * `arg` of type `DOMString`: The `DOMString` with which the range must be replaced.
    /// Exceptions
    ///
    /// INDEX_SIZE_ERR: Raised if the specified `offset` is negative or greater than the number
    ///   of 16-bit units in data, or if the specified `count` is negative.
    /// NO_MODIFICATION_ALLOWED_ERR: Raised if this node is readonly.
    ///
    fn replace(&mut self, offset: usize, count: usize, data: &str) -> Result<()>;
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `Comment` interface.
///
/// # Specification
///
/// This interface inherits from [`CharacterData`](trait.CharacterData.html) and represents the
/// content of a comment, i.e., all the characters between the starting `'<!--'` and ending `'-->'`.
/// Note that this is the definition of a comment in XML, and, in practice, HTML, although some
/// HTML tools may implement the full SGML comment structure.
///
pub trait Comment: CharacterData {}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `Document` interface.
///
pub trait Document: Node {
    ///
    /// The Document Type Declaration (see [`DocumentType`](trait.DocumentType.html)) associated with
    /// this document.
    ///
    /// # Specification
    ///
    /// For HTML documents as well as XML documents without a document type
    /// declaration this returns `null`. The DOM Level 2 does not support editing the Document Type
    /// Declaration. `docType` cannot be altered in any way, including through the use of methods
    /// inherited from the [`Node`](trait.Node.html) interface, such as `insertNode` or `removeNode`.
    ///
    fn doc_type(&self) -> Option<Self::NodeRef>;
    ///
    /// This is a convenience attribute that allows direct access to the child node that is the
    /// root element of the document.
    ///
    /// # Specification
    ///
    /// For HTML documents, this is the element with the tagName `"HTML"`.
    ///
    fn document_element(&self) -> Option<Self::NodeRef>;
    ///
    /// The DOMImplementation object that handles this document.
    ///
    /// # Specification
    ///
    /// A DOM application may use objects from multiple implementations.
    ///
    fn implementation(&self) -> &dyn DOMImplementation<NodeRef = Self::NodeRef>;
    ///
    /// Creates an [`Attribute`](trait.Attribute.html) of the given name. Note that the `Attr`
    /// instance can then be set on an [`Element`](trait.Element.html) using the `setAttributeNode`
    /// method.
    ///
    /// # Specification
    ///
    /// To create an attribute with a qualified name and namespace URI, use the `createAttributeNS`
    /// method.
    ///
    /// **Parameters**
    ///
    /// * `name` of type `DOMString`: The name of the attribute.
    ///
    /// **Return Value**
    ///
    /// * `Attr`: A new `Attr` object with the `nodeName` attribute set to name, and `localName`,
    ///   `prefix`, and `namespaceURI` set to `null`. The value of the attribute is the empty string.
    ///
    /// **Exceptions**
    ///
    /// * `INVALID_CHARACTER_ERR`: Raised if the specified name contains an illegal character.
    ///
    fn create_attribute(&self, name: &str) -> Result<Self::NodeRef>;
    ///
    /// Implementation defined extension: this is the same as `create_attribute` except that it
    /// also sets the attribute value.
    ///
    fn create_attribute_with(&self, name: &str, value: &str) -> Result<Self::NodeRef>;
    ///
    /// Creates an attribute of the given qualified name and namespace URI.
    ///
    /// # Specification
    ///
    /// HTML-only DOM implementations do not need to implement this method.
    ///
    /// **Parameters**
    ///
    /// * `namespaceURI` of type `DOMString`: The namespace URI of the attribute to create.
    /// * `qualifiedName` of type `DOMString`: The qualified name of the attribute to instantiate.
    ///
    /// **Return Value**
    ///
    /// * [`Attr`](trait.Attribute.html): A new `Attr` object with the following attributes:
    ///
    /// | Attribute           | Value               |
    /// |---------------------|---------------------|
    /// | `Node.nodeName`     | `qualifiedName`     |
    /// | `Node.namespaceURI` | `namespaceURI`      |
    /// | `Node.prefix`       | prefix, extracted from `qualifiedName`, or `null` if there is no prefix |
    /// | `Node.localName`    | local name, extracted from `qualifiedName` |
    /// | `Attr.name`         | `qualifiedName`     |
    /// | `Node.nodeValue`    | the empty string    |
    ///
    /// **Exceptions**
    ///
    /// * `INVALID_CHARACTER_ERR`: Raised if the specified qualified name contains an illegal
    ///   character.
    /// * `NAMESPACE_ERR`: Raised if the `qualifiedName` is malformed, if the `qualifiedName` has
    ///   a `prefix` and the `namespaceURI` is `null`, if the `qualifiedName` has a `prefix` that
    ///   is "xml" and the `namespaceURI` is different from "http://www.w3.org/XML/1998/namespace",
    ///   or if the `qualifiedName` is "xmlns" and the namespaceURI is different from
    ///   "http://www.w3.org/2000/xmlns/".
    ///
    fn create_attribute_ns(
        &self,
        namespace_uri: &str,
        qualified_name: &str,
    ) -> Result<Self::NodeRef>;
    ///
    /// Creates a [`CDataSection`](trait.CDataSection.html) node whose value is the specified string.
    ///
    /// # Specification
    ///
    /// **Parameters**
    ///
    /// * `data` of type `DOMString`: The data for the `CDATASection` contents.
    ///
    /// **Return Value**
    ///
    /// * `CDATASection`: The new `CDATASection` object.
    ///
    /// **Exceptions**
    ///
    /// * `NOT_SUPPORTED_ERR`: Raised if this document is an HTML document.
    ///
    fn create_cdata_section(&self, data: &str) -> Result<Self::NodeRef>;
    ///
    /// Creates an empty DocumentFragment object.
    ///
    /// # Specification
    ///
    /// **Return Value**
    ///
    /// `DocumentFragment`: A new `DocumentFragment`.
    ///
    fn create_document_fragment(&self) -> Result<Self::NodeRef>;
    ///
    /// Creates an `EntityReference` object.
    ///
    /// # Specification
    ///
    /// In addition, if the referenced entity is known, the child list of the `EntityReference`
    /// node is made the same as that of the corresponding `Entity` node.
    ///
    /// **Note:** If any descendant of the `Entity` node has an unbound namespace prefix, the
    /// corresponding descendant of the created `EntityReference` node is also unbound; (its
    /// `namespaceURI` is `null`). The DOM Level 2 does not support any mechanism to resolve
    /// namespace prefixes.
    ///
    /// **Parameters**
    ///
    /// * `name` of type `DOMString`: The name of the entity to reference.
    ///
    /// **Return Value**
    ///
    /// * `EntityReference`: The new `EntityReference` object.
    ///
    /// **Exceptions**
    ///
    ///
    /// * `INVALID_CHARACTER_ERR`: Raised if the specified name contains an illegal character.
    /// * `NOT_SUPPORTED_ERR`: Raised if this document is an HTML document.
    ///
    fn create_entity_reference(&self, name: &str) -> Result<Self::NodeRef>;
    ///
    /// Creates a [`Comment`](trait.Comment.html) node given the specified string.
    ///
    /// # Specification
    ///
    /// **Parameters**
    ///
    /// * `data` of type `DOMString`: The data for the node.
    ///
    /// **Return Value**
    ///
    /// * `Comment`: The new `Comment` object.
    ///
    fn create_comment(&self, data: &str) -> Self::NodeRef;
    ///
    /// Creates an element of the type specified.
    ///
    /// # Specification
    ///
    /// Note that the instance returned implements the [`Element`](trait.Element.html) interface, so
    /// attributes can be specified directly on the returned object.
    ///
    /// In addition, if there are known attributes with default values, [`Attr`](trait.Attribute.html)
    /// nodes representing them are automatically created and attached to the element.
    ///
    /// To create an element with a qualified name and namespace URI, use the `createElementNS` method.
    ///
    /// **Parameters**
    ///
    /// * `tagName` of type `DOMString:  The name of the element type to instantiate. For XML, this
    ///   is case-sensitive. For HTML, the `tagName` parameter may be provided in any case, but
    ///   it must be mapped to the canonical uppercase form by the DOM implementation.
    ///
    /// **Return Value**
    ///
    /// * `Element`: A new `Element` object with the `nodeName` attribute set to `tagName`, and
    ///   `localName`, `prefix`, and `namespaceURI` set to `null`.
    ///
    /// **Exceptions**
    ///
    /// * `INVALID_CHARACTER_ERR`: Raised if the specified name contains an illegal character.
    ///
    fn create_element(&self, tag_name: &str) -> Result<Self::NodeRef>;
    ///
    /// Creates an element of the given qualified name and namespace URI.
    ///
    /// # Specification
    ///
    /// HTML-only DOM implementations do not need to implement this method.
    ///
    /// **Parameters**
    ///
    /// * `namespaceURI` of type `DOMString`: The namespace URI of the attribute to create.
    /// * `qualifiedName` of type `DOMString`: The qualified name of the attribute to instantiate.
    ///
    /// **Return Value**
    ///
    /// * `Element`: A new Element object with the following attributes:
    ///
    /// | Attribute           | Value               |
    /// |---------------------|---------------------|
    /// | `Node.nodeName`     | `qualifiedName`     |
    /// | `Node.namespaceURI` | `namespaceURI`      |
    /// | `Node.prefix`       | prefix, extracted from `qualifiedName`, or `null` if there is no prefix |
    /// | `Node.localName`    | local name, extracted from `qualifiedName` |
    /// | `Attr.name`         | `qualifiedName`     |
    /// | `Node.nodeValue`    | the empty string    |
    ///
    /// **Exceptions**
    ///
    /// * `INVALID_CHARACTER_ERR`: Raised if the specified qualified name contains an illegal
    ///   character.
    /// * `NAMESPACE_ERR`: Raised if the `qualifiedName` is malformed, if the `qualifiedName` has
    ///   a `prefix` and the `namespaceURI` is `null`, if the `qualifiedName` has a `prefix` that
    ///   is "xml" and the `namespaceURI` is different from "http://www.w3.org/XML/1998/namespace",
    ///   or if the `qualifiedName` is "xmlns" and the namespaceURI is different from
    ///   "http://www.w3.org/2000/xmlns/".
    ///
    fn create_element_ns(&self, namespace_uri: &str, qualified_name: &str)
        -> Result<Self::NodeRef>;
    ///
    /// Creates a [`ProcessingInstruction`](trait.ProcessingInstruction.html) node given the
    /// specified name and data strings.
    ///
    /// # Specification
    ///
    /// **Parameters**
    ///
    /// * `target` of type `DOMString`: The target part of the processing instruction.
    /// * `data` of type `DOMString`: The `data` for the `node`.
    ///
    /// **Return Value**
    ///
    /// * `ProcessingInstruction`: The new `ProcessingInstruction` object.
    ///
    /// **Exceptions**
    ///
    /// * `INVALID_CHARACTER_ERR`: Raised if the specified target contains an illegal character.
    /// * `NOT_SUPPORTED_ERR`: Raised if this document is an HTML document.
    ///
    fn create_processing_instruction(
        &self,
        target: &str,
        data: Option<&str>,
    ) -> Result<Self::NodeRef>;
    ///
    /// Creates a [`Text`](trait.Text.html) node given the specified string.
    ///
    /// # Specification
    ///
    /// **Parameters**
    ///
    /// * `data` of type `DOMString`: The data for the node.
    ///
    /// **Return Value**
    ///
    /// * `Text`: The new Text object.
    ///
    fn create_text_node(&self, data: &str) -> Self::NodeRef;
    ///
    /// Returns the [`Element`](trait.Element.html) whose ID is given by `elementId`.
    ///
    /// **Note:** this implementation does not support this method, it will always return `None`.
    ///
    /// # Specification
    ///
    /// If no such element exists, returns `null`. Behavior is not defined if more than one element
    /// has this ID.
    ///
    /// **Note:** The DOM implementation must have information that says which attributes are of type
    /// ID. Attributes with the name "ID" are not of type ID unless so defined. Implementations
    /// that do not know whether attributes are of type ID or not are expected to return `null`.
    ///
    /// **Parameters**
    ///
    /// * `elementId` of type `DOMString`: The unique id value for an element.
    ///
    /// **Return Value**
    ///
    /// * `Element`: The matching element.
    ///
    fn get_element_by_id(&self, id: &str) -> Option<Self::NodeRef>;
    ///
    /// Returns a `NodeList` of all the [`Element`](trait.Element.html)s with a given tag name in the
    /// order in which they are encountered in a preorder traversal of the Document tree.
    ///
    /// **Note:** This method will panic if `document_element` is not an `Element` node.
    ///
    /// # Specification
    ///
    /// **Parameters**
    ///
    /// * `tagname` of type `DOMString`: The name of the tag to match on. The special value "*"
    ///   matches all tags.
    ///
    /// **Return Value**
    ///
    /// * `NodeList`: A new `NodeList` object containing all the matched `Element`s.
    ///
    fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<Self::NodeRef>;
    ///
    /// Returns a `NodeList` of all the [`Element`](trait.Element.html)s with a given local name and
    /// namespace URI in the order in which they are encountered in a preorder traversal of the
    /// Document tree.
    ///
    /// **Note:** This method will panic if `document_element` is not an `Element` node.
    ///
    /// # Specification
    ///
    /// **Parameters**
    ///
    /// * `namespaceURI` of type `DOMString`: The namespace URI of the elements to match on. The
    ///   special value "*" matches all namespaces.
    /// * `localName` of type `DOMString`: The local name of the elements to match on. The special
    ///   value "*" matches all local names.
    ///
    /// **Return Value**
    ///
    /// * `NodeList`: A new `NodeList` object containing all the matched `Element`s.
    ///
    fn get_elements_by_tag_name_ns(
        &self,
        namespace_uri: &str,
        local_name: &str,
    ) -> Vec<Self::NodeRef>;
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `DocumentFragment` interface (current unsupported).
///
/// # Specification
///
/// `DocumentFragment` is a "lightweight" or "minimal" [`Document`](trait.Document.html) object. It
/// is very common to want to be able to extract a portion of a document's tree or to create a new
/// fragment of a document. Imagine implementing a user command like cut or rearranging a document
/// by moving fragments around. It is desirable to have an object which can hold such fragments and
/// it is quite natural to use a Node for this purpose. While it is true that a `Document` object
/// could fulfill this role, a `Document` object can potentially be a heavyweight object, depending
/// on the underlying implementation. What is really needed for this is a very lightweight object.
/// `DocumentFragment` is such an object.
///
/// Furthermore, various operations -- such as inserting nodes as children of another Node -- may
/// take `DocumentFragment` objects as arguments; this results in all the child nodes of the
/// `DocumentFragment` being moved to the child list of this node.
///
/// The children of a `DocumentFragment` node are zero or more nodes representing the tops of any
/// sub-trees defining the structure of the document. `DocumentFragment` nodes do not need to be
/// well-formed XML documents (although they do need to follow the rules imposed upon well-formed
/// XML parsed entities, which can have multiple top nodes). For example, a `DocumentFragment`
/// might have only one child and that child node could be a [`Text`](trait.Text.html) node. Such a
/// structure model represents neither an HTML document nor a well-formed XML document.
///
/// When a `DocumentFragment` is inserted into a `Document` (or indeed any other `Node` that may
/// take children) the children of the `DocumentFragment` and not the `DocumentFragment` itself are
/// inserted into the `Node`. This makes the `DocumentFragment` very useful when the user wishes
/// to create nodes that are siblings; the `DocumentFragment` acts as the parent of these nodes
/// so that the user can use the standard methods from the [`Node`](trait.Node.html) interface, such as
/// `insertBefore` and `appendChild`.
///
pub trait DocumentFragment: Node {}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `DocumentType` interface.
///
/// # Specification
///
/// Each [`Document`](trait.Document.html) has a `doctype` attribute whose value is either `null` or
/// a `DocumentType` object. The `DocumentType` interface in the DOM Core provides an interface
/// to the list of entities that are defined for the document, and little else because the effect
/// of namespaces and the various XML schema efforts on DTD representation are not clearly
/// understood as of this writing.
///
/// The DOM Level 2 doesn't support editing `DocumentType` nodes.
///
pub trait DocumentType: Node {
    ///
    /// A `NamedNodeMap` containing the general entities, both external and internal, declared in
    /// the DTD. Parameter entities are not contained. Duplicates are discarded. For example in:
    ///
    /// ```xml
    /// <!DOCTYPE ex SYSTEM "ex.dtd" [
    ///   <!ENTITY foo "foo">
    ///   <!ENTITY bar "bar">
    ///   <!ENTITY bar "bar2">
    ///   <!ENTITY % baz "baz">
    /// ]>
    /// <ex/>
    /// ```
    ///
    /// the interface provides access to `foo` and the first declaration of `bar` but not the
    /// second declaration of `bar` or `baz`. Every node in this map also implements the `Entity`
    /// interface.
    ///
    /// The DOM Level 2 does not support editing entities, therefore `entities` cannot be altered
    /// in any way.
    ///
    fn entities(&self) -> HashMap<Name, Self::NodeRef>;
    ///
    /// A `NamedNodeMap` containing the notations declared in the DTD. Duplicates are discarded.
    /// Every node in this map also implements the `Notation` interface.
    ///
    /// The DOM Level 2 does not support editing notations, therefore `notations` cannot be altered
    /// in any way.
    ///
    fn notations(&self) -> HashMap<Name, Self::NodeRef>;
    /// The public identifier of the external subset.
    fn public_id(&self) -> Option<String>;
    /// The system identifier of the external subset.
    fn system_id(&self) -> Option<String>;
    ///
    /// The internal subset as a string.
    ///
    /// **Note:** The actual content returned depends on how much information is available to the
    /// implementation. This may vary depending on various parameters, including the XML processor
    /// used to build the document.
    ///
    fn internal_subset(&self) -> Option<String>;
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `DOMImplementation` interface.
///
/// The instance used to create a document can be retrieved using the `implementation` method on
/// [`Document`](trait.Document.html). To fetch an implementation to create a document iun the
/// first place use the function [`get_implementation`](fn.get_implementation.html).
///
/// # Specification
///
/// The `DOMImplementation` interface provides a number of methods for performing operations that
/// are independent of any particular instance of the document object model.
///
pub trait DOMImplementation {
    ///
    /// The opaque reference type that wraps the implementation of a node within the DOM.
    ///
    type NodeRef;
    ///
    /// Creates an XML Document object of the specified type with its document element.
    ///
    /// **Note:** This method will panic if it cannot create the document node.
    ///
    /// # Specification
    ///
    /// HTML-only DOM
    /// implementations do not need to implement this method. **Introduced in DOM Level 2**
    ///
    /// **Parameters**
    ///
    /// * `namespaceURI` of type `DOMString`: The namespace URI of the document element to create.
    /// * `qualifiedName` of type `DOMString`: The qualified name of the document element to be created.
    /// * `doctype` of type `DocumentType`: The type of document to be created or null.
    ///   When doctype is not null, its Node.ownerDocument attribute is set to the document being created.
    ///
    /// **Return Value**
    ///
    /// `Document`: A new Document object.
    ///
    /// **Exceptions**
    ///
    /// * `INVALID_CHARACTER_ERR`: Raised if the specified qualified name contains an illegal character.
    /// * `NAMESPACE_ERR`: Raised if the qualifiedName is malformed, if the qualifiedName has a prefix
    ///   and the namespaceURI is null, or if the qualifiedName has a prefix that is "xml" and the
    ///   namespaceURI is different from "http://www.w3.org/XML/1998/namespace".
    /// * `WRONG_DOCUMENT_ERR`: Raised if doctype has already been used with a different document or
    ///   was created from a different implementation.
    ///
    fn create_document(
        &self,
        namespace_uri: &str,
        qualified_name: &str,
        doc_type: Option<Self::NodeRef>,
    ) -> Result<Self::NodeRef>;
    ///
    /// Extension to the standard DOM `create_document` method that takes an options structure to
    /// control the processing of nodes.
    ///
    /// * `options` of type `ProcessingOptions`: the options to be set fot this document.
    ///
    fn create_document_with_options(
        &self,
        namespace_uri: &str,
        qualified_name: &str,
        doc_type: Option<Self::NodeRef>,
        options: ProcessingOptions,
    ) -> Result<Self::NodeRef>;
    ///
    /// Creates an empty `DocumentType` node.
    ///
    /// # Specification
    ///
    /// Entity declarations and notations are not made available. Entity reference expansions and
    /// default attribute additions do not occur. It is expected that a future version of the DOM
    /// will provide a way for populating a `DocumentType`. **Introduced in DOM Level 2**
    ///
    /// HTML-only DOM implementations do not need to implement this method.
    ///
    /// **Parameters**
    ///
    /// * `qualifiedName` of type `DOMString`: The qualified name of the document type to be created.
    /// * `publicId` of type `DOMString`: The external subset public identifier.
    /// * `systemId` of type `DOMString`: The external subset system identifier.
    ///
    /// **Return Value**
    ///
    /// `DocumentType`: A new `DocumentType` node with `Node.ownerDocument` set to null.
    ///
    /// **Exceptions**
    ///
    /// * `INVALID_CHARACTER_ERR`: Raised if the specified qualified name contains an illegal character.
    /// * `NAMESPACE_ERR`: Raised if the `qualifiedName` is malformed.
    ///
    fn create_document_type(
        &self,
        qualified_name: &str,
        public_id: Option<&str>,
        system_id: Option<&str>,
    ) -> Result<Self::NodeRef>;
    ///
    /// Test if the DOM implementation implements a specific feature.
    ///
    /// # Specification
    ///
    /// See DOM Level 2 Core [§1.3. Extended Interfaces](https://www.w3.org/TR/DOM-Level-2-Core/core.html#ID-E067D597)
    ///
    /// **Parameters**
    ///
    /// * `feature` of type `DOMString`: The name of the feature to test (case-insensitive). The values used by DOM features are defined throughout the DOM Level 2 specifications and listed in the Conformance section. The name must be an XML name. To avoid possible conflicts, as a convention, names referring to features defined outside the DOM specification should be made unique by reversing the name of the Internet domain name of the person (or the organization that the person belongs to) who defines the feature, component by component, and using this as a prefix. For instance, the W3C SVG Working Group defines the feature "org.w3c.dom.svg".
    /// * `version` of type `DOMString`: This is the version number of the feature to test. In Level 2, the string can be either "2.0" or "1.0". If the version is not specified, supporting any version of the feature causes the method to return true.
    ///
    /// **Return Value**
    ///
    /// `boolean`: true if the feature is implemented in the specified version, false otherwise.
    ///
    /// **No Exceptions**
    ///
    fn has_feature(&self, feature: &str, version: &str) -> bool;
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `Element` interface.
///
/// # Specification
///
/// The `Element` interface represents an element in an HTML or XML document. Elements may have
/// attributes associated with them; since the `Element` interface inherits from [`Node`](trait.Node.html),
/// the generic `Node` interface attribute attributes may be used to retrieve the set of all
/// attributes for an element. There are methods on the `Element` interface to retrieve either an
/// [`Attr`](trait.Attribute.html) object by name or an attribute value by name. In XML, where an
/// attribute value may contain entity references, an `Attr` object should be retrieved to examine
/// the possibly fairly complex sub-tree representing the attribute value. On the other hand, in
/// HTML, where all attributes have simple string values, methods to directly access an attribute
/// value can safely be used as a convenience.
///
/// **Note:** In DOM Level 2, the method `normalize` is inherited from the `Node` interface where
/// it was moved.
///
pub trait Element: Node {
    ///
    /// The name of the element.
    ///
    /// # Specification
    ///
    /// For example, in:
    ///
    /// ```xml
    /// <elementExample id="demo">
    ///         ...
    /// </elementExample>
    /// ```
    ///
    /// `tagName` has the value "elementExample". Note that this is case-preserving in XML, as
    /// are all of the operations of the DOM. The HTML DOM returns the `tagName` of an HTML element
    /// in the canonical uppercase form, regardless of the case in the source HTML document.
    ///
    fn tag_name(&self) -> String {
        Node::name(self).to_string()
    }
    ///
    /// Retrieves an attribute value by name.
    ///
    /// # Specification
    ///
    /// **Parameters**
    ///
    /// * `name` of type `DOMString`: The name of the attribute to retrieve.
    ///
    /// **Return Value**
    ///
    /// * `DOMString`: The `Attr` value as a string, or the empty string if that attribute does not
    /// have a specified or default value.
    ///
    fn get_attribute(&self, name: &str) -> Option<String>;
    ///
    /// Adds a new attribute.
    ///
    /// # Specification
    ///
    /// If an attribute with that name is already present in the element, its value is changed to
    /// be that of the value parameter. This value is a simple string; it is not parsed as it is
    /// being set. So any markup (such as syntax to be recognized as an entity reference) is
    /// treated as literal text, and needs to be appropriately escaped by the implementation when
    /// it is written out. In order to assign an attribute value that contains entity references,
    /// the user must create an Attr node plus any Text and EntityReference nodes, build the
    /// appropriate subtree, and use setAttributeNode to assign it as the value of an attribute.
    ///
    /// To set an attribute with a qualified name and namespace URI, use the `setAttributeNS` method.
    ///
    /// **Parameters**
    ///
    /// * `name` of type `DOMString`: The name of the attribute to create or alter.
    /// `value` of type `DOMString`: Value to set in string form.
    ///
    /// **Exceptions**
    ///
    /// * `INVALID_CHARACTER_ERR`: Raised if the specified name contains an illegal character.
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node is readonly.
    ///
    fn set_attribute(&mut self, name: &str, value: &str) -> Result<()>;
    ///
    /// Removes an attribute by name. If the removed attribute is known to have a default value, an
    /// attribute immediately appears containing the default value as well as the corresponding
    /// namespace URI, local name, and prefix when applicable.
    ///
    /// # Specification
    ///
    /// To remove an attribute by local name and namespace URI, use the `removeAttributeNS` method.
    ///
    /// **Parameters**
    ///
    /// * `name` of type `DOMString`: The name of the attribute to remove.
    ///
    /// **Exceptions**
    ///
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node is readonly.
    ///
    fn remove_attribute(&mut self, _name: &str) -> Result<()>;
    ///
    /// Retrieves an attribute node by name.
    ///
    /// # Specification
    ///
    /// To retrieve an attribute node by qualified name and namespace URI, use the
    /// `getAttributeNodeNS` method.
    ///
    /// **Parameters**
    ///
    /// * `name` of type `DOMString`: The name (`nodeName`) of the attribute to retrieve.
    ///
    /// **Return Value**
    ///
    /// * `Attr`: The `Attr` node with the specified name (`nodeName`) or null if there is no such
    /// attribute.
    ///
    fn get_attribute_node(&self, name: &str) -> Option<Self::NodeRef>;
    ///
    /// Adds a new attribute node.
    ///
    /// # Specification
    ///
    /// If an attribute with that name (`nodeName`) is already present in the element, it is
    /// replaced by the new one.
    ///
    /// To add a new attribute node with a qualified name and namespace URI, use the
    /// `setAttributeNodeNS` method.
    ///
    /// **Parameters**
    ///
    /// * `newAttr` of type `Attr`: The `Attr` node to add to the attribute list.
    ///
    /// **Return Value**
    ///
    /// * `Attr`: If the `newAttr` attribute replaces an existing attribute, the replaced `Attr`
    ///   node is returned, otherwise `null` is returned.
    ///
    /// **Exceptions**
    ///
    /// * `WRONG_DOCUMENT_ERR`: Raised if `newAttr` was created from a different document than the
    ///   one that created the element.
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node is readonly.
    /// * `INUSE_ATTRIBUTE_ERR`: Raised if `newAttr` is already an attribute of another `Element`
    ///   object. The DOM user must explicitly clone `Attr` nodes to re-use them in other elements.
    ///
    fn set_attribute_node(&mut self, new_attribute: Self::NodeRef) -> Result<Self::NodeRef>;
    ///
    /// Removes the specified attribute node.
    ///
    /// # Specification
    ///
    /// If the removed `Attr` has a default value it is immediately replaced. The replacing
    /// attribute has the same namespace URI and local name, as well as the original prefix, when
    /// applicable.
    ///
    /// **Parameters**
    ///
    /// * `oldAttr` of type `Attr`: The `Attr` node to remove from the attribute list.
    ///
    /// **Return Value**
    ///
    /// * `Attr`: The `Attr` node that was removed.
    ///
    /// **Exceptions**
    ///
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node is readonly.
    /// * `NOT_FOUND_ERR`: Raised if oldAttr is not an attribute of the element.
    ///
    fn remove_attribute_node(&mut self, _old_attribute: Self::NodeRef) -> Result<Self::NodeRef>;
    ///
    /// Returns a `NodeList` of all descendant `Element`s with a given tag name, in the order in
    /// which they are encountered in a preorder traversal of this `Element` tree.
    ///
    /// # Specification
    ///
    /// **Parameters**
    ///
    /// * `name` of type `DOMString`: The name of the tag to match on. The special value "*" matches
    ///   all tags.
    ///
    /// **Return Value**
    ///
    /// * `NodeList`: A list of matching `Element` nodes.
    ///
    fn get_elements_by_tag_name(&self, _tag_name: &str) -> Vec<Self::NodeRef>;
    ///
    /// Retrieves an attribute value by local name and namespace URI.
    ///
    /// # Specification
    ///
    /// HTML-only DOM implementations do not need to implement this method.
    ///
    /// **Parameters**
    ///
    /// * `namespaceURI` of type `DOMString`: The namespace URI of the attribute to retrieve.
    /// * `localName` of type `DOMString`: The local name of the attribute to retrieve.
    ///
    /// **Return Value**
    ///
    /// * `DOMString`: The `Attr` value as a string, or the empty string if that attribute does not
    ///   have a specified or default value.
    ///
    fn get_attribute_ns(&self, _namespace_uri: &str, _local_name: &str) -> Option<String>;
    ///
    /// Adds a new attribute.
    ///
    /// # Specification
    ///
    /// If an attribute with the same local name and namespace URI is already present on the
    /// element, its `prefix` is changed to be the prefix part of the `qualifiedName`, and its
    /// `value` is changed to be the value parameter. This value is a simple string; it is not
    /// parsed as it is being set. So any markup (such as syntax to be recognized as an entity
    /// reference) is treated as literal text, and needs to be appropriately escaped by the
    /// implementation when it is written out. In order to assign an attribute value that contains
    /// entity references, the user must create an `Attr` node plus any `Text` and `EntityReference`
    /// nodes, build the appropriate subtree, and use `setAttributeNodeNS` or `setAttributeNode`
    /// to assign it as the value of an attribute.
    ///
    /// HTML-only DOM implementations do not need to implement this method.
    ///
    /// **Parameters**
    ///
    /// * `namespaceURI` of type `DOMString`: The namespace URI of the attribute to create or alter.
    /// * `qualifiedName` of type `DOMString`: The qualified name of the attribute to create or alter.
    /// * `value` of type `DOMString`: The value to set in string form.
    ///
    /// **Exceptions**
    ///
    /// * `INVALID_CHARACTER_ERR`: Raised if the specified qualified name contains an illegal character.
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node is readonly.
    /// * `NAMESPACE_ERR`: Raised if the `qualifiedName` is malformed, if the `qualifiedName` has a
    ///   prefix and the `namespaceURI` is null, if the `qualifiedName` has a prefix that is "xml"
    ///   and the `namespaceURI` is different from "http://www.w3.org/XML/1998/namespace", or if
    ///   the `qualifiedName` is "xmlns" and the `namespaceURI` is different from
    /// "http://www.w3.org/2000/xmlns/".
    ///
    fn set_attribute_ns(
        &mut self,
        namespace_uri: &str,
        qualified_name: &str,
        value: &str,
    ) -> Result<()>;
    ///
    /// Removes an attribute by local name and namespace URI.
    ///
    /// # Specification
    ///
    /// If the removed attribute has a default value it is immediately replaced. The replacing
    /// attribute has the same namespace URI and local name, as well as the original prefix.
    ///
    /// HTML-only DOM implementations do not need to implement this method.
    ///
    /// **Parameters**
    ///
    /// * `namespaceURI` of type `DOMString`: The namespace URI of the attribute to remove.
    /// * `localName` of type `DOMString`: The local name of the attribute to remove.
    ///
    /// **Exceptions**
    ///
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node is readonly.
    ///
    fn remove_attribute_ns(&mut self, _namespace_uri: &str, _local_name: &str) -> Result<()>;
    ///
    /// Retrieves an Attr node by local name and namespace URI.
    ///
    /// # Specification
    ///
    /// HTML-only DOM implementations do not need to implement this method.
    ///
    /// **Parameters**
    ///
    /// * `namespaceURI` of type `DOMString`: The namespace URI of the attribute to retrieve.
    /// * `localName` of type `DOMString`: The local name of the attribute to retrieve.
    ///
    /// **Return Value**
    ///
    /// * `Attr`: The `Attr` node with the specified attribute local name and namespace URI or
    ///   `null` if there is no such attribute.
    ///
    fn get_attribute_node_ns(
        &self,
        _namespace_uri: &str,
        _local_name: &str,
    ) -> Option<Self::NodeRef>;
    ///
    /// Adds a new attribute.
    ///
    /// # Specification
    ///
    /// If an attribute with that local name and that namespace URI is already present in the
    /// element, it is replaced by the new one.
    ///
    /// HTML-only DOM implementations do not need to implement this method.
    ///
    /// **Parameters**
    ///
    /// * `newAttr` of type `Attr`: The `Attr` node to add to the attribute list.
    ///
    /// **Return Value**
    ///
    /// * `Attr`: If the `newAttr` attribute replaces an existing attribute with the same local name
    ///   and namespace URI, the replaced `Attr` node is returned, otherwise `null` is returned.
    ///
    /// **Exceptions**
    ///
    /// * `WRONG_DOCUMENT_ERR`: Raised if newAttr was created from a different document than the
    ///   one that created the element.
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node is readonly.
    /// * `INUSE_ATTRIBUTE_ERR`: Raised if `newAttr` is already an attribute of another `Element`
    ///   object. The DOM user must explicitly clone `Attr` nodes to re-use them in other elements.
    ///
    fn set_attribute_node_ns(&mut self, _new_attribute: Self::NodeRef) -> Result<Self::NodeRef>;
    ///
    /// Returns a `NodeList` of all the descendant `Element`s with a given local name and namespace
    /// URI in the order in which they are encountered in a preorder traversal of this Element tree.
    ///
    /// # Specification
    ///
    /// HTML-only DOM implementations do not need to implement this method.
    ///
    /// **Parameters**
    ///
    /// * `namespaceURI` of type `DOMString`: The namespace URI of the elements to match on. The
    ///   special value "*" matches all namespaces.
    /// * `localName` of type `DOMString`: The local name of the elements to match on. The special
    ///   value "*" matches all local names.
    ///
    /// **Return Value**
    ///
    /// * `NodeList`: A new `NodeList` object containing all the matched `Element`s.
    ///
    fn get_elements_by_tag_name_ns(
        &self,
        _namespace_uri: &str,
        _local_name: &str,
    ) -> Vec<Self::NodeRef>;
    ///
    /// Returns `true` when an attribute with a given name is specified on this element or has a default
    /// value, `false` otherwise.
    ///
    /// # Specification
    ///
    /// **Parameters**
    ///
    /// * `name` of type `DOMString`: The name of the attribute to look for.
    ///
    /// **Return Value**
    ///
    /// * `boolean`: `true` if an attribute with the given name is specified on this element or
    ///   has a default value, `false` otherwise.
    ///
    fn has_attribute(&self, name: &str) -> bool;
    ///
    /// Returns `true` when an attribute with a given local name and namespace URI is specified on
    /// this element or has a default value, `false` otherwise.
    ///
    /// # Specification
    ///
    /// HTML-only DOM implementations do not need to implement this method.
    ///
    /// **Parameters**
    ///
    /// * `namespaceURI` of type `DOMString`: The namespace URI of the attribute to look for.
    /// `localName` of type `DOMString`: The local name of the attribute to look for.
    ///
    /// **Return Value**
    ///
    /// * `boolean`: `true` if an attribute with the given local name and namespace URI is
    ///   specified or has a default value on this element, `false` otherwise.
    ///
    fn has_attribute_ns(&self, namespace_uri: &str, local_name: &str) -> bool;
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `Entity` interface (currently unsupported).
///
/// # Specification
///
/// This interface represents an entity, either parsed or unparsed, in an XML document. Note that
/// this models the entity itself not the entity declaration. Entity declaration modeling has
/// been left for a later Level of the DOM specification.
///
/// The nodeName attribute that is inherited from Node contains the name of the entity.
///
/// An XML processor may choose to completely expand entities before the structure model is passed
/// to the DOM; in this case there will be no `EntityReference` nodes in the document tree.
///
/// XML does not mandate that a non-validating XML processor read and process entity declarations
/// made in the external subset or declared in external parameter entities. This means that parsed
/// entities declared in the external subset need not be expanded by some classes of applications,
/// and that the replacement value of the entity may not be available. When the replacement value
/// is available, the corresponding `Entity` node's child list represents the structure of that
/// replacement text. Otherwise, the child list is empty.
///
/// The DOM Level 2 does not support editing `Entity` nodes; if a user wants to make changes to
/// the contents of an `Entity`, every related `EntityReference` node has to be replaced in the
/// structure model by a clone of the `Entity`'s contents, and then the desired changes must be
/// made to each of those clones instead. `Entity` nodes and all their descendants are readonly.
///
/// An `Entity` node does not have any parent.
///
/// **Note:** If the entity contains an unbound namespace prefix, the` namespaceURI` of the
/// corresponding node in the `Entity` node subtree is `null`. The same is true for
/// `EntityReference` nodes that refer to this entity, when they are created using the
/// `createEntityReference` method of the [`Document`](trait.Document.html) interface. The DOM
/// Level 2 does not support any mechanism to resolve namespace prefixes.
///
pub trait Entity: Node {
    ///
    /// The public identifier associated with the entity, if specified.
    ///
    /// # Specification
    ///
    /// If the public identifier was not specified, this is `null`.
    ///
    fn public_id(&self) -> Option<String>;
    ///
    /// The system identifier associated with the entity, if specified.
    ///
    /// # Specification
    ///
    /// If the system identifier was not specified, this is `null`.
    ///
    fn system_id(&self) -> Option<String>;
    ///
    /// For unparsed entities, the name of the notation for the entity.
    ///
    ///# Specification
    ///
    /// For parsed entities, this is `null`.
    ///
    fn notation_name(&self) -> Option<String>;
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `EntityReference` interface (currently unsupported).
///
/// # Specification
///
/// `EntityReference` objects may be inserted into the structure model when an entity reference
/// is in the source document, or when the user wishes to insert an entity reference. Note that
/// character references and references to predefined entities are considered to be expanded by
/// the HTML or XML processor so that characters are represented by their Unicode equivalent rather
/// than by an entity reference. Moreover, the XML processor may completely expand references to
/// entities while building the structure model, instead of providing `EntityReference` objects. If
/// it does provide such objects, then for a given `EntityReference` node, it may be that there is
/// no `Entity` node representing the referenced entity. If such an `Entity` exists, then the
/// subtree of the `EntityReference` node is in general a copy of the `Entity` node subtree.
/// However, this may not be true when an entity contains an unbound namespace prefix. In such a
/// case, because the namespace prefix resolution depends on where the entity reference is, the
/// descendants of the `EntityReference` node may be bound to different namespace URIs.
///
/// As for `Entity` nodes, `EntityReference` nodes and all their descendants are readonly.
///
pub trait EntityReference: Node {}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `Node` interface.
///
/// # Specification
///
/// The `Node` interface is the primary datatype for the entire Document Object Model. It represents
/// a single node in the document tree. While all objects implementing the `Node` interface expose
/// methods for dealing with children, not all objects implementing the `Node` interface may have
/// children. For example, `Text` nodes may not have children, and adding children to such nodes
/// results in a `DOMException` being raised.
///
/// The attributes `nodeName`, `nodeValue` and `attributes` are included as a mechanism to get at node
/// information without casting down to the specific derived interface. In cases where there is no
/// obvious mapping of these attributes for a specific nodeType (e.g., `nodeValue` for an `Element` or
/// `attributes` for a `Comment`), this returns `null`. Note that the specialized interfaces may contain
/// additional and more convenient mechanisms to get and set the relevant information.
///
/// The values of `nodeName`, `nodeValue`, and `attributes` vary according to the node type as follows:
///
///
/// | Interface               | nodeName                  | nodeValue                           | attributes   |
/// |-------------------------|---------------------------|-------------------------------------|--------------|
/// | `Attr`                  | name of attribute         | value of attribute                  | `None`       |
/// | `CDATASection`          | `"#cdata-section"`        | content of the CDATA Section        | `None`       |
/// | `Comment`               | `"#comment"`              | content of the comment              | `None`       |
/// | `Document`              | `"#document"`             | `None`                              | `None`       |
/// | `DocumentFragment`      | `"#document-fragment"`    | `None`                              | `None`       |
/// | `DocumentType`          | document type name        | `None`                              | `None`       |
/// | `Element`               | tag name                  | `None`                              | `HashMap`    |
/// | `Entity`                | entity name               | `None`                              | `None`       |
/// | `EntityReference`       | name of entity referenced | `None`                              | `None`       |
/// | `Notation`              | notation name             | `None`                              | `None`       |
/// | `ProcessingInstruction` | `target`                  | entire content excluding the target | `None`       |
/// | `Text`                  | `"#text"`                 | content of the text node            | `None`       |
///
pub trait Node {
    ///
    /// The opaque reference type that wraps the implementation of a node within the DOM.
    ///
    type NodeRef;
    ///
    /// The name of this node, depending on its type; see the table above.
    ///
    fn name(&self) -> Name;
    ///
    /// The value of this node, depending on its type; see the table above. When it is defined to
    /// be `None`, setting it has no effect.
    ///
    /// # Specification
    ///
    /// **Exceptions on setting**
    ///
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised when the node is readonly.
    ///
    /// **Exceptions on retrieval**
    ///
    /// * `DOMSTRING_SIZE_ERR`: Raised when it would return more characters than fit in a DOMString
    /// variable on the implementation platform.
    ///
    fn node_value(&self) -> Option<String>;
    ///
    /// Set the `value` for the node; see [`node_value`](#tymethod.node_value).
    ///
    fn set_node_value(&mut self, value: &str) -> Result<()>;
    ///
    /// Set the `value` for the node to `None`; see [`node_value`](#tymethod.node_value).
    ///
    fn unset_node_value(&mut self) -> Result<()>;
    ///
    /// A code representing the type of the underlying object.
    ///
    fn node_type(&self) -> NodeType;
    ///
    /// The parent of this node. All nodes, except `Attr`, `Document`, `DocumentFragment`,
    /// `Entity`, and `Notation` may have a parent. However, if a node has just been created and not
    /// yet added to the tree, or if it has been removed from the tree, this is `None`.
    ///
    fn parent_node(&self) -> Option<Self::NodeRef>;
    ///
    /// A `Vec` that contains all children of this node. If there are no children,
    /// this is a `Vec` containing no nodes.
    ///
    fn child_nodes(&self) -> Vec<Self::NodeRef>;
    ///
    /// The first child of this node. If there is no such node, this returns `None`.
    ///
    fn first_child(&self) -> Option<Self::NodeRef>;
    ///
    /// The last child of this node. If there is no such node, this returns `None`.
    ///
    fn last_child(&self) -> Option<Self::NodeRef>;
    ///
    /// The node immediately preceding this node. If there is no such node, this returns `None`.
    ///
    fn previous_sibling(&self) -> Option<Self::NodeRef>;
    ///
    /// The node immediately following this node. If there is no such node, this returns `None`.
    ///
    fn next_sibling(&self) -> Option<Self::NodeRef>;
    ///
    /// A `HashMap` containing the attributes of this node (if it is an `Element`) or
    /// `None` otherwise.
    ///
    fn attributes(&self) -> HashMap<Name, Self::NodeRef>;
    ///
    /// The `Document` object associated with this node. This is also the `Document`
    /// object used to create new nodes. When this node is a `Document` or a `DocumentType` which is
    /// not used with any `Document` yet, this is `None`.
    ///
    fn owner_document(&self) -> Option<Self::NodeRef>;
    ///
    /// Inserts the node `newChild` before the existing child node `refChild`.
    ///
    /// # Specification
    ///
    /// If `refChild` is `null`, insert `newChild` at the end of the list of children.
    ///
    /// If `newChild` is a `DocumentFragment` object, all of its children are inserted, in the
    /// same order, before `refChild`. If the `newChild` is already in the tree, it is first removed.
    ///
    /// **Parameters**
    ///
    /// * `newChild` of type `Node`: The node to insert.
    /// * `refChild `of type `Node`: The reference node, i.e., the node before which the new node
    ///   must be inserted.
    ///
    /// **Return Value**
    ///
    /// * `Node`: The node being inserted.
    ///
    /// **Exceptions**
    ///
    /// * `HIERARCHY_REQUEST_ERR`: Raised if this node is of a type that does not allow children
    ///   of the type of the `newChild` node, or if the node to insert is one of this node's
    ///   ancestors.
    /// * `WRONG_DOCUMENT_ERR`: Raised if `newChild` was created from a different document than the
    ///   one that created this node.
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node is readonly or if the parent of the
    ///   node being inserted is readonly.
    /// * `NOT_FOUND_ERR`: Raised if `refChild` is not a child of this node.
    ///
    fn insert_before(
        &mut self,
        new_child: Self::NodeRef,
        ref_child: Option<Self::NodeRef>,
    ) -> Result<Self::NodeRef>;
    ///
    /// Replaces the child node `oldChild` with `newChild` in the list of children, and returns the
    /// `oldChild` node.
    ///
    /// # Specification
    ///
    /// If `newChild` is a `DocumentFragment` object, `oldChild` is replaced by all of the
    /// `DocumentFragment` children, which are inserted in the same order. If the `newChild` is
    /// already in the tree, it is first removed.
    ///
    /// **Parameters**
    ///
    /// * `newChild` of type `Node`: The new node to put in the child list.
    /// * `oldChild` of type `Node`: The node being replaced in the list.
    ///
    /// **Return Value**
    ///
    /// * `Node`: The node replaced.
    ///
    /// **Exceptions**
    ///
    /// * `HIERARCHY_REQUEST_ERR`: Raised if this node is of a type that does not allow children of
    ///   the type of the newChild node, or if the node to put in is one of this node's ancestors.
    /// * `WRONG_DOCUMENT_ERR`: Raised if `newChild` was created from a different document than the
    ///   one that created this node.
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node or the parent of the new node is readonly.
    /// * `NOT_FOUND_ERR`: Raised if oldChild is not a child of this node.
    ///
    fn replace_child(
        &mut self,
        new_child: Self::NodeRef,
        old_child: &Self::NodeRef,
    ) -> Result<Self::NodeRef>;
    ///
    /// Removes the child node indicated by oldChild from the list of children, and returns it.
    ///
    /// # Specification
    ///
    /// **Parameters**
    ///
    /// * `oldChild` of type `Node`: The node being removed.
    ///
    /// **Return Value**
    ///
    /// * `Node`: The node removed.
    ///
    /// **Exceptions**
    ///
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node is readonly.
    /// * `NOT_FOUND_ERR`: Raised if oldChild is not a child of this node.
    ///
    fn remove_child(&mut self, old_child: Self::NodeRef) -> Result<Self::NodeRef>;
    ///
    /// Adds the node `newChild` to the end of the list of children of this node.
    ///
    /// # Specification
    ///
    /// If the `newChild` is already in the tree, it is first removed.
    ///
    /// **Parameters**
    ///
    /// * `newChild` of type `Node`: The node to add. If it is a `DocumentFragment` object, the
    ///   entire contents of the document fragment are moved into the child list of this node.
    ///
    /// **Return Value**
    ///
    /// `Node`: The node added.
    ///
    /// **Exceptions**
    ///
    /// * `HIERARCHY_REQUEST_ERR: Raised if this node is of a type that does not allow children of`
    ///   the type of the `newChild` node, or if the node to append is one of this node's ancestors.
    /// * `WRONG_DOCUMENT_ERR`: Raised if `newChild` was created from a different document than
    ///   the one that created this node.
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node is readonly.
    ///
    fn append_child(&mut self, new_child: Self::NodeRef) -> Result<Self::NodeRef>;
    ///
    /// Returns whether this node has any children.
    ///
    /// # Specification
    ///
    /// **Return Value**
    ///
    /// * `boolean`: `true` if this node has any children, `false` otherwise.
    ///
    fn has_child_nodes(&self) -> bool;
    ///
    /// Returns a duplicate of this node, i.e., serves as a generic copy constructor for nodes.
    ///
    /// **Note:** currently unsupported.
    ///
    /// # Specification
    ///
    /// The duplicate node has no parent; (`parentNode` is null.).
    ///
    /// Cloning an [`Element`](trait.Element.html) copies all attributes and their values, including
    /// those generated by the XML processor to represent defaulted attributes, but this method does
    /// not copy any text it contains unless it is a deep clone, since the text is contained in a
    /// child [`Text`](trait.Text.html) node. Cloning an [`Attribute`](trait.Attribute.html) directly,
    /// as opposed to be cloned as part of an `Element` cloning operation, returns a specified
    /// attribute (`specified` is `true`). Cloning any other type of node simply returns a copy of
    /// this node.
    ///
    /// Note that cloning an immutable subtree results in a mutable copy, but the children of an
    /// `EntityReference` clone are readonly. In addition, clones of unspecified `Attr` nodes are
    /// specified. And, cloning `Document`, `DocumentType`, `Entity`, and `Notation` nodes is
    /// implementation dependent.
    ///
    /// **Parameters**
    ///
    /// * `deep` of type `boolean`: If `true`, recursively clone the subtree under the specified
    ///   node; if `false`, clone only the node itself (and its attributes, if it is an `Element`).
    ///
    /// **Return Value**
    ///
    /// * `Node`:The duplicate node.
    ///
    fn clone_node(&self, deep: bool) -> Option<Self::NodeRef>;
    ///
    /// Puts all [`Text`](trait.Text.html) nodes in the full depth of the sub-tree underneath this
    /// `Node`, including attribute nodes, into a "normal" form where only structure (e.g.,
    /// elements, comments, processing instructions, CDATA sections, and entity references)
    /// separates `Text` nodes, i.e., there are neither adjacent `Text` nodes nor empty `Text` nodes.
    ///
    /// # Specification
    ///
    /// This can be used to ensure that the DOM view of a document is the same as if it were saved
    /// and re-loaded, and is useful when operations (such as XPointer lookups) that depend on a
    /// particular document tree structure are to be used.
    ///
    /// Note: In cases where the document contains [`CDataSection`](trait.CDataSection.html), the
    /// normalize operation alone may not be sufficient, since XPointers do not differentiate
    /// between `Text` nodes and `CDATASection` nodes.
    ///
    fn normalize(&mut self);
    ///
    /// Tests whether the DOM implementation implements a specific feature and that feature is
    /// supported by this node.
    ///
    /// **Note:** currently unsupported.
    ///
    /// # Specification
    ///
    /// **Parameters**
    ///
    /// * `feature` of type `DOMString`: The name of the feature to test. This is the same name
    ///   which can be passed to the method `hasFeature` on `DOMImplementation`.
    /// * `version` of type `DOMString`: This is the version number of the feature to test. In
    ///   Level 2, version 1, this is the string "2.0". If the version is not specified, supporting
    ///   any version of the feature will cause the method to return true.
    ///
    /// **Return Value**
    ///
    /// * `boolean`: Returns `true` if the specified feature is supported on this node, `false`
    ///   otherwise.
    ///
    fn is_supported(&self, feature: &str, version: &str) -> bool;
    ///
    /// Returns whether this node (if it is an element) has any attributes.
    ///
    /// **Return Value**
    ///
    /// * `boolean`: `true` if this node has any attributes, `false` otherwise.
    ///
    fn has_attributes(&self) -> bool;
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `Notation` interface (currently unsupported).
///
/// # Specification
///
/// This interface represents a notation declared in the DTD. A notation either declares, by name,
/// the format of an unparsed entity (see section 4.7 of the XML 1.0 specification), or is used
/// for formal declaration of processing instruction targets (see section 2.6 of the XML 1.0
/// specification). The `nodeName` attribute inherited from [`Node`](trait.Node.html) is set to the
/// declared name of the notation.
///
/// The DOM Level 1 does not support editing `Notation` nodes; they are therefore readonly.
///
/// A `Notation` node does not have any parent.
///
pub trait Notation: Node {
    ///
    /// The public identifier of this notation.
    ///
    /// # Specification
    ///
    /// If the public identifier was not specified, this is `null`.
    ///
    fn public_id(&self) -> Option<String>;
    ///
    /// The system identifier of this notation.
    ///
    /// # Specification
    ///
    /// If the system identifier was not specified, this is `null`.
    ///
    fn system_id(&self) -> Option<String>;
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `ProcessingInstruction` interface.
///
/// # Specification
///
/// The `ProcessingInstruction` interface represents a "processing instruction", used in XML as a
/// way to keep processor-specific information in the text of the document.
///
pub trait ProcessingInstruction: Node {
    ///
    /// The number of 16-bit units that are available through `data`.
    ///
    fn length(&self) -> usize {
        match self.data() {
            None => 0,
            Some(s) => s.len(),
        }
    }
    ///
    /// The content of this processing instruction.
    ///
    /// # Specification
    ///
    /// This is from the first non white space character after the target to the character
    /// immediately preceding the `'?>'`.
    ///
    /// **Exceptions on setting**
    ///
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised when the node is readonly.
    ///
    fn data(&self) -> Option<String> {
        Node::node_value(self)
    }
    ///
    /// Set the `data` for the node; see [`data`](#tymethod.data).
    ///
    fn set_data(&mut self, data: &str) -> Result<()> {
        Node::set_node_value(self, data)
    }
    ///
    /// Set the `data` for the node to `None`; see [`data`](#tymethod.data).
    ///
    fn unset_data(&mut self) -> Result<()> {
        Node::unset_node_value(self)
    }
    ///
    /// The target of this processing instruction.
    ///
    /// # Specification
    ///
    /// XML defines this as being the first token following the markup that begins the processing
    /// instruction.
    ///
    fn target(&self) -> String {
        Node::name(self).to_string()
    }
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `Text` interface.
///
/// # Specification
///
/// The `Text` interface inherits from [`CharacterData`](trait.CharacterData.html) and represents the
/// textual content (termed character data in XML) of an [`Element`](trait.Element.html) or
/// [`Attr`](trait.Attribute.html). If there is no markup inside an element's content, the text is
/// contained in a single object implementing the `Text` interface that is the only child of the
/// element. If there is markup, it is parsed into the information items (elements, comments,
/// etc.) and `Text` nodes that form the list of children of the element.
///
/// When a document is first made available via the DOM, there is only one `Text` node for each
/// block of text. Users may create adjacent `Text` nodes that represent the contents of a given
/// element without any intervening markup, but should be aware that there is no way to represent
/// the separations between these nodes in XML or HTML, so they will not (in general) persist
/// between DOM editing sessions. The `normalize()` method on [`Node`](trait.Node.html) merges any
/// such adjacent `Text` objects into a single node for each block of text.
///
pub trait Text: CharacterData {
    ///
    /// Breaks this node into two nodes at the specified offset, keeping both in the tree as siblings.
    ///
    /// # Specification
    ///
    /// After being split, this node will contain all the content up to the offset point. A new
    /// node of the same type, which contains all the content at and after the offset point, is
    /// returned. If the original node had a parent node, the new node is inserted as the next
    /// sibling of the original node. When the offset is equal to the length of this node, the
    /// new node has no data.
    ///
    /// **Parameters**
    ///
    /// * `offset` of type `unsigned long`: The 16-bit unit offset at which to split, starting from 0.
    ///
    /// **Return Value**
    ///
    /// * `Text`: The new node, of the same type as this node.
    ///
    /// **Exceptions**
    ///
    /// * `INDEX_SIZE_ERR`: Raised if the specified offset is negative or greater than the number
    ///   of 16-bit units in data.
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node is readonly.
    ///
    fn split(&mut self, offset: usize) -> Result<Self::NodeRef>;
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `NodeType` set of constants.
///
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum NodeType {
    /// The node is an [`Element`](trait.Element.html)
    Element = 1,
    /// The node is an [`Attribute`](trait.Attribute.html)
    Attribute,
    /// The node is a [`Text`](trait.Text.html)
    Text,
    /// The node is a [`CDataSection`](trait.CDataSection.html)
    CData,
    /// The node is an `EntityReference`
    EntityReference,
    /// The node is an `Entity`
    Entity,
    /// The node is a [`ProcessingInstruction`](trait.ProcessingInstruction.html)
    ProcessingInstruction,
    /// The node is a [`Comment`](trait.Comment.html)
    Comment,
    /// The node is a [`Document`](trait.Document.html)
    Document,
    /// The node is a [`DocumentType`](trait.DocumentType.html)
    DocumentType,
    /// The node is a `DocumentFragment`
    DocumentFragment,
    /// The node is a `Notation`
    Notation,
}
