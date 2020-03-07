use self::super::error::{Error, Result};
use self::super::name::Name;
use self::super::rc_cell::*;
use self::super::syntax::*;
use self::super::traits::*;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Node for RefNode {
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
        unimplemented!()
    }

    fn next_sibling(&self) -> Option<RefNode> {
        unimplemented!()
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

    fn insert_before(&mut self, new_child: RefNode, ref_child: &RefNode) -> Result<RefNode> {
        // TODO: see `append_child` for specifics
        let mut mut_self = self.borrow_mut();
        match mut_self
            .i_child_nodes
            .iter()
            .position(|child| child == ref_child)
        {
            None => mut_self.i_child_nodes.push(new_child.clone()),
            Some(position) => mut_self.i_child_nodes.insert(position, new_child.clone()),
        }
        Ok(new_child)
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
        self.replace(offset, count, "")
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

    fn implementation(&self) -> &dyn DOMImplementation {
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
        None
    }

    fn get_elements_by_tag_name(&self, _tag_name: &str) -> Vec<RefNode> {
        unimplemented!()
    }

    fn get_elements_by_tag_name_ns(&self, _namespace_uri: &str, _local_name: &str) -> Vec<RefNode> {
        unimplemented!()
    }
}

// ------------------------------------------------------------------------------------------------

impl DocumentType for RefNode {
    fn public_id(&self) -> Option<String> {
        let as_element = self as &dyn Element;
        as_element.get_attribute(XML_DOCTYPE_PUBLIC)
    }

    fn system_id(&self) -> Option<String> {
        let as_element = self as &dyn Element;
        as_element.get_attribute(XML_DOCTYPE_SYSTEM)
    }
}

// ------------------------------------------------------------------------------------------------

impl Element for RefNode {
    fn get_attribute(&self, name: &str) -> Option<String> {
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

    fn set_attribute(&mut self, name: &str, value: &str) -> Result<()> {
        let attr_name = Name::from_str(name)?;
        let attr_node = NodeImpl::new_attribute(attr_name, Some(value));
        self.set_attribute_node(RefNode::new(attr_node)).map(|_| ())
    }

    fn remove_attribute(&mut self, name: &str) -> Result<()> {
        let _attr_name = Name::from_str(name)?;
        unimplemented!()
    }

    fn get_attribute_node(&self, _name: &str) -> Option<RefNode> {
        unimplemented!()
    }

    fn set_attribute_node(&mut self, new_attribute: RefNode) -> Result<RefNode> {
        let mut mut_self = self.borrow_mut();
        let _safe_to_ignore = mut_self
            .i_attributes
            .insert(new_attribute.name(), new_attribute.clone());
        Ok(new_attribute)
    }

    fn remove_attribute_node(&mut self, _old_attribute: RefNode) -> Result<RefNode> {
        unimplemented!()
    }

    fn get_elements_by_tag_name(&self, _tag_name: &str) -> Vec<RefNode> {
        unimplemented!()
    }

    fn get_attribute_ns(&self, _namespace_uri: &str, _local_name: &str) -> Option<String> {
        unimplemented!()
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

    fn get_attribute_node_ns(&self, _namespace_uri: &str, _local_name: &str) -> Option<RefNode> {
        unimplemented!()
    }

    fn set_attribute_node_ns(&mut self, new_attribute: RefNode) -> Result<RefNode> {
        self.set_attribute_node(new_attribute)
    }

    fn get_elements_by_tag_name_ns(&self, _namespace_uri: &str, _local_name: &str) -> Vec<RefNode> {
        unimplemented!()
    }

    fn has_attribute(&self, _name: &str) -> bool {
        unimplemented!()
    }

    fn has_attribute_ns(&self, _namespace_uri: &str, _local_name: &str) -> bool {
        unimplemented!()
    }
}

// ------------------------------------------------------------------------------------------------

impl ProcessingInstruction for RefNode {}

// ------------------------------------------------------------------------------------------------

impl Text for RefNode {
    fn split(&self, _offset: usize) -> Result<RefNode> {
        unimplemented!()
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for RefNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.node_type() {
            NodeType::Element => {
                let element = self as &dyn Element;
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
                let attribute = self as &dyn Attribute;
                write!(
                    f,
                    "\"{}\"=\"{}\"",
                    attribute.name(),
                    attribute.value().unwrap()
                )
            }
            NodeType::Text => {
                let char_data = self as &dyn CharacterData;
                match char_data.data() {
                    None => write!(f, ""),
                    Some(data) => write!(f, "{}", data),
                }
            }
            NodeType::CData => {
                let char_data = self as &dyn CharacterData;
                match char_data.data() {
                    None => write!(f, ""),
                    Some(data) => write!(f, "{} {} {}", XML_COMMENT_START, data, XML_COMMENT_END),
                }
            }
            NodeType::ProcessingInstruction => {
                let pi = self as &dyn ProcessingInstruction;
                match pi.data() {
                    None => write!(f, "{}{}{}>", XML_PI_START, self.target(), XML_PI_END),
                    Some(data) => {
                        write!(f, "{}{} {}{}>", XML_PI_START, pi.target(), data, XML_PI_END)
                    }
                }
            }
            NodeType::Comment => {
                let char_data = self as &dyn CharacterData;
                match char_data.data() {
                    None => write!(f, ""),
                    Some(data) => write!(f, "{}{}{}", XML_CDATA_START, data, XML_CDATA_END),
                }
            }
            NodeType::Document => {
                for child in self.child_nodes() {
                    write!(f, "{}", child.to_string())?;
                }
                let document = self as &dyn Document;
                match document.document_element() {
                    None => write!(f, ""),
                    Some(document_element) => write!(f, "{}", document_element),
                }
            }
            NodeType::DocumentType => {
                let doc_type = self as &dyn DocumentType;
                write!(
                    f,
                    "{} {} {} {} {}",
                    XML_DOCTYPE_START,
                    doc_type.name(),
                    match doc_type.public_id() {
                        None => "".to_string(),
                        Some(public_id) => format!("{} {}", XML_DOCTYPE_PUBLIC, public_id),
                    },
                    match doc_type.system_id() {
                        None => "".to_string(),
                        Some(system_id) => format!("{} {}", XML_DOCTYPE_SYSTEM, system_id),
                    },
                    XML_DOCTYPE_END
                )
            }
            _ => write!(f, ""),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl NodeImpl {
    pub(crate) fn new_element(name: Name) -> Self {
        Self {
            i_node_type: NodeType::Element,
            i_name: name,
            i_value: None,
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub(crate) fn new_attribute(name: Name, value: Option<&str>) -> Self {
        Self {
            i_node_type: NodeType::Attribute,
            i_name: name,
            i_value: value.map(|v| v.to_string()),
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub(crate) fn new_text(data: &str) -> Self {
        Self {
            i_node_type: NodeType::Text,
            i_name: Name::for_text(),
            i_value: Some(data.to_string()),
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub(crate) fn new_cdata(data: &str) -> Self {
        Self {
            i_node_type: NodeType::CData,
            i_name: Name::for_cdata(),
            i_value: Some(data.to_string()),
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub(crate) fn new_processing_instruction(target: Name, data: Option<&str>) -> Self {
        Self {
            i_node_type: NodeType::ProcessingInstruction,
            i_name: target,
            i_value: data.map(|v| v.to_string()),
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub(crate) fn new_comment(data: &str) -> Self {
        Self {
            i_node_type: NodeType::Comment,
            i_name: Name::for_cdata(),
            i_value: Some(data.to_string()),
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub(crate) fn new_document(name: Name, doc_type: Option<RefNode>) -> Self {
        Self {
            i_node_type: NodeType::Document,
            i_name: name,
            i_value: None,
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: doc_type,
        }
    }
    pub(crate) fn new_document_type(name: Name, public_id: &str, system_id: &str) -> Self {
        let new_doc_type = Self {
            i_node_type: NodeType::DocumentType,
            i_name: name,
            i_value: None,
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        };
        let mut ref_node: RefNode = RcRefCell::new(new_doc_type);
        let as_element = &mut ref_node as &mut dyn Element;
        as_element
            .set_attribute(XML_DOCTYPE_PUBLIC, public_id)
            .expect("invalid public ID");
        as_element
            .set_attribute(XML_DOCTYPE_SYSTEM, system_id)
            .expect("invalid system ID");
        ref_node.unwrap()
    }
}

// ------------------------------------------------------------------------------------------------

///
/// Internal use only
///
#[doc(hidden)]
#[derive(Clone, Debug)]
struct Implementation {}

impl DOMImplementation for Implementation {
    fn create_document(
        &self,
        namespace_uri: &str,
        qualified_name: &str,
        doc_type: Option<RefNode>,
    ) -> Result<RefNode> {
        let name = Name::new_ns(namespace_uri, qualified_name)?;
        let node_impl = NodeImpl::new_document(name, doc_type);
        Ok(RefNode::new(node_impl))
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
        (feature == XML_FEATURE_CORE || feature == XML_FEATURE_XML)
            && (version == XML_FEATURE_V1 || version == XML_FEATURE_V2)
    }
}

const THIS_IMPLEMENTATION: &'static dyn DOMImplementation = &Implementation {};

///
/// Return a reference to an instance of this `DOMImplementation` implementation.
///
/// This function gets around the DOM bootstrap issue, the `implementation` method on the
/// [`Document`](trait.Document.html) trait requires an instance of `Document`; however, the
/// `create_document` method on `DOMImplementation` requires an instance from `implementation`.
///
/// Note that Java, for example, solves the bootstrap problem with a factory and builder pattern:
///
/// ```java
/// DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
/// DocumentBuilder builder = factory.newDocumentBuilder();
/// Document doc = builder.newDocument();
/// ```
///
pub fn get_implementation() -> &'static dyn DOMImplementation {
    THIS_IMPLEMENTATION
}
