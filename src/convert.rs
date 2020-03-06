/*!
Provides safe `RefNode` conversion functions.

Note that all of the `as_{name}` functions work as follows.

* If the `node_type` corresponds to the correct type, it returns OK.
* If the `node_type` does not correspond to the correct type, it returns `Error::InvalidState`.
* If the `node_type` is not implemented it returns `Error::NotSupported`.

*/
use self::super::error::*;
use self::super::traits::*;

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
pub fn as_attribute(ref_node: &RefNode) -> Result<&dyn Attribute>  {
    if ref_node.borrow().i_node_type == NodeType::Attribute {
        Ok(ref_node as &dyn Attribute)
    } else {
        warn!("ref_node.node_type != Attribute");
        Err(Error::InvalidState)
    }
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `Attribute`.
///
#[inline]
pub fn as_attribute_mut(ref_node: &mut RefNode) -> Result<&mut dyn Attribute>  {
    if ref_node.borrow().i_node_type == NodeType::Attribute {
        Ok(ref_node as &mut dyn Attribute)
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
/// Safely _cast_ the specified `RefNode` into a  `Element`.
///
#[inline]
pub fn as_element(ref_node: &RefNode) -> Result<&dyn Element>  {
    if ref_node.borrow().i_node_type == NodeType::Element {
        Ok(ref_node as &dyn Element)
    } else {
        warn!("ref_node.node_type != Element");
        Err(Error::InvalidState)
    }
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `Element`.
///
#[inline]
pub fn as_element_mut(ref_node: &mut RefNode) -> Result<&mut dyn Element>  {
    if ref_node.borrow().i_node_type == NodeType::Element {
        Ok(ref_node as &mut dyn Element)
    } else {
        warn!("ref_node.node_type != Element");
        Err(Error::InvalidState)
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
pub fn as_text(ref_node: &RefNode) -> Result<&dyn Text>  {
    if ref_node.borrow().i_node_type == NodeType::Text {
        Ok(ref_node as &dyn Text)
    } else {
        warn!("ref_node.node_type != Text");
        Err(Error::InvalidState)
    }
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `Text`.
///
#[inline]
pub fn as_text_mut(ref_node: &mut RefNode) -> Result<&mut dyn Text>  {
    if ref_node.borrow().i_node_type == NodeType::Text {
        Ok(ref_node as &mut dyn Text)
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
pub fn as_cdata_section(ref_node: &RefNode) -> Result<&dyn CDataSection>  {
    if ref_node.borrow().i_node_type == NodeType::CData {
        Ok(ref_node as &dyn CDataSection)
    } else {
        warn!("ref_node.node_type != CData");
        Err(Error::InvalidState)
    }
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `CDataSection`.
///
#[inline]
pub fn as_cdata_section_mut(ref_node: &mut RefNode) -> Result<&mut dyn CDataSection>  {
    if ref_node.borrow().i_node_type == NodeType::CData {
        Ok(ref_node as &mut dyn CDataSection)
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
pub fn as_entity_reference(_ref_node: &RefNode) -> Result<&dyn EntityReference>  {
    warn!("node type EntityReference unsupported");
    Err(Error::NotSupported)
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `EntityReference`.
///
#[inline]
pub fn as_entity_reference_mut(_ref_node: &mut RefNode) -> Result<&mut dyn EntityReference>  {
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
pub fn as_entity(_ref_node: &RefNode) -> Result<&dyn Entity>  {
    warn!("node type Entity unsupported");
    Err(Error::NotSupported)
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `Entity`.
///
#[inline]
pub fn as_entity_mut(_ref_node: &mut RefNode) -> Result<&mut dyn Entity>  {
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
pub fn as_processing_instruction(ref_node: &RefNode) -> Result<&dyn ProcessingInstruction>  {
    if ref_node.borrow().i_node_type == NodeType::ProcessingInstruction {
        Ok(ref_node as &dyn ProcessingInstruction)
    } else {
        warn!("ref_node.node_type != ProcessingInstruction");
        Err(Error::InvalidState)
    }
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `ProcessingInstruction`.
///
#[inline]
pub fn as_processing_instruction_mut(ref_node: &mut RefNode) -> Result<&mut dyn ProcessingInstruction>  {
    if ref_node.borrow().i_node_type == NodeType::ProcessingInstruction {
        Ok(ref_node as &mut dyn ProcessingInstruction)
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
pub fn as_comment(ref_node: &RefNode) -> Result<&dyn Comment>  {
    if ref_node.borrow().i_node_type == NodeType::Comment {
        Ok(ref_node as &dyn Comment)
    } else {
        warn!("ref_node.node_type != Comment");
        Err(Error::InvalidState)
    }
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `Comment`.
///
#[inline]
pub fn as_comment_mut(ref_node: &mut RefNode) -> Result<&mut dyn Comment>  {
    if ref_node.borrow().i_node_type == NodeType::Comment {
        Ok(ref_node as &mut dyn Comment)
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
pub fn as_document(ref_node: &RefNode) -> Result<&dyn Document>  {
    if ref_node.borrow().i_node_type == NodeType::Document {
        Ok(ref_node as &dyn Document)
    } else {
        warn!("ref_node.node_type != Attribute");
        Err(Error::InvalidState)
    }
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `Document`.
///
#[inline]
pub fn as_document_mut(ref_node: &mut RefNode) -> Result<&mut dyn Document>  {
    if ref_node.borrow().i_node_type == NodeType::Document {
        Ok(ref_node as &mut dyn Document)
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
pub fn as_document_type(ref_node: &RefNode) -> Result<&dyn DocumentType>  {
    if ref_node.borrow().i_node_type == NodeType::DocumentType {
        Ok(ref_node as &dyn DocumentType)
    } else {
        warn!("ref_node.node_type != DocumentType");
        Err(Error::InvalidState)
    }
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `DocumentType`.
///
#[inline]
pub fn as_document_type_mut(ref_node: &mut RefNode) -> Result<&mut dyn DocumentType>  {
    if ref_node.borrow().i_node_type == NodeType::DocumentType {
        Ok(ref_node as &mut dyn DocumentType)
    } else {
        warn!("ref_node.node_type != DocumentType");
        Err(Error::InvalidState)
    }
}

///
/// Determines if the specified node is of type `NodeType::DocumentFragment`.
///
#[inline]
pub fn is_document_fragment(_ref_node: &RefNode) -> bool {
    panic!("node type DocumentFragment unsupported");
}

///
/// Safely _cast_ the specified `RefNode` into a  `DocumentFragment`.
///
#[inline]
pub fn as_document_fragment(_ref_node: &RefNode) -> Result<&dyn DocumentFragment>  {
    warn!("node type DocumentFragment unsupported");
    Err(Error::NotSupported)
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `DocumentFragment`.
///
#[inline]
pub fn as_document_fragment_mut(_ref_node: &mut RefNode) -> Result<&mut dyn DocumentFragment>  {
    warn!("node type DocumentFragment unsupported");
    Err(Error::NotSupported)
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
pub fn as_notation(_ref_node: &RefNode) -> Result<&dyn Notation>  {
    warn!("node type Notation unsupported");
    Err(Error::NotSupported)
}

///
/// Safely _cast_ the specified `RefNode` into a mutable `Notation`.
///
#[inline]
pub fn as_notation_mut(_ref_node: &mut RefNode) -> Result<&mut dyn Notation>  {
    warn!("node type Notation unsupported");
    Err(Error::NotSupported)
}
