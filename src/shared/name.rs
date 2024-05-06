use crate::shared::error::*;
use crate::shared::syntax::*;
use crate::shared::text::is_xml_name;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::{from_utf8, FromStr};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Corresponds to attributes `localName`, `namespaceURI`, and `prefix` on the DOM `Node` interface.
///
/// # Specification
///
/// `localName` **of type `DOMString`, readonly, introduced in DOM Level 2**
///
/// > Returns the local part of the qualified name of this node.
/// > For nodes of any type other than ELEMENT_NODE and ATTRIBUTE_NODE and nodes created with a DOM
/// > Level 1 method, such as createElement from the Document interface, this is always null.
///
/// `namespaceURI` **of type `DOMString`, readonly, introduced in DOM Level 2**
/// > The namespace URI of this node, or null if it is unspecified.
/// > This is not a computed value that is the result of a namespace lookup based on an examination
/// > of the namespace declarations in scope. It is merely the namespace URI given at creation time.
/// > For nodes of any type other than ELEMENT_NODE and ATTRIBUTE_NODE and nodes created with a DOM
/// > Level 1 method, such as createElement from the Document interface, this is always null.
///
/// > **Note:** Per the Namespaces in XML Specification an attribute does not inherit its namespace
/// > from the element it is attached to. If an attribute is not explicitly given a namespace, it
/// > simply has no namespace.
///
/// `prefix` **of type `DOMString`, introduced in DOM Level 2**
///
/// > The namespace prefix of this node, or null if it is unspecified.
/// > Note that setting this attribute, when permitted, changes the nodeName attribute, which holds
/// > the qualified name, as well as the tagName and name attributes of the Element and Attr
/// > interfaces, when applicable.
/// > Note also that changing the prefix of an attribute that is known to have a default value,
/// > does not make a new attribute with the default value and the original prefix appear, since
/// > the namespaceURI and localName do not change.
/// > For nodes of any type other than ELEMENT_NODE and ATTRIBUTE_NODE and nodes created with a DOM
/// > Level 1 method, such as createElement from the Document interface, this is always null.
/// >
/// > **Exceptions on setting**
/// >
/// > * `INVALID_CHARACTER_ERR`: Raised if the specified prefix contains an illegal character.
/// > * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node is readonly.
/// > * `NAMESPACE_ERR`: Raised if the specified prefix is malformed, if the namespaceURI of this
/// >   node is null, if the specified prefix is "xml" and the namespaceURI of this node is
/// >   different from "http://www.w3.org/XML/1998/namespace", if this node is an attribute and the
/// >   specified prefix is "xmlns" and the namespaceURI of this node is different from
/// >   "http://www.w3.org/2000/xmlns/", or if this node is an attribute and the qualifiedName of
/// >   this node is "xmlns".
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Name {
    pub(crate) namespace_uri: Option<String>,
    pub(crate) prefix: Option<String>,
    pub(crate) local_name: String,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.prefix {
            Some(prefix) => write!(f, "{}{}{}", prefix, XML_NS_SEPARATOR, self.local_name),
            None => write!(f, "{}", self.local_name),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl FromStr for Name {
    type Err = Error;

    fn from_str(value: &str) -> StdResult<Self, Self::Err> {
        if value.is_empty() {
            Err(Error::Syntax)
        } else {
            let parts = value
                .split(XML_NS_SEPARATOR)
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            match parts.len() {
                1 => Name::new(Name::check_part(parts.get(0).unwrap())?, None, None),
                2 => Name::new(
                    Name::check_part(parts.get(1).unwrap())?,
                    Some(Name::check_part(parts.get(0).unwrap())?),
                    None,
                ),
                _ => Err(Error::Syntax),
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl TryFrom<&[u8]> for Name {
    type Error = Error;

    fn try_from(value: &[u8]) -> StdResult<Self, Self::Error> {
        match from_utf8(value) {
            Ok(str) => Self::from_str(str),
            Err(e) => {
                error!("Could not convert from UTF-8, error {:?}", e);
                Err(Error::InvalidCharacter)
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Name {
    ///
    /// Construct a new `Name` from the specified namespace URI and qualified name.
    ///
    /// Note, errors include a malformed URI, or malformed prefix or local name.
    ///
    pub fn new_ns(namespace_uri: impl AsRef<str>, qualified_name: impl AsRef<str>) -> Result<Self> {
        let mut parsed = Name::from_str(qualified_name.as_ref())?;
        parsed.namespace_uri = Some(Self::check_namespace_uri(
            namespace_uri.as_ref(),
            &parsed.prefix,
            &parsed.local_name,
        )?);
        Ok(parsed)
    }

    ///
    /// Construct a new `Name` from any combination of local name, prefix, and namespace URI.
    ///
    /// Note, errors include a malformed URI, or malformed prefix or local name.
    ///
    fn new(
        local_name: String,
        prefix: Option<String>,
        namespace_uri: Option<String>,
    ) -> Result<Self> {
        if local_name.is_empty() {
            warn!("local_name may not be empty");
            return Err(Error::Syntax);
        }
        if let Some(prefix) = &prefix {
            if prefix.is_empty() {
                warn!("prefix may not be empty");
                return Err(Error::Syntax);
            }
        }
        if let Some(namespace_uri) = &namespace_uri {
            if namespace_uri.is_empty() {
                warn!("namespace_uri may not be empty");
                return Err(Error::Syntax);
            }
        }
        Ok(Self {
            namespace_uri,
            prefix,
            local_name,
        })
    }

    fn check_part(part: impl AsRef<str>) -> Result<String> {
        let part = part.as_ref();
        if part.is_empty() {
            Err(Error::Syntax)
        } else if is_xml_name(part) {
            Ok(part.to_string())
        } else {
            Err(Error::InvalidCharacter)
        }
    }

    fn check_namespace_uri(
        namespace_uri: impl AsRef<str>,
        prefix: &Option<String>,
        local: impl AsRef<str>,
    ) -> Result<String> {
        let namespace_uri = namespace_uri.as_ref();
        let local = local.as_ref();

        if namespace_uri.is_empty() {
            return Err(Error::Syntax)
        }

        if let Some(prefix) = prefix {
            if (prefix == XML_NS_ATTRIBUTE && namespace_uri != XML_NS_URI)
                || (prefix == XMLNS_NS_ATTRIBUTE && namespace_uri != XMLNS_NS_URI) {
                return Err(Error::Namespace);
            }
        }

        if (local == XML_NS_ATTRIBUTE && namespace_uri != XML_NS_URI)
            || (local == XMLNS_NS_ATTRIBUTE && namespace_uri != XMLNS_NS_URI) {
            return Err(Error::Namespace)
        }

        Ok(namespace_uri.to_string())
    }

    ///
    /// Return the reserved name for `CDATA` section nodes
    ///
    pub fn for_cdata() -> Self {
        Self {
            namespace_uri: None,
            prefix: None,
            local_name: XML_NAME_CDATA.to_string(),
        }
    }

    ///
    /// Return the reserved name for `Comment` nodes
    ///
    pub fn for_comment() -> Self {
        Self {
            namespace_uri: None,
            prefix: None,
            local_name: XML_NAME_COMMENT.to_string(),
        }
    }

    ///
    /// Return the reserved name for `Document` nodes
    ///
    pub fn for_document() -> Self {
        Self {
            namespace_uri: None,
            prefix: None,
            local_name: XML_NAME_DOCUMENT.to_string(),
        }
    }

    ///
    /// Return the reserved name for `Document` nodes
    ///
    pub fn for_document_fragment() -> Self {
        Self {
            namespace_uri: None,
            prefix: None,
            local_name: XML_NAME_DOCUMENT_FRAGMENT.to_string(),
        }
    }

    ///
    /// Return the reserved name for `Text` nodes
    ///
    pub fn for_text() -> Self {
        Self {
            namespace_uri: None,
            prefix: None,
            local_name: XML_NAME_TEXT.to_string(),
        }
    }

    ///
    /// Return the reserved name for `DocumentType` `public_id` attribute
    ///
    pub fn for_public_id() -> Self {
        Self {
            namespace_uri: None,
            prefix: None,
            local_name: XML_DOCTYPE_PUBLIC.to_string(),
        }
    }

    ///
    /// Return the reserved name for `DocumentType` `system_id` attribute
    ///
    pub fn for_system_id() -> Self {
        Self {
            namespace_uri: None,
            prefix: None,
            local_name: XML_DOCTYPE_SYSTEM.to_string(),
        }
    }

    ///
    /// Return the reserved name for `Entity` `notation_name` attribute
    ///
    #[allow(dead_code)]
    pub(crate) fn for_null() -> Self {
        Self {
            namespace_uri: None,
            prefix: None,
            local_name: "null".to_string(),
        }
    }

    ///
    /// Does this appear to be an `xmlns` attribute.
    ///
    pub fn is_namespace_attribute(&self) -> bool {
        let xmlns_ns = Some(XMLNS_NS_URI.to_string());
        let xmlns_attribute = XMLNS_NS_ATTRIBUTE.to_string();
        self.namespace_uri == xmlns_ns
            && ((self.local_name == xmlns_attribute && self.prefix == None)
                || self.prefix == Some(xmlns_attribute))
    }

    ///
    /// Construct a name for an `xmlns` attribute.
    ///
    pub fn for_namespace(prefix: Option<&str>) -> Self {
        let xmlns_ns = Some(XMLNS_NS_URI.to_string());
        let xmlns_attribute = XMLNS_NS_ATTRIBUTE.to_string();
        match prefix {
            None => Self::new(xmlns_attribute, None, xmlns_ns).unwrap(),
            Some(prefix) => Self::new(prefix.to_string(), Some(xmlns_attribute), xmlns_ns).unwrap(),
        }
    }

    ///
    /// Does this appear to be an `id` attribute.
    ///
    pub fn is_id_attribute(&self, lax: bool) -> bool {
        let id_attribute = XML_NS_ATTR_ID.to_string();
        if lax {
            //
            // any attribute with the local_name 'id'
            //
            self.local_name == id_attribute
        } else {
            let xml_ns = XML_NS_URI.to_string();
            let xml_prefix = XML_NS_ATTRIBUTE.to_string();
            //
            // has to be 'xml:id', either by the prefix 'xml' or using the correct namespace
            self.local_name == id_attribute
                && (self.namespace_uri == Some(xml_ns) || self.prefix == Some(xml_prefix))
        }
    }

    ///
    /// Construct a name for an `xml:id` attribute.
    /// ///
    pub fn for_xml_id() -> Self {
        Self {
            namespace_uri: Some(XML_NS_URI.to_string()),
            prefix: Some(XML_NS_ATTRIBUTE.to_string()),
            local_name: XML_NS_ATTR_ID.to_string(),
        }
    }

    ///
    /// Return this name's namespace URI.
    ///
    pub fn namespace_uri(&self) -> &Option<String> {
        &self.namespace_uri
    }

    ///
    /// Return this name's local name.
    ///
    pub fn local_name(&self) -> &String {
        &self.local_name
    }

    ///
    /// Return this name's prefix.
    ///
    pub fn prefix(&self) -> &Option<String> {
        &self.prefix
    }

    ///
    /// Set this name's prefix.
    ///
    pub fn set_prefix(&mut self, new_prefix: Option<&str>) -> Result<()> {
        self.prefix = new_prefix.map(String::from);
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::shared::error::Error;
    use crate::shared::name::Name;
    use crate::shared::syntax::{XMLNS_NS_URI, XML_NS_URI};
    use std::str::FromStr;

    #[test]
    fn test_parse_invalid_chars() {
        for c in " \t\r\n\u{0}!?".chars() {
            let name = Name::from_str(&format!("he{}lo", c));
            assert!(name.is_err());
        }
    }

    #[test]
    fn test_parse_local() {
        let name = Name::from_str("hello").unwrap();
        assert_eq!(name.local_name, "hello".to_string());
        assert!(name.prefix().is_none());
        assert!(name.namespace_uri().is_none());
    }

    #[test]
    fn test_parse_qualified() {
        let name = Name::from_str("x:hello").unwrap();
        assert_eq!(name.local_name, "hello".to_string());
        assert_eq!(name.prefix(), &Some("x".to_string()));
        assert!(name.namespace_uri().is_none());
    }

    #[test]
    fn test_parse_namespaced() {
        let name = Name::new_ns("http://example.org/schema/x", "x:hello").unwrap();
        assert_eq!(name.local_name, "hello".to_string());
        assert_eq!(name.prefix(), &Some("x".to_string()));
        assert_eq!(
            name.namespace_uri(),
            &Some("http://example.org/schema/x".to_string())
        );
    }

    #[test]
    fn test_error_on_empty() {
        let name = Name::from_str("");
        assert_eq!(name.err().unwrap(), Error::Syntax);

        let name = Name::from_str(":name");
        assert_eq!(name.err().unwrap(), Error::Syntax);

        let name = Name::from_str("prefix:");
        assert_eq!(name.err().unwrap(), Error::Syntax);

        let name = Name::new_ns("", "prefix:name");
        assert_eq!(name.err().unwrap(), Error::Syntax);
    }

    #[test]
    fn test_xml_ns_names() {
        const RDF_NS: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

        let name = Name::new_ns(XML_NS_URI, "xml:id");
        assert!(name.is_ok());
        let name = name.unwrap();
        assert!(name.is_id_attribute(true));
        assert!(name.is_id_attribute(false));

        let name = Name::new_ns(RDF_NS, "xml:id");
        assert_eq!(name.err().unwrap(), Error::Namespace);

        let name = Name::from_str("another:id");
        assert!(name.is_ok());
        let name = name.unwrap();
        assert!(name.is_id_attribute(true));
        assert!(!name.is_id_attribute(false));

        let name = Name::from_str("x:hello");
        assert!(name.is_ok());
        let name = name.unwrap();
        assert!(!name.is_id_attribute(true));
        assert!(!name.is_id_attribute(false));
    }

    #[test]
    fn test_xmlns_ns_names() {
        const RDF_NS: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

        let name = Name::new_ns(XMLNS_NS_URI, "xmlns");
        assert!(name.is_ok());
        assert!(name.unwrap().is_namespace_attribute());

        let name = Name::new_ns(XMLNS_NS_URI, "xmlns:p");
        assert!(name.is_ok());
        assert!(name.unwrap().is_namespace_attribute());

        let name = Name::new_ns(RDF_NS, "xmlns");
        assert_eq!(name.err().unwrap(), Error::Namespace);

        let name = Name::new_ns(RDF_NS, "xmlns:rdf");
        assert_eq!(name.err().unwrap(), Error::Namespace);

        let name = Name::from_str("x:hello").unwrap();
        assert!(!name.is_namespace_attribute());
    }
}
