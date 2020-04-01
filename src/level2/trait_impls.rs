use crate::level2::convert::*;
use crate::level2::dom_impl::{get_implementation, Implementation};
use crate::level2::ext::convert::as_element_namespaced_mut;
use crate::level2::ext::options::ProcessingOptions;
use crate::level2::node_impl::*;
use crate::level2::traits::*;
use crate::shared::display;
use crate::shared::error::*;
use crate::shared::name::Name;
use crate::shared::syntax::*;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! unwrap_extension_field {
    ($node:expr, $variant:ident, $field:ident) => {{
        let ref_self = $node.borrow();
        if let Extension::$variant { $field, .. } = &ref_self.i_extension {
            $field.clone()
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            Default::default()
        }
    }};
    ($node:expr, $variant:ident, $field:ident, $closure_fn:expr) => {{
        let ref_self = $node.borrow();
        if let Extension::$variant { $field, .. } = &ref_self.i_extension {
            $closure_fn($field)
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            Default::default()
        }
    }};
    ($node:expr, $variant:ident, $field:ident, $some_closure:expr) => {{
        let ref_self = $node.borrow();
        if let Extension::$variant { $field, .. } = &ref_self.i_extension {
            match $field {
                None => Default::default(),
                Some(value) => $some_closure(value),
            }
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            Default::default()
        }
    }};
    ($node:expr, $variant:ident, $field:ident, $none_closure:expr, $some_closure:expr) => {{
        let ref_self = $node.borrow();
        if let Extension::$variant { $field, .. } = &ref_self.i_extension {
            match $field {
                None => $none_closure(),
                Some(value) => $some_closure(value),
            }
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            Default::default()
        }
    }};
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Attribute for RefNode {
    fn owner_element(&self) -> Option<Self::NodeRef> {
        unwrap_extension_field!(
            self,
            Attribute,
            i_owner_element,
            |i_owner_element: &Option<WeakRefNode>| {
                match i_owner_element {
                    None => None,
                    Some(weak_ref) => match weak_ref.clone().upgrade() {
                        None => {
                            warn!("{}", MSG_WEAK_REF);
                            None
                        }
                        Some(ref_element) => Some(ref_element),
                    },
                }
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl CDataSection for RefNode {}

// ------------------------------------------------------------------------------------------------

impl CharacterData for RefNode {
    fn substring_data(&self, offset: usize, count: usize) -> Result<String> {
        if offset + count == offset {
            return Ok(String::new());
        }
        let ref_self = self.borrow();
        match &ref_self.i_value {
            None => {
                warn!("{}", MSG_INDEX_ERROR);
                Err(Error::IndexSize)
            }
            Some(data) => {
                if offset >= data.len() {
                    warn!("{}", MSG_INDEX_ERROR);
                    Err(Error::IndexSize)
                } else if offset + count >= data.len() {
                    Ok(data[offset..].to_string())
                } else {
                    Ok(data[offset..offset + count].to_string())
                }
            }
        }
    }

    fn append_data(&mut self, new_data: &str) -> Result<()> {
        if new_data.is_empty() {
            return Ok(());
        }
        let mut mut_self = self.borrow_mut();
        match &mut_self.i_value {
            None => mut_self.i_value = Some(new_data.to_string()),
            Some(old_data) => mut_self.i_value = Some(format!("{}{}", old_data, new_data)),
        }
        Ok(())
    }

    fn insert_data(&mut self, offset: usize, new_data: &str) -> Result<()> {
        if new_data.is_empty() {
            return Ok(());
        }
        self.replace_data(offset, 0, new_data)
    }

    fn delete_data(&mut self, offset: usize, count: usize) -> Result<()> {
        if offset + count == offset {
            return Ok(());
        }
        const NOTHING: &str = "";
        self.replace_data(offset, count, NOTHING)
    }

    fn replace_data(&mut self, offset: usize, count: usize, replace_data: &str) -> Result<()> {
        let mut mut_self = self.borrow_mut();
        match &mut_self.i_value {
            None => {
                if offset + count != 0 {
                    warn!("{}", MSG_INDEX_ERROR);
                    Err(Error::IndexSize)
                } else {
                    mut_self.i_value = Some(replace_data.to_string());
                    Ok(())
                }
            }
            Some(old_data) => {
                if offset >= old_data.len() {
                    warn!("{}", MSG_INDEX_ERROR);
                    Err(Error::IndexSize)
                } else {
                    let mut new_data = old_data.clone();
                    if offset + count >= old_data.len() {
                        new_data.replace_range(offset.., replace_data);
                    } else {
                        new_data.replace_range(offset..offset + count, replace_data);
                    }
                    mut_self.i_value = Some(new_data);
                    Ok(())
                }
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Comment for RefNode {}

// ------------------------------------------------------------------------------------------------

impl Document for RefNode {
    fn doc_type(&self) -> Option<RefNode> {
        unwrap_extension_field!(self, Document, i_document_type)
    }

    fn document_element(&self) -> Option<RefNode> {
        self.child_nodes().first().map(Clone::clone)
    }

    fn implementation(&self) -> &dyn DOMImplementation<NodeRef = RefNode> {
        let ref_self = self.borrow();
        if let Extension::Document {
            i_implementation, ..
        } = &ref_self.i_extension
        {
            i_implementation.clone()
        } else {
            panic!("{}", MSG_INVALID_EXTENSION);
        }
    }

    fn create_attribute(&self, name: &str) -> Result<RefNode> {
        let name = Name::from_str(name)?;
        let node_impl = NodeImpl::new_attribute(self.clone().downgrade(), name, None);
        Ok(RefNode::new(node_impl))
    }

    fn create_attribute_with(&self, name: &str, value: &str) -> Result<RefNode> {
        let name = Name::from_str(name)?;
        let node_impl = NodeImpl::new_attribute(self.clone().downgrade(), name, Some(value));
        Ok(RefNode::new(node_impl))
    }

    fn create_attribute_ns(&self, namespace_uri: &str, qualified_name: &str) -> Result<RefNode> {
        let name = Name::new_ns(namespace_uri, qualified_name)?;
        let node_impl = NodeImpl::new_attribute(self.clone().downgrade(), name, None);
        Ok(RefNode::new(node_impl))
    }

    fn create_cdata_section(&self, data: &str) -> Result<RefNode> {
        let node_impl = NodeImpl::new_cdata(self.clone().downgrade(), data);
        Ok(RefNode::new(node_impl))
    }

    fn create_document_fragment(&self) -> Result<RefNode> {
        let node_impl = NodeImpl::new_document_fragment(self.clone().downgrade());
        Ok(RefNode::new(node_impl))
    }

    fn create_entity_reference(&self, name: &str) -> Result<RefNode> {
        let name = Name::from_str(name)?;
        let node_impl = NodeImpl::new_entity_reference(self.clone().downgrade(), name);
        Ok(RefNode::new(node_impl))
    }

    fn create_comment(&self, data: &str) -> RefNode {
        let node_impl = NodeImpl::new_comment(self.clone().downgrade(), data);
        RefNode::new(node_impl)
    }

    fn create_element(&self, tag_name: &str) -> Result<RefNode> {
        let name = Name::from_str(tag_name)?;
        let node_impl = NodeImpl::new_element(self.clone().downgrade(), name);
        Ok(RefNode::new(node_impl))
    }

    fn create_element_ns(&self, namespace_uri: &str, qualified_name: &str) -> Result<RefNode> {
        let name = Name::new_ns(namespace_uri, qualified_name)?;
        let node_impl = NodeImpl::new_element(self.clone().downgrade(), name);
        Ok(RefNode::new(node_impl))
    }

    fn create_processing_instruction(&self, target: &str, data: Option<&str>) -> Result<RefNode> {
        //
        // Ensure:
        //
        // `PITarget  ::=  Name - (('X' | 'x') ('M' | 'm') ('L' | 'l'))`
        //
        if target.to_ascii_lowercase() == XML_PI_RESERVED {
            return Err(Error::Syntax);
        }
        let target = Name::from_str(target)?;
        let node_impl =
            NodeImpl::new_processing_instruction(self.clone().downgrade(), target, data);
        Ok(RefNode::new(node_impl))
    }

    fn create_text_node(&self, data: &str) -> RefNode {
        let node_impl = NodeImpl::new_text(self.clone().downgrade(), data);
        RefNode::new(node_impl)
    }

    fn get_element_by_id(&self, id: &str) -> Option<RefNode> {
        let ref_self = self.borrow();
        if let Extension::Document { i_id_map, .. } = &ref_self.i_extension {
            match i_id_map.get(&id.to_string()) {
                None => None,
                Some(weak_ref) => match weak_ref.clone().upgrade() {
                    None => {
                        warn!("{}", MSG_WEAK_REF);
                        None
                    }
                    Some(ref_element) => Some(ref_element),
                },
            }
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            None
        }
    }

    fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<RefNode> {
        //
        // Delegate this call to the document element
        //
        if let Some(root_element) = self.document_element() {
            Element::get_elements_by_tag_name(&root_element, tag_name)
        } else {
            Vec::default()
        }
    }

    fn get_elements_by_tag_name_ns(&self, namespace_uri: &str, local_name: &str) -> Vec<RefNode> {
        //
        // Delegate this call to the document element
        //
        if let Some(root_element) = self.document_element() {
            Element::get_elements_by_tag_name_ns(&root_element, namespace_uri, local_name)
        } else {
            Vec::default()
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl DocumentFragment for RefNode {}

// ------------------------------------------------------------------------------------------------

impl DocumentType for RefNode {
    fn entities(&self) -> HashMap<Name, Self::NodeRef, RandomState> {
        unwrap_extension_field!(self, DocumentType, i_entities)
    }

    fn notations(&self) -> HashMap<Name, Self::NodeRef, RandomState> {
        unwrap_extension_field!(self, DocumentType, i_notations)
    }

    fn public_id(&self) -> Option<String> {
        unwrap_extension_field!(self, DocumentType, i_public_id)
    }

    fn system_id(&self) -> Option<String> {
        unwrap_extension_field!(self, DocumentType, i_system_id)
    }

    fn internal_subset(&self) -> Option<String> {
        unwrap_extension_field!(self, DocumentType, i_internal_subset)
    }
}

// ------------------------------------------------------------------------------------------------

impl DOMImplementation for Implementation {
    type NodeRef = RefNode;

    fn create_document(
        &self,
        namespace_uri: Option<&str>,
        qualified_name: Option<&str>,
        doc_type: Option<RefNode>,
    ) -> Result<RefNode> {
        create_document_with_options(namespace_uri, qualified_name, doc_type, Default::default())
    }

    fn create_document_type(
        &self,
        qualified_name: &str,
        public_id: Option<&str>,
        system_id: Option<&str>,
    ) -> Result<RefNode> {
        let name = Name::from_str(qualified_name)?;
        let node_impl = NodeImpl::new_document_type(None, name, public_id, system_id);
        Ok(RefNode::new(node_impl))
    }

    fn has_feature(&self, feature: &str, version: &str) -> bool {
        (feature == XML_FEATURE_CORE || feature == XML_FEATURE_XML)
            && (version == XML_FEATURE_V1 || version == XML_FEATURE_V2)
    }
}

// ------------------------------------------------------------------------------------------------

impl Element for RefNode {
    fn get_attribute(&self, name: &str) -> Option<String> {
        match self.get_attribute_node(name) {
            None => None,
            Some(attribute_node) => match as_attribute(&attribute_node) {
                Ok(attribute) => attribute.value(),
                Err(_) => {
                    warn!("{}", MSG_INVALID_NODE_TYPE);
                    None
                }
            },
        }
    }

    fn set_attribute(&mut self, name: &str, value: &str) -> Result<()> {
        let attr_name = Name::from_str(name)?;
        let attr_node = {
            let ref_self = &self.borrow_mut();
            let document = ref_self.i_owner_document.as_ref().unwrap();
            NodeImpl::new_attribute(document.clone(), attr_name, Some(value))
        };
        self.set_attribute_node(RefNode::new(attr_node)).map(|_| ())
    }

    fn remove_attribute(&mut self, name: &str) -> Result<()> {
        match self.get_attribute_node(name) {
            None => Ok(()),
            Some(attribute_node) => self.remove_attribute_node(attribute_node).map(|_| ()),
        }
    }

    fn get_attribute_node(&self, name: &str) -> Option<RefNode> {
        if is_element(self) {
            match Name::from_str(name) {
                Ok(name) => {
                    let ref_self = self.borrow();
                    if let Extension::Element { i_attributes, .. } = &ref_self.i_extension {
                        let node_name = name.to_string();
                        i_attributes
                            .iter()
                            .find(|(name, _)| name.to_string() == node_name)
                            .map(|(_, node)| node.clone())
                    } else {
                        warn!("{}", MSG_INVALID_EXTENSION);
                        None
                    }
                }
                Err(_) => {
                    warn!("{}: '{}'", MSG_INVALID_NAME, name);
                    None
                }
            }
        } else {
            warn!("{}", MSG_INVALID_NODE_TYPE);
            None
        }
    }

    fn set_attribute_node(&mut self, new_attribute: RefNode) -> Result<RefNode> {
        if is_element(self) && is_attribute(&new_attribute) {
            let name: Name = new_attribute.node_name();
            if name.is_namespace_attribute() {
                //
                // Add to the element's namespace mapping hash
                //
                let attribute = as_attribute(&new_attribute).unwrap();
                let namespace_uri = attribute.value().unwrap();

                let as_namespaced = as_element_namespaced_mut(self).unwrap();
                let _ignore = match &name.prefix() {
                    None => as_namespaced.insert_mapping(None, &namespace_uri),
                    Some(prefix) => as_namespaced.insert_mapping(Some(prefix), &namespace_uri),
                }?;
            }
            let mut mut_self = self.borrow_mut();
            if let Extension::Element { i_attributes, .. } = &mut mut_self.i_extension {
                let _safe_to_ignore =
                    i_attributes.insert(new_attribute.node_name(), new_attribute.clone());
                {
                    //
                    // Add to the owning document's id_map hash
                    //
                    let attribute = as_attribute(&new_attribute).unwrap();
                    let document = attribute.owner_document().unwrap();
                    let mut mut_document = document.borrow_mut();
                    let lax =
                        if let Extension::Document { i_options, .. } = &mut_document.i_extension {
                            i_options.has_assume_ids()
                        } else {
                            warn!("{}", MSG_INVALID_EXTENSION);
                            false
                        };
                    if name.is_id_attribute(lax) {
                        if let Extension::Document { i_id_map, .. } = &mut mut_document.i_extension
                        {
                            let id_value = attribute.value().unwrap();
                            if i_id_map.contains_key(&id_value) {
                                warn!("{}", MSG_DUPLICATE_ID);
                                return Err(Error::Syntax);
                            }
                            let _safe_to_ignore =
                                i_id_map.insert(id_value, self.clone().downgrade());
                        } else {
                            warn!("{}", MSG_INVALID_EXTENSION);
                        }
                    }
                }
                Ok(new_attribute)
            } else {
                warn!("{}", MSG_INVALID_EXTENSION);
                Err(Error::Syntax)
            }
        } else {
            warn!("{}", MSG_INVALID_NODE_TYPE);
            Err(Error::InvalidState)
        }
    }

    fn remove_attribute_node(&mut self, old_attribute: RefNode) -> Result<RefNode> {
        if is_element(self) {
            let mut mut_self = self.borrow_mut();
            if let Extension::Element { i_attributes, .. } = &mut mut_self.i_extension {
                let _safe_to_ignore = i_attributes.remove(&old_attribute.node_name());
                let mut_old = old_attribute.clone();
                let mut mut_old = mut_old.borrow_mut();
                mut_old.i_parent_node = None;
                // TODO: remove from Element::namespaces
                // TODO: remove from Document::id_map
                Ok(old_attribute)
            } else {
                warn!("{}", MSG_INVALID_EXTENSION);
                Err(Error::Syntax)
            }
        } else {
            warn!("{}", MSG_INVALID_NODE_TYPE);
            Err(Error::InvalidState)
        }
    }

    fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<RefNode> {
        let mut results = Vec::default();
        if is_element(self) {
            let tag_name = tag_name.to_string();
            let ref_self = self.borrow();
            if tag_name_match(&ref_self.i_name.to_string(), &tag_name) {
                results.push(self.clone());
            }
            for child_node in &ref_self.i_child_nodes {
                match as_element(child_node) {
                    Ok(ref_child) => results.extend(ref_child.get_elements_by_tag_name(&tag_name)),
                    Err(_) => {
                        warn!("{}", MSG_INVALID_NODE_TYPE);
                    }
                }
            }
        }
        results
    }

    fn get_attribute_ns(&self, namespace_uri: &str, local_name: &str) -> Option<String> {
        match self.get_attribute_node_ns(namespace_uri, local_name) {
            None => None,
            Some(attribute_node) => match as_attribute(&attribute_node) {
                Ok(attribute) => attribute.value(),
                Err(_) => {
                    warn!("{}", MSG_INVALID_NODE_TYPE);
                    None
                }
            },
        }
    }

    fn set_attribute_ns(
        &mut self,
        namespace_uri: &str,
        qualified_name: &str,
        value: &str,
    ) -> Result<()> {
        let attr_name = Name::new_ns(namespace_uri, qualified_name)?;
        let attr_node = {
            let ref_self = &self.borrow_mut();
            let document = ref_self.i_owner_document.as_ref().unwrap();
            NodeImpl::new_attribute(document.clone(), attr_name, Some(value))
        };
        self.set_attribute_node(RefNode::new(attr_node)).map(|_| ())
    }

    fn remove_attribute_ns(&mut self, namespace_uri: &str, local_name: &str) -> Result<()> {
        match self.get_attribute_node_ns(namespace_uri, local_name) {
            None => Ok(()),
            Some(attribute_node) => self.remove_attribute_node(attribute_node).map(|_| ()),
        }
    }

    fn get_attribute_node_ns(&self, namespace_uri: &str, local_name: &str) -> Option<RefNode> {
        if is_element(self) {
            match Name::new_ns(namespace_uri, local_name) {
                Ok(_) => {
                    let ref_self = self.borrow();
                    if let Extension::Element { i_attributes, .. } = &ref_self.i_extension {
                        let namespace_uri = &Some(namespace_uri.to_string());
                        let local_name = &local_name.to_string();
                        i_attributes
                            .iter()
                            .find(|(name, _)| {
                                name.namespace_uri() == namespace_uri
                                    && name.local_name() == local_name
                            })
                            .map(|(_, node)| node.clone())
                    } else {
                        warn!("{}", MSG_INVALID_EXTENSION);
                        None
                    }
                }
                Err(_) => {
                    warn!("{}: '{}'", MSG_INVALID_NAME, local_name);
                    None
                }
            }
        } else {
            warn!("{}", MSG_INVALID_NODE_TYPE);
            None
        }
    }

    fn set_attribute_node_ns(&mut self, new_attribute: RefNode) -> Result<RefNode> {
        self.set_attribute_node(new_attribute)
    }

    fn get_elements_by_tag_name_ns(&self, namespace_uri: &str, local_name: &str) -> Vec<RefNode> {
        let mut results = Vec::default();
        if is_element(self) {
            let namespace_uri = namespace_uri.to_string();
            let local_name = local_name.to_string();
            let ref_self = self.borrow();
            if namespaced_name_match(
                match ref_self.i_name.namespace_uri() {
                    None => None,
                    Some(s) => Some(s.as_str()),
                },
                &ref_self.i_name.local_name(),
                &namespace_uri,
                &local_name,
            ) {
                results.push(self.clone());
            }
            for child_node in &ref_self.i_child_nodes {
                match as_element(child_node) {
                    Ok(ref_child) => results
                        .extend(ref_child.get_elements_by_tag_name_ns(&namespace_uri, &local_name)),
                    Err(_) => {
                        warn!("{}", MSG_INVALID_NODE_TYPE);
                    }
                }
            }
        }
        results
    }

    fn has_attribute(&self, name: &str) -> bool {
        if is_element(self) {
            match Name::from_str(name) {
                Ok(name) => {
                    let ref_self = self.borrow();
                    if let Extension::Element { i_attributes, .. } = &ref_self.i_extension {
                        i_attributes
                            .keys()
                            .any(|n| n.to_string() == name.to_string())
                    } else {
                        warn!("{}", MSG_INVALID_EXTENSION);
                        false
                    }
                }
                Err(_) => {
                    warn!("{}: '{}'", MSG_INVALID_NAME, name);
                    false
                }
            }
        } else {
            warn!("{}", MSG_INVALID_NODE_TYPE);
            false
        }
    }

    fn has_attribute_ns(&self, namespace_uri: &str, local_name: &str) -> bool {
        if is_element(self) {
            match Name::new_ns(namespace_uri, local_name) {
                Ok(name) => {
                    let ref_self = self.borrow();
                    if let Extension::Element { i_attributes, .. } = &ref_self.i_extension {
                        i_attributes.keys().any(|n| {
                            n.namespace_uri() == name.namespace_uri()
                                && n.local_name() == name.local_name()
                        })
                    } else {
                        warn!("{}", MSG_INVALID_EXTENSION);
                        false
                    }
                }
                Err(_) => {
                    warn!("{}: '{}'", MSG_INVALID_NAME, local_name);
                    false
                }
            }
        } else {
            warn!("{}", MSG_INVALID_NODE_TYPE);
            false
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Entity for RefNode {
    fn public_id(&self) -> Option<String> {
        unwrap_extension_field!(self, Entity, i_public_id)
    }

    fn system_id(&self) -> Option<String> {
        unwrap_extension_field!(self, Entity, i_system_id)
    }

    fn notation_name(&self) -> Option<String> {
        unwrap_extension_field!(self, Entity, i_notation_name)
    }
}

// ------------------------------------------------------------------------------------------------

impl EntityReference for RefNode {}

// ------------------------------------------------------------------------------------------------

impl Node for RefNode {
    type NodeRef = RefNode;

    fn node_name(&self) -> Name {
        let ref_self = self.borrow();
        ref_self.i_name.clone()
    }

    //
    // For Attribute instances:
    // On retrieval, the value of the attribute is returned as a string. Character and general
    // entity references are replaced with their values. See also the method `getAttribute` on the
    // `Element` interface.
    //
    // On setting, this creates a `Text` node with the unparsed contents of the string. I.e. any
    // characters that an XML processor would recognize as markup are instead treated as literal
    // text. See also the method `setAttribute` on the `Element` interface.
    //
    fn node_value(&self) -> Option<String> {
        // TODO: attribute special handling
        let ref_self = self.borrow();
        ref_self.i_value.clone()
    }

    fn set_node_value(&mut self, value: &str) -> Result<()> {
        // TODO: attribute special handling
        let mut mut_self = self.borrow_mut();
        mut_self.i_value = Some(value.to_string());
        Ok(())
    }

    fn unset_node_value(&mut self) -> Result<()> {
        // TODO: attribute special handling
        let mut mut_self = self.borrow_mut();
        mut_self.i_value = None;
        Ok(())
    }

    fn node_type(&self) -> NodeType {
        let ref_self = self.borrow();
        ref_self.i_node_type.clone()
    }

    fn parent_node(&self) -> Option<RefNode> {
        if is_attribute(self) {
            return None;
        }
        let ref_self = self.borrow();
        match &ref_self.i_parent_node {
            None => None,
            Some(node) => node.clone().upgrade(),
        }
    }

    fn child_nodes(&self) -> Vec<RefNode> {
        let ref_self = self.borrow();
        ref_self.i_child_nodes.clone()
    }

    fn first_child(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        ref_self.i_child_nodes.first().map(|node| node.clone())
    }

    fn last_child(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        ref_self.i_child_nodes.last().map(|node| node.clone())
    }

    fn previous_sibling(&self) -> Option<RefNode> {
        if is_attribute(self) {
            return None;
        }
        let ref_self = self.borrow();
        match &ref_self.i_parent_node {
            None => {
                warn!("{}", MSG_NO_PARENT_NODE);
                None
            }
            Some(parent_node) => {
                let parent_node = parent_node.clone();
                let parent_node = parent_node.upgrade()?;
                let ref_parent = parent_node.borrow();
                match ref_parent
                    .i_child_nodes
                    .iter()
                    .position(|child| child == self)
                {
                    None => None,
                    Some(index) => {
                        if index == 0 {
                            None
                        } else {
                            let sibling = ref_parent.i_child_nodes.get(index - 1);
                            sibling.cloned()
                        }
                    }
                }
            }
        }
    }

    fn next_sibling(&self) -> Option<RefNode> {
        if is_attribute(self) {
            return None;
        }
        let ref_self = self.borrow();
        match &ref_self.i_parent_node {
            None => {
                warn!("{}", MSG_NO_PARENT_NODE);
                None
            }
            Some(parent_node) => {
                let parent_node = parent_node.clone();
                let parent_node = parent_node.upgrade()?;
                let ref_parent = parent_node.borrow();
                match ref_parent
                    .i_child_nodes
                    .iter()
                    .position(|child| child == self)
                {
                    None => None,
                    Some(index) => {
                        let sibling = ref_parent.i_child_nodes.get(index + 1);
                        sibling.cloned()
                    }
                }
            }
        }
    }

    fn attributes(&self) -> HashMap<Name, RefNode, RandomState> {
        if is_element(self) {
            unwrap_extension_field!(self, Element, i_attributes)
        } else {
            warn!("{}", MSG_INVALID_NODE_TYPE);
            HashMap::default()
        }
    }

    fn owner_document(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        match &ref_self.i_owner_document {
            None => None,
            Some(node) => node.clone().upgrade(),
        }
    }

    fn insert_before(&mut self, new_child: RefNode, ref_child: Option<RefNode>) -> Result<RefNode> {
        fn insert_or_append(
            parent_node: &mut RefNode,
            new_child: &RefNode,
            insert_position: Option<usize>,
        ) {
            let mut mut_parent = parent_node.borrow_mut();
            let new_child = new_child.clone();
            match insert_position {
                None => mut_parent.i_child_nodes.push(new_child),
                Some(position) => mut_parent.i_child_nodes.insert(position, new_child),
            }
        }

        if !is_child_allowed(self, &new_child) {
            println!("The child you tried to add is not valid for this parent.");
            return Err(Error::HierarchyRequest);
        }

        //
        // Special case for Document only.
        //
        if is_document(self)
            && is_element(&new_child)
            && self
                .child_nodes()
                .iter()
                .any(|n| n.node_type() == NodeType::Element)
        {
            println!("cannot add more than one element to a document");
            return Error::HierarchyRequest.into();
        }

        //
        // Find the index in `child_nodes` of the `ref_child`.
        //
        let insert_position = match ref_child {
            None => None,
            Some(ref_child) => match self
                .borrow()
                .i_child_nodes
                .iter()
                .position(|child| child == &ref_child)
            {
                None => {
                    warn!("insert_before: ref_child not found in `child_nodes`");
                    return Error::NotFound.into();
                }
                position => position,
            },
        };

        {
            //
            // CHECK: Raise `Error::WrongDocument` if `newChild` was created from a different
            // document than the one that created this node.
            //
            let self_parent = &self.borrow().i_parent_node;
            let child_parent = &self.borrow().i_parent_node;
            if !match (self_parent, child_parent) {
                (None, None) => true,
                (Some(_), None) => true,
                (None, Some(_)) => false,
                (Some(self_parent), Some(child_parent)) => {
                    let self_parent = self_parent.clone().upgrade().unwrap();
                    let child_parent = child_parent.clone().upgrade().unwrap();
                    self_parent == child_parent
                }
            } {
                return Err(Error::WrongDocument);
            }
        }

        //
        // Remove from it's current parent
        //
        match new_child.parent_node() {
            None => (),
            Some(mut parent_node) => {
                let _safe_to_ignore = parent_node.remove_child(new_child.clone())?;
            }
        }

        {
            //
            // update new child with references from self
            //
            let ref_self = self.borrow();
            let mut mut_child = new_child.borrow_mut();
            mut_child.i_parent_node = Some(self.to_owned().downgrade());
            if is_document(self) {
                mut_child.i_owner_document = Some(self.clone().downgrade());
            } else {
                mut_child.i_owner_document = ref_self.i_owner_document.clone();
            }
        }

        if is_document_fragment(&new_child) {
            //
            // Special case
            //
            for (index, child) in new_child.child_nodes().iter().enumerate() {
                match insert_position {
                    None => insert_or_append(self, child, None),
                    Some(position) => insert_or_append(self, child, Some(position + index)),
                }
            }
        } else {
            insert_or_append(self, &new_child, insert_position)
        }

        Ok(new_child)
    }

    fn replace_child(&mut self, new_child: RefNode, old_child: RefNode) -> Result<RefNode> {
        if !is_child_allowed(self, &new_child) {
            return Err(Error::HierarchyRequest);
        }
        let exists = {
            let ref_self = self.borrow();
            ref_self.i_child_nodes.contains(&old_child.clone())
        };
        if exists {
            let next_node = old_child.next_sibling();
            let removed = self.remove_child(old_child)?;
            let _safe_to_ignore = self.insert_before(new_child, next_node)?;
            Ok(removed)
        } else {
            warn!("replace_child: old_child not found in `child_nodes`");
            Err(Error::NotFound)
        }
    }

    fn remove_child(&mut self, old_child: Self::NodeRef) -> Result<Self::NodeRef> {
        let position = {
            let ref_self = self.borrow();
            ref_self
                .i_child_nodes
                .iter()
                .position(|child| child == &old_child)
        };
        match position {
            None => {
                warn!("remove_child: old_child not found in `child_nodes`");
                Err(Error::NotFound)
            }
            Some(position) => {
                let removed = {
                    let mut mut_self = self.borrow_mut();
                    mut_self.i_child_nodes.remove(position)
                };
                let mut mut_removed = removed.borrow_mut();
                mut_removed.i_parent_node = None;
                Ok(removed.clone())
            }
        }
    }

    fn append_child(&mut self, new_child: RefNode) -> Result<RefNode> {
        self.insert_before(new_child, None)
    }

    fn has_child_nodes(&self) -> bool {
        !self.child_nodes().is_empty()
    }

    fn clone_node(&self, deep: bool) -> Option<RefNode> {
        let ref_self = self.borrow();
        let new_node = ref_self.clone_node(deep);
        Some(RefNode::new(new_node))
    }

    fn normalize(&mut self) {
        for child_node in self.child_nodes() {
            if is_text(&child_node) {
                if CharacterData::length(&child_node) == 0 {
                    if self.remove_child(child_node).is_err() {
                        panic!("Could not remove unnecessary text node");
                    }
                } else if let Some(last_child_node) = child_node.previous_sibling() {
                    let last_child_node = &mut last_child_node.clone();
                    if is_text(&last_child_node) {
                        if last_child_node
                            .append_data(&child_node.node_value().unwrap())
                            .is_err()
                        {
                            panic!("Could not merge text nodes");
                        }
                        if self.remove_child(child_node).is_err() {
                            panic!("Could not remove unnecessary text node");
                        }
                    }
                }
            }
        }
    }

    fn is_supported(&self, feature: &str, version: &str) -> bool {
        get_implementation().has_feature(feature, version)
    }

    fn has_attributes(&self) -> bool {
        !self.attributes().is_empty()
    }
}

// ------------------------------------------------------------------------------------------------

impl Notation for RefNode {
    fn public_id(&self) -> Option<String> {
        unwrap_extension_field!(self, Notation, i_public_id)
    }

    fn system_id(&self) -> Option<String> {
        unwrap_extension_field!(self, Notation, i_system_id)
    }
}

// ------------------------------------------------------------------------------------------------

impl ProcessingInstruction for RefNode {}

// ------------------------------------------------------------------------------------------------

impl Text for RefNode {
    fn split(&mut self, offset: usize) -> Result<RefNode> {
        let new_data = {
            let text = as_character_data_mut(self)?;
            let length = text.length();
            if offset >= length {
                String::new()
            } else {
                let count = length - offset;
                let new_data = text.substring_data(offset, count)?;
                text.delete_data(offset, count)?;
                new_data
            }
        };

        let new_node = {
            //
            // Create a new node and adjust contents
            //
            let mut_self = self.borrow_mut();
            match mut_self.i_node_type {
                NodeType::Text => {
                    let document = mut_self.i_owner_document.as_ref().unwrap();
                    Ok(NodeImpl::new_text(document.clone(), &new_data))
                }
                NodeType::CData => {
                    let document = mut_self.i_owner_document.as_ref().unwrap();
                    Ok(NodeImpl::new_cdata(document.clone(), &new_data))
                }
                _ => {
                    warn!("{}", MSG_INVALID_NODE_TYPE);
                    Err(Error::Syntax)
                }
            }?
        };

        let new_node = RefNode::new(new_node);
        if let Some(mut parent) = self.parent_node() {
            let _safe_to_ignore = parent.insert_before(new_node.clone(), self.next_sibling())?;
        }
        Ok(new_node)
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for RefNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        display::fmt_node(self, f)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

const WILD_CARD: &str = "*";

fn tag_name_match(test: &str, against: &str) -> bool {
    (test == against) || test == WILD_CARD || against == WILD_CARD
}

fn namespaced_name_match(
    test_ns: Option<&str>,
    test_local: &str,
    against_ns: &str,
    against_local: &str,
) -> bool {
    match test_ns {
        None => {
            against_ns == WILD_CARD
                && ((test_local == against_local)
                    || test_local == WILD_CARD
                    || against_local == WILD_CARD)
        }
        Some(test_ns) => {
            ((test_ns == against_ns) || test_ns == WILD_CARD || against_ns == WILD_CARD)
                && ((test_local == against_local)
                    || test_local == WILD_CARD
                    || against_local == WILD_CARD)
        }
    }
}

//
// From [https://www.w3.org/TR/DOM-Level-2-Core/core.html#ID-1590626202]
//
// The DOM presents documents as a hierarchy of Node objects that also implement other, more
// specialized interfaces. Some types of nodes may have child nodes of various types, and others
// are leaf nodes that cannot have anything below them in the document structure. For XML and HTML,
// the node types, and which node types they may have as children, are as follows:
//
// * Document -- Element (maximum of one), ProcessingInstruction, Comment, DocumentType (maximum of one)
// * DocumentFragment -- Element, ProcessingInstruction, Comment, Text, CDATASection, EntityReference
// * DocumentType -- no children
// * EntityReference -- Element, ProcessingInstruction, Comment, Text, CDATASection, EntityReference
// * Element -- Element, Text, Comment, ProcessingInstruction, CDATASection, EntityReference
// * Attr -- Text, EntityReference
// * ProcessingInstruction -- no children
// * Comment -- no children
// * Text -- no children
// * CDATASection -- no children
// * Entity -- Element, ProcessingInstruction, Comment, Text, CDATASection, EntityReference
// * Notation -- no children
//
fn is_child_allowed(parent: &RefNode, child: &RefNode) -> bool {
    let self_node_type = { &parent.borrow().i_node_type };
    let child_node_type = { &child.borrow().i_node_type };
    match self_node_type {
        NodeType::Element => match child_node_type {
            NodeType::Element
            | NodeType::Text
            | NodeType::Comment
            | NodeType::ProcessingInstruction
            | NodeType::CData
            | NodeType::EntityReference => true,
            _ => false,
        },
        NodeType::Attribute => match child_node_type {
            NodeType::Text | NodeType::EntityReference => true,
            _ => false,
        },
        NodeType::Text => false,
        NodeType::CData => false,
        NodeType::EntityReference => match child_node_type {
            NodeType::Element
            | NodeType::Text
            | NodeType::Comment
            | NodeType::ProcessingInstruction
            | NodeType::CData
            | NodeType::EntityReference => true,
            _ => false,
        },
        NodeType::Entity => match child_node_type {
            NodeType::Element
            | NodeType::Text
            | NodeType::Comment
            | NodeType::ProcessingInstruction
            | NodeType::CData
            | NodeType::EntityReference => true,
            _ => false,
        },
        NodeType::ProcessingInstruction => false,
        NodeType::Comment => false,
        NodeType::Document => match child_node_type {
            NodeType::Element | NodeType::Comment | NodeType::ProcessingInstruction => true,
            _ => false,
        },
        NodeType::DocumentType => false,
        NodeType::DocumentFragment => match child_node_type {
            NodeType::Element
            | NodeType::Text
            | NodeType::Comment
            | NodeType::ProcessingInstruction
            | NodeType::CData
            | NodeType::EntityReference => true,
            _ => false,
        },
        NodeType::Notation => false,
    }
}

pub(crate) fn create_document_with_options(
    namespace_uri: Option<&str>,
    qualified_name: Option<&str>,
    doc_type: Option<RefNode>,
    options: ProcessingOptions,
) -> Result<RefNode> {
    let node_impl = NodeImpl::new_document(doc_type, options);
    let mut document_node = RefNode::new(node_impl);

    //
    // If specified, create a new root element
    //
    let element: Option<RefNode> = {
        let ref_document = as_document(&document_node)?;
        match (namespace_uri, qualified_name) {
            (Some(namespace_uri), Some(qualified_name)) => {
                Some(ref_document.create_element_ns(namespace_uri, qualified_name)?)
            }
            (None, Some(qualified_name)) => Some(ref_document.create_element(qualified_name)?),
            (Some(_), None) => return Error::Namespace.into(),
            (None, None) => None,
        }
    };

    //
    // If successfully created, append root element. This can only be done once.
    //
    if let Some(element_node) = element {
        let document = as_document_mut(&mut document_node)?;
        let _safe_to_ignore = document.append_child(element_node)?;
    }

    Ok(document_node)
}
