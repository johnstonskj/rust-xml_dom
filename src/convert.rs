/*!
Provides safe `RefNode` conversion functions.

Note that all of the `as_{name}` functions work as follows.

* If the `node_type` corresponds to the correct type, it returns OK.
* If the `node_type` does not correspond to the correct type, it returns `Error::InvalidState`.
* If the `node_type` is not implemented it returns `Error::NotSupported`.

*/
use crate::error::{Error, Result};
use crate::namespaced::{MutNamespaced, Namespaced};
use crate::node_impl::*;
use crate::traits::*;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Type for dynamic trait cast
pub type RefAttribute<'a> = &'a dyn Attribute<NodeRef = RefNode>;
/// Type for mutable dynamic trait cast
pub type MutRefAttribute<'a> = &'a mut dyn Attribute<NodeRef = RefNode>;

/// Type for dynamic trait cast
pub type RefElement<'a> = &'a dyn Element<NodeRef = RefNode>;
/// Type for mutable dynamic trait cast
pub type MutRefElement<'a> = &'a mut dyn Element<NodeRef = RefNode>;
/// Type for dynamic trait cast
pub type RefNamespaced<'a> = &'a dyn Namespaced<NodeRef = RefNode>;
pub(crate) type MutRefNamespaced<'a> = &'a mut dyn MutNamespaced<NodeRef = RefNode>;

/// Type for dynamic trait cast
pub type RefCharacterData<'a> = &'a dyn CharacterData<NodeRef = RefNode>;
/// Type for mutable dynamic trait cast
pub type MutRefCharacterData<'a> = &'a mut dyn CharacterData<NodeRef = RefNode>;

/// Type for dynamic trait cast
pub type RefText<'a> = &'a dyn Text<NodeRef = RefNode>;
/// Type for mutable dynamic trait cast
pub type MutRefText<'a> = &'a mut dyn Text<NodeRef = RefNode>;

/// Type for dynamic trait cast
pub type RefCDataSection<'a> = &'a dyn CDataSection<NodeRef = RefNode>;
/// Type for mutable dynamic trait cast
pub type MutRefCDataSection<'a> = &'a mut dyn CDataSection<NodeRef = RefNode>;

/// Type for dynamic trait cast
pub type RefEntityReference<'a> = &'a dyn EntityReference<NodeRef = RefNode>;
/// Type for mutable dynamic trait cast
pub type MutRefEntityReference<'a> = &'a mut dyn EntityReference<NodeRef = RefNode>;

/// Type for dynamic trait cast
pub type RefEntity<'a> = &'a dyn Entity<NodeRef = RefNode>;
/// Type for mutable dynamic trait cast
pub type MutRefEntity<'a> = &'a mut dyn Entity<NodeRef = RefNode>;

/// Type for dynamic trait cast
pub type RefProcessingInstruction<'a> = &'a dyn ProcessingInstruction<NodeRef = RefNode>;
/// Type for mutable dynamic trait cast
pub type MutRefProcessingInstruction<'a> = &'a mut dyn ProcessingInstruction<NodeRef = RefNode>;

/// Type for dynamic trait cast
pub type RefComment<'a> = &'a dyn Comment<NodeRef = RefNode>;
/// Type for mutable dynamic trait cast
pub type MutRefComment<'a> = &'a mut dyn Comment<NodeRef = RefNode>;

/// Type for dynamic trait cast
pub type RefDocument<'a> = &'a dyn Document<NodeRef = RefNode>;
/// Type for mutable dynamic trait cast
pub type MutRefDocument<'a> = &'a mut dyn Document<NodeRef = RefNode>;

/// Type for dynamic trait cast
pub type RefDocumentType<'a> = &'a dyn DocumentType<NodeRef = RefNode>;
/// Type for mutable dynamic trait cast
pub type MutRefDocumentType<'a> = &'a mut dyn DocumentType<NodeRef = RefNode>;

/// Type for dynamic trait cast
pub type RefDocumentFragment<'a> = &'a dyn DocumentFragment<NodeRef = RefNode>;
/// Type for mutable dynamic trait cast
pub type MutRefDocumentFragment<'a> = &'a mut dyn DocumentFragment<NodeRef = RefNode>;

/// Type for dynamic trait cast
pub type RefNotation<'a> = &'a dyn Notation<NodeRef = RefNode>;
/// Type for mutable dynamic trait cast
pub type MutRefNotation<'a> = &'a mut dyn Notation<NodeRef = RefNode>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Determines if the specified node is of type `NodeType::Attribute`.
///
#[inline]
pub fn is_attribute(ref_node: &RefNode) -> bool {
    ref_node.borrow().i_node_type == NodeType::Attribute
}

///
/// Safely _cast_ the specified `RefNode` into a  `Attribute`.
///
#[inline]
pub fn as_attribute(ref_node: &RefNode) -> Result<RefAttribute<'_>> {
    if ref_node.borrow().i_node_type == NodeType::Attribute {
        Ok(ref_node as RefAttribute<'_>)
    } else {
        warn!("ref_node.node_type != Attribute");
        Err(Error::InvalidState)
    }
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `Attribute`.
///
#[inline]
pub fn as_attribute_mut(ref_node: &mut RefNode) -> Result<MutRefAttribute<'_>> {
    if ref_node.borrow().i_node_type == NodeType::Attribute {
        Ok(ref_node as MutRefAttribute<'_>)
    } else {
        warn!("ref_node.node_type != Attribute");
        Err(Error::InvalidState)
    }
}

///
/// Determines if the specified node is of type `NodeType::Element`.
///
#[inline]
pub fn is_element(ref_node: &RefNode) -> bool {
    ref_node.borrow().i_node_type == NodeType::Element
}

///
/// Safely _cast_ the specified `RefNode` into an  `Element`.
///
#[inline]
pub fn as_element(ref_node: &RefNode) -> Result<RefElement<'_>> {
    if ref_node.borrow().i_node_type == NodeType::Element {
        Ok(ref_node as RefElement<'_>)
    } else {
        warn!("ref_node.node_type != Element");
        Err(Error::InvalidState)
    }
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `Element`.
///
#[inline]
pub fn as_element_mut(ref_node: &mut RefNode) -> Result<MutRefElement<'_>> {
    if ref_node.borrow().i_node_type == NodeType::Element {
        Ok(ref_node as MutRefElement<'_>)
    } else {
        warn!("ref_node.node_type != Element");
        Err(Error::InvalidState)
    }
}

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
        warn!("ref_node.node_type != Element");
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
        warn!("ref_node.node_type != Element");
        Err(Error::InvalidState)
    }
}

///
/// Determines if the specified node is a type of `CharacterData`.
///
#[inline]
pub fn is_character_data(ref_node: &RefNode) -> bool {
    match ref_node.borrow().i_node_type {
        NodeType::CData | NodeType::Comment | NodeType::Text => true,
        _ => false,
    }
}

///
/// Safely _cast_ the specified `RefNode` into a  `Text`.
///
#[inline]
pub fn as_character_data(ref_node: &RefNode) -> Result<RefCharacterData<'_>> {
    match ref_node.borrow().i_node_type {
        NodeType::CData | NodeType::Comment | NodeType::Text => {
            Ok(ref_node as RefCharacterData<'_>)
        }
        _ => {
            warn!("ref_node.node_type != CharacterData");
            Err(Error::InvalidState)
        }
    }
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `Text`.
///
#[inline]
pub fn as_character_data_mut(ref_node: &mut RefNode) -> Result<MutRefCharacterData<'_>> {
    let node_type = { &ref_node.borrow().i_node_type.clone() };
    match node_type {
        NodeType::CData | NodeType::Comment | NodeType::Text => {
            Ok(ref_node as MutRefCharacterData<'_>)
        }
        _ => {
            warn!("ref_node.node_type != CharacterData");
            Err(Error::InvalidState)
        }
    }
}

///
/// Determines if the specified node is of type `NodeType::Text`.
///
#[inline]
pub fn is_text(ref_node: &RefNode) -> bool {
    ref_node.borrow().i_node_type == NodeType::Text
}

///
/// Safely _cast_ the specified `RefNode` into a  `Text`.
///
#[inline]
pub fn as_text(ref_node: &RefNode) -> Result<RefText<'_>> {
    if ref_node.borrow().i_node_type == NodeType::Text {
        Ok(ref_node as RefText<'_>)
    } else {
        warn!("ref_node.node_type != Text");
        Err(Error::InvalidState)
    }
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `Text`.
///
#[inline]
pub fn as_text_mut(ref_node: &mut RefNode) -> Result<MutRefText<'_>> {
    if ref_node.borrow().i_node_type == NodeType::Text {
        Ok(ref_node as MutRefText<'_>)
    } else {
        warn!("ref_node.node_type != Text");
        Err(Error::InvalidState)
    }
}

///
/// Determines if the specified node is of type `NodeType::CDataSection`.
///
#[inline]
pub fn is_cdata_section(ref_node: &RefNode) -> bool {
    ref_node.borrow().i_node_type == NodeType::CData
}

///
/// Safely _cast_ the specified `RefNode` into a  `CDataSection`.
///
#[inline]
pub fn as_cdata_section(ref_node: &RefNode) -> Result<RefCDataSection<'_>> {
    if ref_node.borrow().i_node_type == NodeType::CData {
        Ok(ref_node as RefCDataSection<'_>)
    } else {
        warn!("ref_node.node_type != CData");
        Err(Error::InvalidState)
    }
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `CDataSection`.
///
#[inline]
pub fn as_cdata_section_mut(ref_node: &mut RefNode) -> Result<MutRefCDataSection<'_>> {
    if ref_node.borrow().i_node_type == NodeType::CData {
        Ok(ref_node as MutRefCDataSection<'_>)
    } else {
        warn!("ref_node.node_type != CData");
        Err(Error::InvalidState)
    }
}

///
/// Determines if the specified node is of type `NodeType::EntityReference`.
///
#[inline]
pub fn is_entity_reference(_ref_node: &RefNode) -> bool {
    panic!("node type EntityReference unsupported");
}

///
/// Safely _cast_ the specified `RefNode` into a  `EntityReference`.
///
#[inline]
pub fn as_entity_reference(_ref_node: &RefNode) -> Result<RefEntityReference<'_>> {
    warn!("node type EntityReference unsupported");
    Err(Error::NotSupported)
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `EntityReference`.
///
#[inline]
pub fn as_entity_reference_mut(_ref_node: &mut RefNode) -> Result<MutRefEntityReference<'_>> {
    warn!("node type EntityReference unsupported");
    Err(Error::NotSupported)
}

///
/// Determines if the specified node is of type `NodeType::Entity`.
///
#[inline]
pub fn is_entity(_ref_node: &RefNode) -> bool {
    panic!("node type Entity unsupported");
}

///
/// Safely _cast_ the specified `RefNode` into a  `Entity`.
///
#[inline]
pub fn as_entity(_ref_node: &RefNode) -> Result<RefEntity<'_>> {
    warn!("node type Entity unsupported");
    Err(Error::NotSupported)
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `Entity`.
///
#[inline]
pub fn as_entity_mut(_ref_node: &mut RefNode) -> Result<MutRefEntity<'_>> {
    warn!("node type Entity unsupported");
    Err(Error::NotSupported)
}

///
/// Determines if the specified node is of type `NodeType::ProcessingInstruction`.
///
#[inline]
pub fn is_processing_instruction(ref_node: &RefNode) -> bool {
    ref_node.borrow().i_node_type == NodeType::ProcessingInstruction
}

///
/// Safely _cast_ the specified `RefNode` into a  `ProcessingInstruction`.
///
#[inline]
pub fn as_processing_instruction(ref_node: &RefNode) -> Result<RefProcessingInstruction<'_>> {
    if ref_node.borrow().i_node_type == NodeType::ProcessingInstruction {
        Ok(ref_node as RefProcessingInstruction<'_>)
    } else {
        warn!("ref_node.node_type != ProcessingInstruction");
        Err(Error::InvalidState)
    }
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `ProcessingInstruction`.
///
#[inline]
pub fn as_processing_instruction_mut(
    ref_node: &mut RefNode,
) -> Result<MutRefProcessingInstruction<'_>> {
    if ref_node.borrow().i_node_type == NodeType::ProcessingInstruction {
        Ok(ref_node as MutRefProcessingInstruction<'_>)
    } else {
        warn!("ref_node.node_type != ProcessingInstruction");
        Err(Error::InvalidState)
    }
}

///
/// Determines if the specified node is of type `NodeType::Comment`.
///
#[inline]
pub fn is_comment(ref_node: &RefNode) -> bool {
    ref_node.borrow().i_node_type == NodeType::Comment
}

///
/// Safely _cast_ the specified `RefNode` into a  `Comment`.
///
#[inline]
pub fn as_comment(ref_node: &RefNode) -> Result<RefComment<'_>> {
    if ref_node.borrow().i_node_type == NodeType::Comment {
        Ok(ref_node as RefComment<'_>)
    } else {
        warn!("ref_node.node_type != Comment");
        Err(Error::InvalidState)
    }
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `Comment`.
///
#[inline]
pub fn as_comment_mut(ref_node: &mut RefNode) -> Result<MutRefComment<'_>> {
    if ref_node.borrow().i_node_type == NodeType::Comment {
        Ok(ref_node as MutRefComment<'_>)
    } else {
        warn!("ref_node.node_type != Comment");
        Err(Error::InvalidState)
    }
}

///
/// Determines if the specified node is of type `NodeType::Attribute`.
///
#[inline]
pub fn is_document(ref_node: &RefNode) -> bool {
    ref_node.borrow().i_node_type == NodeType::Document
}

///
/// Safely _cast_ the specified `RefNode` into a  `Document`.
///
#[inline]
pub fn as_document(ref_node: &RefNode) -> Result<RefDocument<'_>> {
    if ref_node.borrow().i_node_type == NodeType::Document {
        Ok(ref_node as RefDocument<'_>)
    } else {
        warn!("ref_node.node_type != Attribute");
        Err(Error::InvalidState)
    }
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `Document`.
///
#[inline]
pub fn as_document_mut(ref_node: &mut RefNode) -> Result<MutRefDocument<'_>> {
    if ref_node.borrow().i_node_type == NodeType::Document {
        Ok(ref_node as MutRefDocument<'_>)
    } else {
        warn!("ref_node.node_type != Attribute");
        Err(Error::InvalidState)
    }
}

///
/// Determines if the specified node is of type `NodeType::DocumentType`.
///
#[inline]
pub fn is_document_type(ref_node: &RefNode) -> bool {
    ref_node.borrow().i_node_type == NodeType::DocumentType
}

///
/// Safely _cast_ the specified `RefNode` into a  `DocumentType`.
///
#[inline]
pub fn as_document_type(ref_node: &RefNode) -> Result<RefDocumentType<'_>> {
    if ref_node.borrow().i_node_type == NodeType::DocumentType {
        Ok(ref_node as RefDocumentType<'_>)
    } else {
        warn!("ref_node.node_type != DocumentType");
        Err(Error::InvalidState)
    }
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `DocumentType`.
///
#[inline]
pub fn as_document_type_mut(ref_node: &mut RefNode) -> Result<RefDocumentType<'_>> {
    if ref_node.borrow().i_node_type == NodeType::DocumentType {
        Ok(ref_node as MutRefDocumentType<'_>)
    } else {
        warn!("ref_node.node_type != DocumentType");
        Err(Error::InvalidState)
    }
}

///
/// Determines if the specified node is of type `NodeType::DocumentFragment`.
///
#[inline]
pub fn is_document_fragment(ref_node: &RefNode) -> bool {
    ref_node.borrow().i_node_type == NodeType::DocumentFragment
}

///
/// Safely _cast_ the specified `RefNode` into a  `DocumentFragment`.
///
#[inline]
pub fn as_document_fragment(ref_node: &RefNode) -> Result<RefDocumentFragment<'_>> {
    if ref_node.borrow().i_node_type == NodeType::DocumentFragment {
        Ok(ref_node as RefDocumentFragment<'_>)
    } else {
        warn!("ref_node.node_type != DocumentFragment");
        Err(Error::InvalidState)
    }
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `DocumentFragment`.
///
#[inline]
pub fn as_document_fragment_mut(ref_node: &mut RefNode) -> Result<MutRefDocumentFragment<'_>> {
    if ref_node.borrow().i_node_type == NodeType::DocumentFragment {
        Ok(ref_node as MutRefDocumentFragment<'_>)
    } else {
        warn!("ref_node.node_type != DocumentFragment");
        Err(Error::InvalidState)
    }
}

///
/// Determines if the specified node is of type `NodeType::Notation`.
///
#[inline]
pub fn is_notation(_ref_node: &RefNode) -> bool {
    panic!("node type Notation unsupported");
}

///
/// Safely _cast_ the specified `RefNode` into a `Notation`.
///
#[inline]
pub fn as_notation(_ref_node: &RefNode) -> Result<RefNotation<'_>> {
    warn!("node type Notation unsupported");
    Err(Error::NotSupported)
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `Notation`.
///
#[inline]
pub fn as_notation_mut(_ref_node: &mut RefNode) -> Result<MutRefNotation<'_>> {
    warn!("node type Notation unsupported");
    Err(Error::NotSupported)
}
