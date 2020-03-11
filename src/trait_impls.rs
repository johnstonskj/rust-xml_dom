use crate::convert::*;
use crate::dom_impl::{get_implementation, Implementation};
use crate::error::{Error, Result};
use crate::name::Name;
use crate::node_impl::*;
use crate::syntax::*;
use crate::traits::*;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Node for RefNode {
    type NodeRef = RefNode;

    fn name(&self) -> Name {
        let ref_self = self.borrow();
        ref_self.i_name.clone()
    }

    fn node_value(&self) -> Option<String> {
        let ref_self = self.borrow();
        ref_self.i_value.clone()
    }

    fn set_node_value(&mut self, value: &str) -> Result<()> {
        let mut mut_self = self.borrow_mut();
        mut_self.i_value = Some(value.to_string());
        Ok(())
    }

    fn unset_node_value(&mut self) -> Result<()> {
        let mut mut_self = self.borrow_mut();
        mut_self.i_value = None;
        Ok(())
    }

    fn node_type(&self) -> NodeType {
        let ref_self = self.borrow();
        ref_self.i_node_type.clone()
    }

    fn parent_node(&self) -> Option<RefNode> {
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
        match ref_self.i_child_nodes.first() {
            None => None,
            Some(node) => Some(node.clone()),
        }
    }

    fn last_child(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        match ref_self.i_child_nodes.first() {
            None => None,
            Some(node) => Some(node.clone()),
        }
    }

    fn previous_sibling(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        match &ref_self.i_parent_node {
            None => None,
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
                            sibling.map(|n| n.clone())
                        }
                    }
                }
            }
        }
    }

    fn next_sibling(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        match &ref_self.i_parent_node {
            None => None,
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
                        sibling.map(|n| n.clone())
                    }
                }
            }
        }
    }

    fn attributes(&self) -> HashMap<Name, RefNode, RandomState> {
        let ref_self = self.borrow();
        ref_self.i_attributes.clone()
    }

    fn owner_document(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        match &ref_self.i_owner_document {
            None => None,
            Some(node) => node.clone().upgrade(),
        }
    }

    fn insert_before(&mut self, new_child: RefNode, ref_child: Option<RefNode>) -> Result<RefNode> {
        match ref_child {
            None => {
                let mut_node = as_element_mut(self)?;
                mut_node.append_child(new_child)
            }
            Some(ref_child) => {
                // TODO: see `append_child` for specifics
                let mut mut_self = self.borrow_mut();
                match mut_self
                    .i_child_nodes
                    .iter()
                    .position(|child| child == &ref_child)
                {
                    None => mut_self.i_child_nodes.push(new_child.clone()),
                    Some(position) => mut_self
                        .i_child_nodes
                        .insert(position + 1, new_child.clone()),
                }
                Ok(new_child)
            }
        }
    }

    fn replace_child(&mut self, _new_child: RefNode, _old_child: &RefNode) -> Result<RefNode> {
        // TODO: see `append_child` for specifics
        unimplemented!()
    }

    fn append_child(&mut self, new_child: RefNode) -> Result<RefNode> {
        // TODO: Check to see if it is in the tree already, if so remove it
        // update child with references
        {
            let mut mut_child = new_child.borrow_mut();
            mut_child.i_parent_node = Some(self.to_owned().downgrade());

            let ref_self = self.borrow();
            mut_child.i_document_element = ref_self.i_document_element.clone();
        }
        let mut mut_self = self.borrow_mut();

        // TODO: generalize, for each parent type, is child allowed
        let child_node_type = new_child.node_type();
        if mut_self.i_node_type == NodeType::Document && child_node_type == NodeType::Element {
            // a document may only have one child element
            mut_self.i_document_element = Some(new_child.clone());
        } else {
            mut_self.i_child_nodes.push(new_child.clone());
        }

        // TODO: deal with document fragment as special case

        Ok(new_child)
    }

    fn has_child_nodes(&self) -> bool {
        !self.child_nodes().is_empty()
    }

    fn clone_node(&self, _deep: bool) -> Option<RefNode> {
        unimplemented!()
    }

    fn normalize(&mut self) {
        unimplemented!()
    }

    fn is_supported(&self, feature: &str, version: &str) -> bool {
        get_implementation().has_feature(feature, version)
    }

    fn has_attributes(&self) -> bool {
        !self.attributes().is_empty()
    }
}

// ------------------------------------------------------------------------------------------------

impl Attribute for RefNode {}

// ------------------------------------------------------------------------------------------------

impl CDataSection for RefNode {}

// ------------------------------------------------------------------------------------------------

impl CharacterData for RefNode {
    fn substring(&self, offset: usize, count: usize) -> Result<String> {
        let ref_self = self.borrow();
        match &ref_self.i_value {
            None => Err(Error::IndexSize),
            Some(data) => Ok(data[offset..offset + count].to_string()),
        }
    }

    fn append(&mut self, new_data: &str) -> Result<()> {
        if !new_data.is_empty() {
            let mut mut_self = self.borrow_mut();
            match &mut_self.i_value {
                None => mut_self.i_value = Some(new_data.to_string()),
                Some(old_data) => mut_self.i_value = Some(format!("{}{}", old_data, new_data)),
            }
        }
        Ok(())
    }

    fn insert(&mut self, offset: usize, new_data: &str) -> Result<()> {
        if !new_data.is_empty() {
            let mut mut_self = self.borrow_mut();
            match &mut_self.i_value {
                None => {
                    if offset != 0 {
                        Err(Error::IndexSize)
                    } else {
                        mut_self.i_value = Some(new_data.to_string());
                        Ok(())
                    }
                }
                Some(old_data) => {
                    if offset >= old_data.len() {
                        Err(Error::IndexSize)
                    } else {
                        mut_self.i_value = Some(format!("{}{}", old_data, new_data));
                        Ok(())
                    }
                }
            }
        } else {
            Ok(())
        }
    }

    fn delete(&mut self, offset: usize, count: usize) -> Result<()> {
        const NOTHING: &str = "";
        self.replace(offset, count, NOTHING)
    }

    fn replace(&mut self, offset: usize, count: usize, replace_data: &str) -> Result<()> {
        if count > 0 {
            let mut mut_self = self.borrow_mut();
            match &mut_self.i_value {
                None => {
                    if offset != 0 {
                        Err(Error::IndexSize)
                    } else {
                        Ok(())
                    }
                }
                Some(old_data) => {
                    if offset >= old_data.len() {
                        Err(Error::IndexSize)
                    } else {
                        let mut new_data = old_data.clone();
                        new_data.replace_range(offset..offset + count, replace_data);
                        mut_self.i_value = Some(new_data);
                        Ok(())
                    }
                }
            }
        } else {
            Ok(())
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Comment for RefNode {}

// ------------------------------------------------------------------------------------------------

impl Document for RefNode {
    fn doc_type(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        ref_self.i_document_type.clone()
    }

    fn document_element(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        ref_self.i_document_element.clone()
    }

    fn implementation(&self) -> &dyn DOMImplementation<NodeRef = RefNode> {
        get_implementation()
    }

    fn create_attribute(&self, name: &str) -> Result<RefNode> {
        let name = Name::from_str(name)?;
        let node_impl = NodeImpl::new_attribute(name, None);
        Ok(RefNode::new(node_impl))
    }

    fn create_attribute_with(&self, name: &str, value: &str) -> Result<RefNode> {
        let name = Name::from_str(name)?;
        let node_impl = NodeImpl::new_attribute(name, Some(value));
        Ok(RefNode::new(node_impl))
    }

    fn create_attribute_ns(&self, namespace_uri: &str, qualified_name: &str) -> Result<RefNode> {
        let name = Name::new_ns(namespace_uri, qualified_name)?;
        let node_impl = NodeImpl::new_attribute(name, None);
        Ok(RefNode::new(node_impl))
    }

    fn create_cdata_section(&self, data: &str) -> Result<RefNode> {
        let node_impl = NodeImpl::new_cdata(data);
        Ok(RefNode::new(node_impl))
    }

    fn create_document_fragment(&self) -> Result<RefNode> {
        unimplemented!()
    }

    fn create_entity_reference(&self, _name: &str) -> Result<RefNode> {
        unimplemented!()
    }

    fn create_comment(&self, data: &str) -> RefNode {
        let node_impl = NodeImpl::new_comment(data);
        RefNode::new(node_impl)
    }

    fn create_element(&self, tag_name: &str) -> Result<RefNode> {
        let name = Name::from_str(tag_name)?;
        let node_impl = NodeImpl::new_element(name);
        Ok(RefNode::new(node_impl))
    }

    fn create_element_ns(&self, namespace_uri: &str, qualified_name: &str) -> Result<RefNode> {
        let name = Name::new_ns(namespace_uri, qualified_name)?;
        let node_impl = NodeImpl::new_element(name);
        Ok(RefNode::new(node_impl))
    }

    fn create_processing_instruction(&self, target: &str, data: Option<&str>) -> Result<RefNode> {
        let target = Name::from_str(target)?;
        let node_impl = NodeImpl::new_processing_instruction(target, data);
        Ok(RefNode::new(node_impl))
    }

    fn create_text_node(&self, data: &str) -> RefNode {
        let node_impl = NodeImpl::new_text(data);
        RefNode::new(node_impl)
    }

    fn get_element_by_id(&self, _id: &str) -> Option<RefNode> {
        //
        // Until we are schema/data-type aware we do not know which attributes are actually XML
        // identifiers. A common, but erroneous, implementation is to look for attributes named
        // "id", we aren't going to do that. It may be possible to make that a flag on
        // implementation for those that want the lax behavior.
        //
        None
    }

    fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<RefNode> {
        //
        // Delegate this call to the document element
        //
        let ref_self = self.borrow();
        match &ref_self.i_document_element {
            None => Vec::default(),
            Some(root_node) => {
                let root_element = as_element(root_node).expect("invalid node type");
                root_element.get_elements_by_tag_name(tag_name)
            }
        }
    }

    fn get_elements_by_tag_name_ns(&self, namespace_uri: &str, local_name: &str) -> Vec<RefNode> {
        //
        // Delegate this call to the document element
        //
        let ref_self = self.borrow();
        match &ref_self.i_document_element {
            None => Vec::default(),
            Some(root_node) => {
                let root_element = as_element(root_node).expect("invalid node type");
                root_element.get_elements_by_tag_name_ns(namespace_uri, local_name)
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl DocumentType for RefNode {
    fn public_id(&self) -> Option<String> {
        let ref_self = self.borrow();
        match ref_self.i_attributes.get(&Name::for_public_id()) {
            None => None,
            Some(ref_node) => {
                let ref_node = ref_node.borrow();
                ref_node.i_value.clone()
            }
        }
    }

    fn system_id(&self) -> Option<String> {
        let ref_self = self.borrow();
        match ref_self.i_attributes.get(&Name::for_system_id()) {
            None => None,
            Some(ref_node) => {
                let ref_node = ref_node.borrow();
                ref_node.i_value.clone()
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl DOMImplementation for Implementation {
    type NodeRef = RefNode;

    fn create_document(
        &self,
        namespace_uri: &str,
        qualified_name: &str,
        doc_type: Option<RefNode>,
    ) -> Result<RefNode> {
        let name = Name::new_ns(namespace_uri, qualified_name)?;
        let node_impl = NodeImpl::new_document(name, doc_type);
        let mut document_node = RefNode::new(node_impl);
        let document =
            as_document_mut(&mut document_node).expect("could not cast node to Document");
        let element = document.create_element_ns(namespace_uri, qualified_name)?;
        let _dont_care = document.append_child(element)?;
        Ok(document_node)
    }

    fn create_document_type(
        &self,
        qualified_name: &str,
        public_id: &str,
        system_id: &str,
    ) -> Result<RefNode> {
        let name = Name::from_str(qualified_name)?;
        let node_impl = NodeImpl::new_document_type(name, public_id, system_id);
        Ok(RefNode::new(node_impl))
    }

    fn has_feature(&self, feature: &str, version: &str) -> bool {
        ((feature == XML_FEATURE_CORE || feature == XML_FEATURE_XML) && (version == XML_FEATURE_V1)
            || (feature == XML_FEATURE_CORE && version == XML_FEATURE_V2))
    }
}

// ------------------------------------------------------------------------------------------------

impl Element for RefNode {
    fn get_attribute(&self, name: &str) -> Option<String> {
        if !is_element(self) {
            // shortcut as only elements have attributes
            None
        } else {
            match Name::from_str(name) {
                Ok(attr_name) => {
                    let self_copy = self.clone();
                    let self_copy = self_copy.borrow();
                    match self_copy.i_attributes.get(&attr_name) {
                        None => None,
                        Some(attr_node) => {
                            let attribute = attr_node.borrow();
                            match &attribute.i_value {
                                None => None,
                                Some(value) => Some(value.clone()),
                            }
                        }
                    }
                }
                Err(_) => None,
            }
        }
    }

    fn set_attribute(&mut self, name: &str, value: &str) -> Result<()> {
        let attr_name = Name::from_str(name)?;
        let attr_node = NodeImpl::new_attribute(attr_name, Some(value));
        self.set_attribute_node(RefNode::new(attr_node)).map(|_| ())
    }

    fn remove_attribute(&mut self, _name: &str) -> Result<()> {
        if !is_element(self) {
            // shortcut as only elements have attributes
            Ok(())
        } else {
            // TODO: deal with namespaces
            unimplemented!()
        }
    }

    fn get_attribute_node(&self, _name: &str) -> Option<RefNode> {
        if !is_element(self) {
            // shortcut as only elements have attributes
            None
        } else {
            unimplemented!()
        }
    }

    fn set_attribute_node(&mut self, new_attribute: RefNode) -> Result<RefNode> {
        // ENSURE: You can ONLY add attributes to an element
        if !is_element(self) || !is_attribute(&new_attribute) {
            Err(Error::InvalidState)
        } else {
            let name: Name = new_attribute.name();
            if name.is_namespace_attribute() {
                let attribute = as_attribute(&new_attribute).unwrap();
                let namespace_uri = attribute.value().unwrap();

                let as_namespaced = as_element_namespaced_mut(self).unwrap();
                let _ignore = match &name.prefix() {
                    None => as_namespaced.insert(None, &namespace_uri),
                    Some(prefix) => as_namespaced.insert(Some(prefix), &namespace_uri),
                }?;
            }
            let mut mut_self = self.borrow_mut();
            let _safe_to_ignore = mut_self
                .i_attributes
                .insert(new_attribute.name(), new_attribute.clone());
            Ok(new_attribute)
        }
    }

    fn remove_attribute_node(&mut self, _old_attribute: RefNode) -> Result<RefNode> {
        unimplemented!()
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
                    Err(_) => (),
                }
            }
        }
        results
    }

    fn get_attribute_ns(&self, _namespace_uri: &str, _local_name: &str) -> Option<String> {
        if !is_element(self) {
            // shortcut as only elements have attributes
            None
        } else {
            unimplemented!()
        }
    }

    fn set_attribute_ns(
        &mut self,
        namespace_uri: &str,
        qualified_name: &str,
        value: &str,
    ) -> Result<()> {
        let attr_name = Name::new_ns(namespace_uri, qualified_name)?;
        let attr_node = NodeImpl::new_attribute(attr_name, Some(value));
        self.set_attribute_node(RefNode::new(attr_node)).map(|_| ())
    }

    fn remove_attribute_ns(&mut self, _namespace_uri: &str, _local_name: &str) -> Result<()> {
        unimplemented!()
    }

    fn get_attribute_node_ns(&self, namespace_uri: &str, local_name: &str) -> Option<RefNode> {
        if is_element(self) {
            match Name::new_ns(namespace_uri, local_name) {
                Ok(name) => {
                    let ref_self = self.borrow();
                    ref_self.i_attributes.get(&name).map(|n| n.clone())
                }
                Err(_) => None,
            }
        } else {
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
                ref_self.i_name.namespace_uri(),
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
                    Err(_) => (),
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
                    ref_self.i_attributes.contains_key(&name)
                }
                Err(_) => false,
            }
        } else {
            false
        }
    }

    fn has_attribute_ns(&self, namespace_uri: &str, local_name: &str) -> bool {
        if is_element(self) {
            match Name::new_ns(namespace_uri, local_name) {
                Ok(name) => {
                    let ref_self = self.borrow();
                    ref_self.i_attributes.contains_key(&name)
                }
                Err(_) => false,
            }
        } else {
            false
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl ProcessingInstruction for RefNode {}

// ------------------------------------------------------------------------------------------------

impl Text for RefNode {
    fn split(&mut self, offset: usize) -> Result<RefNode> {
        let new_data = {
            let text = as_text_mut(self).unwrap();
            let count = text.length() - (offset + 1);
            let new_data = text.substring(offset, count)?;
            text.delete(offset, count)?;
            new_data
        };

        let mut_self = self.borrow_mut();
        let mut new_node = match mut_self.i_node_type {
            NodeType::Text => Ok(NodeImpl::new_text(&new_data)),
            NodeType::CData => Ok(NodeImpl::new_cdata(&new_data)),
            _ => Err(Error::HierarchyRequest),
        }?;
        Ok(match &mut_self.i_parent_node {
            None => RefNode::new(new_node),
            Some(parent) => {
                new_node.i_parent_node = Some(parent.clone());
                let new_node = RefNode::new(new_node);
                let parent = parent.clone();
                let mut parent = parent.upgrade().unwrap();
                let parent_element = as_element_mut(&mut parent)?;
                let self_node = as_element(self)?;
                let _ignored =
                    parent_element.insert_before(new_node.clone(), self_node.next_sibling())?;
                new_node
            }
        })
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for RefNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.node_type() {
            NodeType::Element => {
                let element = self as &dyn Element<NodeRef = RefNode>;
                write!(f, "{}{}", XML_ELEMENT_START_START, element.name())?;
                for attr in element.attributes().values() {
                    write!(f, " {}", attr.to_string())?;
                }
                write!(f, "{}", XML_ELEMENT_START_END)?;
                for child in element.child_nodes() {
                    write!(f, "{}", child.to_string())?;
                }
                write!(
                    f,
                    "{}{}{}",
                    XML_ELEMENT_END_START,
                    element.name(),
                    XML_ELEMENT_END_END
                )
            }
            NodeType::Attribute => {
                let attribute = self as &dyn Attribute<NodeRef = RefNode>;
                write!(f, "{}=\"{}\"", attribute.name(), attribute.value().unwrap())
            }
            NodeType::Text => {
                let char_data = self as &dyn CharacterData<NodeRef = RefNode>;
                match char_data.data() {
                    None => Ok(()),
                    Some(data) => write!(f, "{}", data),
                }
            }
            NodeType::CData => {
                let char_data = self as &dyn CharacterData<NodeRef = RefNode>;
                match char_data.data() {
                    None => Ok(()),
                    Some(data) => write!(f, "{} {} {}", XML_COMMENT_START, data, XML_COMMENT_END),
                }
            }
            NodeType::ProcessingInstruction => {
                let pi = self as &dyn ProcessingInstruction<NodeRef = RefNode>;
                match pi.data() {
                    None => write!(f, "{}{}{}", XML_PI_START, pi.target(), XML_PI_END),
                    Some(data) => {
                        write!(f, "{}{} {}{}", XML_PI_START, pi.target(), data, XML_PI_END)
                    }
                }
            }
            NodeType::Comment => {
                let char_data = self as &dyn CharacterData<NodeRef = RefNode>;
                match char_data.data() {
                    None => Ok(()),
                    Some(data) => write!(f, "{}{}{}", XML_CDATA_START, data, XML_CDATA_END),
                }
            }
            NodeType::Document => {
                let document = self as &dyn Document<NodeRef = RefNode>;
                match document.doc_type() {
                    None => (),
                    Some(doc_type) => write!(f, "{}", doc_type)?,
                }
                for child in self.child_nodes() {
                    write!(f, "{}", child.to_string())?;
                }
                match document.document_element() {
                    None => Ok(()),
                    Some(document_element) => write!(f, "{}", document_element),
                }
            }
            NodeType::DocumentType => {
                let doc_type = self as &dyn DocumentType<NodeRef = RefNode>;
                write!(f, "{} {}", XML_DOCTYPE_START, doc_type.name())?;
                match &doc_type.public_id() {
                    None => (),
                    Some(id) => {
                        write!(f, " {} \"{}\"", XML_DOCTYPE_PUBLIC, id)?;
                    }
                }
                match &doc_type.system_id() {
                    None => (),
                    Some(id) => {
                        write!(f, " {} \"{}\"", XML_DOCTYPE_SYSTEM, id)?;
                    }
                }
                write!(f, "{}", XML_DOCTYPE_END)
            }
            _ => Ok(()),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

const WILD_CARD: &str = "*";

fn tag_name_match(test: &String, against: &String) -> bool {
    let wild = &WILD_CARD.to_string();
    (test == against) || test == wild || against == wild
}

fn namespaced_name_match(
    test_ns: &Option<String>,
    test_local: &String,
    against_ns: &String,
    against_local: &String,
) -> bool {
    let wild = &WILD_CARD.to_string();
    match test_ns {
        None => {
            against_ns == wild
                && ((test_local == against_local) || test_local == wild || against_local == wild)
        }
        Some(test_ns) => {
            ((test_ns == against_ns) || test_ns == wild || against_ns == wild)
                && ((test_local == against_local) || test_local == wild || against_local == wild)
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_node(name: &str) -> RefNode {
        let name = Name::from_str(name).unwrap();
        let node = NodeImpl::new_element(name);
        RefNode::new(node)
    }

    #[test]
    fn test_next_sibling() {
        //
        // Setup the tree
        //
        let mut root_node = make_node("element");
        {
            let root_element = as_element_mut(&mut root_node).unwrap();

            for index in 1..6 {
                let child_node = make_node(&format!("child-{}", index));
                let _ignore = root_element.append_child(child_node.clone());
            }
        }

        {
            assert_eq!(root_node.borrow().i_child_nodes.len(), 5);
        }

        //
        // Ask for siblings
        //
        let ref_root = root_node.borrow();
        let mid_node = ref_root.i_child_nodes.get(2).unwrap();
        let ref_mid = as_element(mid_node).unwrap();
        assert_eq!(ref_mid.name().to_string(), "child-3".to_string());

        let next_node = ref_mid.next_sibling().unwrap();
        let ref_next = as_element(&next_node).unwrap();
        assert_eq!(ref_next.name().to_string(), "child-4".to_string());

        let last_node = ref_next.next_sibling().unwrap();
        let ref_last = as_element(&last_node).unwrap();
        assert_eq!(ref_last.name().to_string(), "child-5".to_string());

        let no_node = ref_last.next_sibling();
        assert!(no_node.is_none());
    }

    #[test]
    fn test_previous_sibling() {
        //
        // Setup the tree
        //
        let mut root_node = make_node("element");
        {
            let root_element = as_element_mut(&mut root_node).unwrap();

            for index in 1..6 {
                let child_node = make_node(&format!("child-{}", index));
                let _ignore = root_element.append_child(child_node.clone());
            }
        }

        {
            assert_eq!(root_node.borrow().i_child_nodes.len(), 5);
        }

        //
        // Ask for siblings
        //
        let ref_root = root_node.borrow();
        let mid_node = ref_root.i_child_nodes.get(2).unwrap();
        let ref_mid = as_element(mid_node).unwrap();
        assert_eq!(ref_mid.name().to_string(), "child-3".to_string());

        let previous_node = ref_mid.previous_sibling().unwrap();
        let ref_previous = as_element(&previous_node).unwrap();
        assert_eq!(ref_previous.name().to_string(), "child-2".to_string());

        let first_node = ref_previous.previous_sibling().unwrap();
        let ref_first = as_element(&first_node).unwrap();
        assert_eq!(ref_first.name().to_string(), "child-1".to_string());

        let no_node = ref_first.previous_sibling();
        assert!(no_node.is_none());
    }
}
