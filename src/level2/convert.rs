/*!
Provides safe `RefNode` conversion functions.

Note that all of the `as_{name}` functions work as follows.

* If the `node_type` corresponds to the correct type, it returns OK.
* If the `node_type` does not correspond to the correct type, it returns `Error::InvalidState`.
* If the `node_type` is not implemented it returns `Error::NotSupported`.

*/
use crate::level2::node_impl::*;
use crate::level2::traits::*;
use crate::shared::error::{Error, Result, MSG_INVALID_NODE_TYPE};

use crate::{make_is_as_functions, make_ref_type};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

make_ref_type!(RefAttribute, MutRefAttribute, Attribute);

make_ref_type!(RefCDataSection, MutRefCDataSection, CDataSection);

make_ref_type!(RefCharacterData, MutRefCharacterData, CharacterData);

make_ref_type!(RefComment, MutRefComment, Comment);

make_ref_type!(RefDocument, MutRefDocument, Document);

make_ref_type!(
    RefDocumentFragment,
    MutRefDocumentFragment,
    DocumentFragment
);

make_ref_type!(RefDocumentType, MutRefDocumentType, DocumentType);

make_ref_type!(RefElement, MutRefElement, Element);

make_ref_type!(RefEntity, MutRefEntity, Entity);

make_ref_type!(RefEntityReference, MutRefEntityReference, EntityReference);

make_ref_type!(RefNotation, MutRefNotation, Notation);

make_ref_type!(
    RefProcessingInstruction,
    MutRefProcessingInstruction,
    ProcessingInstruction
);

make_ref_type!(RefText, MutRefText, Text);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

make_is_as_functions!(
    is_attribute,
    NodeType::Attribute,
    as_attribute,
    RefAttribute,
    as_attribute_mut,
    MutRefAttribute
);

make_is_as_functions!(
    is_element,
    NodeType::Element,
    as_element,
    RefElement,
    as_element_mut,
    MutRefElement
);

///
/// Determines if the specified node is a type of `CharacterData`.
///
#[inline]
pub fn is_character_data(ref_node: &RefNode) -> bool {
    matches!(
        ref_node.borrow().i_node_type,
        NodeType::CData | NodeType::Comment | NodeType::Text
    )
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
            warn!("{}", MSG_INVALID_NODE_TYPE);
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
            warn!("{}", MSG_INVALID_NODE_TYPE);
            Err(Error::InvalidState)
        }
    }
}

make_is_as_functions!(
    is_text,
    NodeType::Text,
    as_text,
    RefText,
    as_text_mut,
    MutRefText
);

make_is_as_functions!(
    is_cdata_section,
    NodeType::CData,
    as_cdata_section,
    RefCDataSection,
    as_cdata_section_mut,
    MutRefCDataSection
);

make_is_as_functions!(
    is_entity_reference,
    NodeType::EntityReference,
    as_entity_reference,
    RefEntityReference,
    as_entity_reference_mut,
    MutRefEntityReference
);

make_is_as_functions!(
    is_entity_,
    NodeType::Entity,
    as_entity,
    RefEntity,
    as_entity_mut,
    MutRefEntity
);

make_is_as_functions!(
    is_processing_instruction,
    NodeType::ProcessingInstruction,
    as_processing_instruction,
    RefProcessingInstruction,
    as_processing_instruction_mut,
    MutRefProcessingInstruction
);

make_is_as_functions!(
    is_comment,
    NodeType::Comment,
    as_comment,
    RefComment,
    as_comment_mut,
    MutRefComment
);

make_is_as_functions!(
    is_document,
    NodeType::Document,
    as_document,
    RefDocument,
    as_document_mut,
    MutRefDocument
);

make_is_as_functions!(
    is_document_type,
    NodeType::DocumentType,
    as_document_type,
    RefDocumentType,
    as_document_type_mut,
    MutRefDocumentType
);

make_is_as_functions!(
    is_document_fragment,
    NodeType::DocumentFragment,
    as_document_fragment,
    RefDocumentFragment,
    as_document_fragment_mut,
    MutRefDocumentFragment
);

make_is_as_functions!(
    is_notation,
    NodeType::Notation,
    as_notation,
    RefNotation,
    as_notation_mut,
    MutRefNotation
);
