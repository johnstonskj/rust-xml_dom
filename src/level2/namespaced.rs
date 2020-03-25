use crate::level2::node_impl::Extension;
use crate::level2::node_impl::RefNode;
use crate::level2::traits::{Element, NodeType};
use crate::shared::error::{
    Error, Result, MSG_INVALID_EXTENSION, MSG_INVALID_NODE_TYPE, MSG_WEAK_REF,
};

// ------------------------------------------------------------------------------------------------
// Public Types
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
pub trait Namespaced: Element {
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
    fn get_prefix(&self, namespace_uri: &str) -> Option<Option<String>>;
    ///
    /// Returns the prefix associated with the provided `namespace_uri` for this element by looking
    /// up the DOM tree through `parent_node` links. Returns `None` if the namespace is not mapped
    /// with a prefix for this, or any parent, element.
    ///  
    fn resolve_prefix(&self, namespace_uri: &str) -> Option<Option<String>>;
}

#[doc(hidden)]
pub(crate) trait MutNamespaced: Namespaced {
    fn insert_mapping(
        &mut self,
        prefix: Option<&str>,
        namespace_uri: &str,
    ) -> Result<Option<String>>;
    fn remove_mapping(&mut self, prefix: Option<&str>) -> Result<Option<String>>;
    fn normalize_mappings(&mut self) -> Result<()>;
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Namespaced for RefNode {
    fn contains_mapping(&self, prefix: Option<&str>) -> bool {
        let ref_self = self.borrow();
        if ref_self.i_node_type == NodeType::Element {
            if let Extension::Element { i_namespaces, .. } = &ref_self.i_extension {
                i_namespaces.contains_key(&prefix.map(String::from))
            } else {
                warn!("{}", MSG_INVALID_EXTENSION);
                false
            }
        } else {
            warn!("{}", MSG_INVALID_NODE_TYPE);
            false
        }
    }

    fn get_namespace(&self, prefix: Option<&str>) -> Option<String> {
        let ref_self = self.borrow();
        if ref_self.i_node_type == NodeType::Element {
            if let Extension::Element { i_namespaces, .. } = &ref_self.i_extension {
                let value = i_namespaces.get(&prefix.map(String::from));
                value.map(String::to_string)
            } else {
                warn!("{}", MSG_INVALID_EXTENSION);
                None
            }
        } else {
            warn!("{}", MSG_INVALID_NODE_TYPE);
            None
        }
    }

    fn resolve_namespace(&self, prefix: Option<&str>) -> Option<String> {
        match self.get_namespace(prefix) {
            None => {
                let ref_self = self.borrow();
                match &ref_self.i_parent_node {
                    None => None,
                    Some(parent) => {
                        let parent = parent.clone();
                        let parent_node = parent.upgrade().expect(MSG_WEAK_REF);
                        parent_node.resolve_namespace(prefix)
                    }
                }
            }
            found => found,
        }
    }

    fn contains_mapped_namespace(&self, namespace_uri: &str) -> bool {
        self.get_prefix(namespace_uri).is_some()
    }

    fn get_prefix(&self, namespace_uri: &str) -> Option<Option<String>> {
        let ref_self = self.borrow();
        if ref_self.i_node_type == NodeType::Element {
            if let Extension::Element { i_namespaces, .. } = &ref_self.i_extension {
                let ns = namespace_uri.to_string();
                let value = i_namespaces.iter().find(|(_, v)| **v == ns);
                match value {
                    None => None,
                    Some((k, _)) => Some(k.as_ref().map(String::from)),
                }
            } else {
                warn!("{}", MSG_INVALID_EXTENSION);
                None
            }
        } else {
            warn!("{}", MSG_INVALID_NODE_TYPE);
            None
        }
    }

    fn resolve_prefix(&self, namespace_uri: &str) -> Option<Option<String>> {
        match self.get_prefix(namespace_uri) {
            None => {
                let ref_self = self.borrow();
                match &ref_self.i_parent_node {
                    None => None,
                    Some(parent) => {
                        let parent = parent.clone();
                        let parent_node = parent.upgrade().expect(MSG_WEAK_REF);
                        parent_node.resolve_prefix(namespace_uri)
                    }
                }
            }
            found => found,
        }
    }
}

impl MutNamespaced for RefNode {
    fn insert_mapping(
        &mut self,
        prefix: Option<&str>,
        namespace_uri: &str,
    ) -> Result<Option<String>> {
        let mut mut_self = self.borrow_mut();
        if mut_self.i_node_type == NodeType::Element {
            if let Extension::Element { i_namespaces, .. } = &mut mut_self.i_extension {
                Ok(i_namespaces.insert(prefix.map(String::from), namespace_uri.to_string()))
            } else {
                warn!("{}", MSG_INVALID_EXTENSION);
                Err(Error::InvalidState)
            }
        } else {
            warn!("{}", MSG_INVALID_NODE_TYPE);
            Err(Error::InvalidState)
        }
    }

    fn remove_mapping(&mut self, prefix: Option<&str>) -> Result<Option<String>> {
        let mut mut_self = self.borrow_mut();
        if mut_self.i_node_type == NodeType::Element {
            if let Extension::Element { i_namespaces, .. } = &mut mut_self.i_extension {
                Ok(i_namespaces.remove(&prefix.map(String::from)))
            } else {
                warn!("{}", MSG_INVALID_EXTENSION);
                Err(Error::InvalidState)
            }
        } else {
            warn!("{}", MSG_INVALID_NODE_TYPE);
            Err(Error::InvalidState)
        }
    }

    fn normalize_mappings(&mut self) -> Result<()> {
        // TODO: ensure this element has a mapping for it's own namespace and for any namespaced attributes
        unimplemented!()
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::level2::convert::{
        as_document_mut, as_element_mut, as_element_namespaced, as_element_namespaced_mut,
        MutRefNamespaced, RefNamespaced,
    };
    use crate::level2::{get_implementation, RefNode};

    const HTML: &str = "http://www.w3.org/1999/xhtml";
    const XSD: &str = "http://www.w3.org/2001/XMLSchema";
    const XSLT: &str = "http://www.w3.org/1999/XSL/Transform";
    const EX: &str = "http://example.org/xmlns/example";

    fn make_document_node() -> RefNode {
        get_implementation()
            .create_document(Some("http://example.org/"), Some("root"), None)
            .unwrap()
    }

    fn make_node(document: &mut RefNode, name: &str) -> RefNode {
        let document = as_document_mut(document).unwrap();
        let element = document.create_element(name).unwrap();
        let mut document_element = document.document_element().unwrap();
        let document_element = as_element_mut(&mut document_element).unwrap();
        let result = document_element.append_child(element.clone());
        println!("{:#?}", result);
        assert!(result.is_ok());
        element
    }

    #[test]
    fn test_empty_element() {
        let mut document = make_document_node();
        let ref_node = make_node(&mut document, "element");
        let namespaced = as_element_namespaced(&ref_node).unwrap();

        // prefix
        assert!(!namespaced.contains_mapping(None));
        assert!(!namespaced.contains_mapping(Some("s")));
        assert!(namespaced.get_namespace(None).is_none());
        assert!(namespaced.get_namespace(Some("s")).is_none());
        assert!(namespaced.resolve_namespace(None).is_none());
        assert!(namespaced.resolve_namespace(Some("s")).is_none());

        // namespace
        assert!(!namespaced.contains_mapped_namespace(HTML));
        assert!(namespaced.get_prefix(HTML).is_none());
        assert!(namespaced.resolve_prefix(HTML).is_none());
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_this_element_string_prefix() {
        let mut document = make_document_node();
        let mut ref_node = make_node(&mut document, "element");
        let namespaced = &mut ref_node as MutRefNamespaced<'_>;

        namespaced.insert_mapping(Some("xsd"), XSD);

        // prefix
        let ns_result = Some(XSD.to_string());

        assert_eq!(namespaced.contains_mapping(None), false);
        assert_eq!(namespaced.contains_mapping(Some("xsd")), true);
        assert_eq!(namespaced.get_namespace(None), None);
        assert_eq!(namespaced.get_namespace(Some("xsd")), ns_result);
        assert_eq!(namespaced.resolve_namespace(None), None);
        assert_eq!(namespaced.resolve_namespace(Some("xsd")), ns_result);

        // namespace
        let prefix_result = Some(Some("xsd".to_string()));

        assert_eq!(namespaced.contains_mapped_namespace(HTML), false);
        assert_eq!(namespaced.contains_mapped_namespace(XSD), true);
        assert_eq!(namespaced.get_prefix(XSD), prefix_result);
        assert_eq!(namespaced.resolve_prefix(XSD), prefix_result);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_this_element_none_prefix() {
        let mut document = make_document_node();
        let mut ref_node = make_node(&mut document, "element");
        let namespaced = &mut ref_node as MutRefNamespaced<'_>;

        namespaced.insert_mapping(None, XSD);

        // prefix
        let ns_result = Some(XSD.to_string());

        assert_eq!(namespaced.contains_mapping(None), true);
        assert_eq!(namespaced.contains_mapping(Some("xsd")), false);
        assert_eq!(namespaced.get_namespace(None), ns_result);
        assert_eq!(namespaced.get_namespace(Some("xsd")), None);
        assert_eq!(namespaced.resolve_namespace(None), ns_result);
        assert_eq!(namespaced.resolve_namespace(Some("xsd")), None);

        // namespace
        let prefix_result = Some(None);

        assert_eq!(namespaced.contains_mapped_namespace(HTML), false);
        assert_eq!(namespaced.contains_mapped_namespace(XSD), true);
        assert_eq!(namespaced.get_prefix(XSD), prefix_result);
        assert_eq!(namespaced.resolve_prefix(XSD), prefix_result);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_tree_resolve() {
        let mut document = make_document_node();
        //
        // Setup the tree
        //
        let mut ref_node = make_node(&mut document, "element");
        let ref_root = as_element_namespaced_mut(&mut ref_node).unwrap();
        ref_root.insert_mapping(Some("xsd"), XSD);

        let mut ref_teen_1 = make_node(&mut document, "teen1");
        {
            let ref_teen_ns = as_element_namespaced_mut(&mut ref_teen_1).unwrap();
            ref_teen_ns.insert_mapping(None, EX);
        }
        ref_root.append_child(ref_teen_1.clone());

        let mut ref_teen_2 = make_node(&mut document, "teen2");
        {
            let ref_teen_ns = as_element_namespaced_mut(&mut ref_teen_2).unwrap();
            ref_teen_ns.insert_mapping(None, HTML);
        }
        ref_root.append_child(ref_teen_2.clone());

        let mut ref_child = make_node(&mut document, "child");
        {
            let ref_child_ns = as_element_namespaced_mut(&mut ref_child).unwrap();
            ref_child_ns.insert_mapping(Some("xslt"), XSLT);
        }
        {
            let ref_teen = as_element_namespaced_mut(&mut ref_teen_2).unwrap();
            ref_teen.append_child(ref_child.clone());
        }

        let ns_child = &ref_child as RefNamespaced<'_>;

        //
        // Get
        //
        assert_eq!(ref_root.get_namespace(Some("xsd")), Some(XSD.to_string()));
        assert_eq!(ref_root.get_namespace(Some("xslt")), None);
        assert_eq!(ns_child.get_namespace(Some("xsd")), None);
        assert_eq!(ns_child.get_namespace(Some("xslt")), Some(XSLT.to_string()));

        //
        // Resolve
        //
        assert_eq!(
            ns_child.resolve_namespace(Some("xsd")),
            Some(XSD.to_string())
        );
        assert_eq!(ns_child.resolve_namespace(None), Some(HTML.to_string()));
        assert_eq!(
            ns_child.resolve_namespace(Some("xslt")),
            Some(XSLT.to_string())
        );

        //
        // Resolve by namespace
        //
        assert_eq!(ns_child.resolve_prefix(XSD), Some(Some("xsd".to_string())));
        assert_eq!(ns_child.resolve_prefix(HTML), Some(None));
        assert_eq!(
            ns_child.resolve_prefix(XSLT),
            Some(Some("xslt".to_string()))
        );
    }
}
