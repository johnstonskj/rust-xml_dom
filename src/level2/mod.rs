/*!
Implementation for DOM Core Level 2.

# Interface Mapping

| IDL Interface           | Rust Mapping                                                |
|-------------------------|-------------------------------------------------------------|
| `Attr`                  | [`Attribute`](level2/trait.Attribute.html)                         |
| _`CharacterData`_       | [`CharacterData`](level2/trait.CharacterData.html)                 |
| `CDATASection`          | [`CDataSection`](level2/trait.CDataSection.html)                   |
| `Comment`               | [`Comment`](level2/trait.Comment.html)                             |
| `Document`              | [`Document`](level2/trait.Document.html)                           |
| `DocumentFragment`      | [`DocumentFragment`](level2/trait.DocumentFragment.html)           |
| `DocumentType`          | [`DocumentType`](level2/trait.DocumentType.html)                   |
| `DOMImplementation`     | [`DOMImplementation`](level2/trait.DOMImplementation.html)         |
| `Element`               | [`Element`](level2/trait.Element.html)                             |
| `Entity`                | [`Entity`](level2/trait.Entity.html)                               |
| `EntityReference`       | [`EntityReference`](level2/trait.EntityReference.html)             |
| `NamedNodeMap`          | `HashMap<Name, RefNode>`                                    |
| `Node`                  | [`Node`](level2/trait.Node.html)                                   |
| `NodeList`              | `Vec<Rc<RefNode>>`                                          |
| `Notation`              | [`Notation`](level2/trait.Notation.html)                           |
| `ProcessingInstruction` | [`ProcessingInstruction`](level2/trait.ProcessingInstruction.html) |
| `Text`                  | [`Text`](level2/trait.Text.html)                                   |

# Conformance

The `has_feature` method on [`DOMImplementation`](trait.DOMImplementation.html) and `is_supported` on
[`Node`](trait.Node.html) will return true when the request is for support of the Core or XML
feature and supports both version 1.0 and version 2.0 of Core and version 1.0 of XML.

```rust
use xml_dom::level2::{DOMImplementation, get_implementation};

let implementation = get_implementation();
assert!(implementation.has_feature("Core", "1.0"));
assert!(implementation.has_feature("Core", "2.0"));
assert!(implementation.has_feature("XML", "1.0"));
assert!(implementation.has_feature("XML", "2.0"));
```

# Extensions

The following extensions are provided beyond the DOM Level 2 specification, all extensions are in the
[`level2::ext`](level2/ext/index.html) module.

1. The [`get_implementation`](dom_impl/fn.get_implementation.html) function returns an instance of
   `DOMImplementation` to allow bootstrapping the creation of documents. This satisfies the
   requirement from the specification: _"The DOM Level 2 API does not define a standard way to
   create DOMImplementation objects; DOM implementations must provide some proprietary way of
   bootstrapping these DOM interfaces, and then all other objects can be built from there."_.
1. The [`get_implementation_version`](dom_impl/fn.get_implementation_version.html) function in the
   [`dom_impl`](dom_impl/index.html) module returns a vendor-specific version identifier for the
   `DOMImplementation`.
1. The standard `DOMImplementation` trait also has an additional member
   [`create_document_with_options`](trait.DOMImplementation.html#tymethod.create_document_with_options),
   and associated [`ProcessingOptions`](options/struct.ProcessingOptions.html) structure, that can set
   optional behavior for a given `Document` instance.
1. The trait [`DocumentDecl`](trait.DocumentDecl.html) extends `Document` with the ability to set
   and retrieve the XML declaration from the document's prolog.
1. The trait [`Namespaced`](trait.Namespaced.html) extends `Element` with the ability to look-up
   namespace mappings (using the standard `xmlns` attribute).
1. The functions [`create_entity`](dom_impl/fn.create_entity.html),
   [`create_internal_entity`](dom_impl/fn.create_internal_entity.html), and
   [`create_notation`](dom_impl/fn.create_notation.html) in the
   [`dom_impl`](dom_impl/index.html) module provide the ability to create instances of these
   Level 2 extended interfaces. In general most clients using the DOM do not need to create these
   however parsers constructing the DOM may.

*/

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

pub mod convert;

pub mod dom_impl;
pub use dom_impl::get_implementation;

mod node_impl;
pub use node_impl::RefNode;

pub mod ext;

// ------------------------------------------------------------------------------------------------
// Re-Export
// ------------------------------------------------------------------------------------------------

pub use crate::shared::error::{Error, Result};

pub use crate::shared::name::*;

mod traits;
pub use traits::*;

// ------------------------------------------------------------------------------------------------
// Private Modules
// ------------------------------------------------------------------------------------------------

mod trait_impls;
