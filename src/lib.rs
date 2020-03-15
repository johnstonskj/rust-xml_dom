/*!
A reasonably faithful implementation of the W3C [Document Object Model Core, Level
2](https://www.w3.org/TR/DOM-Level-2-Core).

This crate provides a trait-based implementation of the DOM with minimal changes to the style
and semantics defined in the Level 2 specification. The specific mapping from the IDL in the
specification is described [below](#idl-to-rust-mapping), however from a purely style point of
view the implementation has the following characteristics:

1. It maintains a reasonable separation between the node type traits and the tree implementation
   using opaque Node reference types.
1. Where possible the names from IDL are used with minimal conversion, however some redundant
   suffixes (`_data`, `_node`) have been reduced for brevity/clarity.
1. This leads to a replication of the typical programmer experience where casting between the
   node traits is required. This is supported by the `xml_dom::convert` module.

# Example

```rust
use xml_dom::*;
use xml_dom::convert::*;

let implementation = get_implementation();
let mut document_node = implementation
    .create_document("http://www.w3.org/1999/xhtml", "html", None)
    .unwrap();
println!("document 1: {:#?}", document_node);

let document = as_document_mut(&mut document_node).unwrap();
let mut root_node = document.document_element().unwrap();

let root = as_element_mut(&mut root_node).unwrap();
root.set_attribute("lang", "en");
let _head = root.append_child(document.create_element("head").unwrap());
let _body = root.append_child(document.create_element("body").unwrap());

let xml = document_node.to_string();
println!("document 2: {}", xml);
```

# Specification

* [Document Object Model (DOM) Level 1 Specification](https://www.w3.org/TR/REC-DOM-Level-1/),
  Version 1.0, W3C Recommendation 1 October, 1998. Specifically §1, _Document Object Model (Core)
  Level 1_.
* [Document Object Model (DOM) Level 2 Core Specification](https://www.w3.org/TR/DOM-Level-2-Core/),
  Version 1.0, W3C Recommendation 13 November, 2000. Specifically §1, _Document Object Model Core_.

## Conformance

TBD

The `has_feature` method [`DOMImplementation`](struct.DOMImplementation.html) and `is_supported` on
[`Node`](trait.Node.html) will return true when the request is for support of the Core or XML
feature and supports both version 1.0 and version 2.0 of Core and version 1.0 of XML. Currently some
of the XML extended interfaces are unsupported.

```rust
use xml_dom::{DOMImplementation, get_implementation};

let implementation = get_implementation();
assert!(implementation.has_feature("Core", "1.0"));
assert!(implementation.has_feature("Core", "2.0"));
assert!(implementation.has_feature("XML", "1.0"));
assert!(!implementation.has_feature("XML", "2.0"));
```

# IDL to Rust Mapping

From the core documentation:

> The `Node` interface is the primary datatype for the entire Document Object Model. It represents
> a single node in the document tree. While all objects implementing the `Node` interface expose
> methods for dealing with children, not all objects implementing the `Node` interface may have
> children. For example, `Text` nodes may not have children, and adding children to such nodes
> results in a DOMException being raised.

> The attributes `nodeName`, `nodeValue` and `attributes` are included as a mechanism to get at
> node information without casting down to the specific derived interface. In cases where there is
> no obvious mapping of these attributes for a specific `nodeType` (e.g., `nodeValue` for an
> `Element` or `attributes` for a `Comment`), this returns `null`. Note that the specialized
> interfaces may contain additional and more convenient mechanisms to get and set the relevant
> information.

Wherever possible the documentation included in sections headed **Specification**  is taken from
the specification documents listed above.

## Interface Mapping

The actual concrete types used in the DOM tree are [`RefNode`](type.RefNode.html)
and [`WeakRefNode`](type.WeakRefNode.html) which in turn are references to the opaque
[`NodeImpl`](struct.NodeImpl.html) struct. Only `RefNode` implements all of the DOM interfaces
and in general the programmer should never need to interact with `WeakRefNode`.

| IDL Interface           | Rust Mapping                                                |
|-------------------------|-------------------------------------------------------------|
| `Attr`                  | [`Attribute`](trait.Attribute.html)                         |
| _`CharacterData`_       | [`CharacterData`](trait.CharacterData.html)                 |
| `CDATASection`          | [`CDataSection`](trait.CDataSection.html)                   |
| `Comment`               | [`Comment`](trait.Comment.html)                             |
| `Document`              | [`Document`](trait.Document.html)                           |
| `DocumentFragment`      | [`DocumentFragment`](trait.DocumentFragment.html)           |
| `DocumentType`          | [`DocumentType`](trait.DocumentType.html)                   |
| `DOMImplementation`     | [`DOMImplementation`](trait.DOMImplementation.html)         |
| `Element`               | [`Element`](trait.Element.html)                             |
| `Entity`                | [`Entity`](trait.Entity.html)                               |
| `EntityReference`       | [`EntityReference`](trait.EntityReference.html)             |
| `NamedNodeMap`          | `HashMap<Name, RefNode>`                                    |
| `Node`                  | [`Node`](trait.Node.html)                                   |
| `NodeList`              | `Vec<Rc<RefNode>>`                                          |
| `Notation`              | [`Notation`](trait.Notation.html)                           |
| `ProcessingInstruction` | [`ProcessingInstruction`](trait.ProcessingInstruction.html) |
| `Text`                  | [`Text`](trait.Text.html)                                   |

* The exception type `DOMException` and associated constants are represented by the enumeration
  `Error`.
* IDL Interface attributes are represented by functions;
  * readonly attributes simply have an `attribute_name` getter,
  * writeable attributes also have a `set_attribute_name` setter,
  * some attributes allow null in which case they have an `unset_attribute_name` setter.
* IDL function names are altered from `lowerCamelCase` to `snake_case`.
* IDL functions that are marked `raises(DOMException)` return [`Result`](type.Result.html) with
  [`Error`](enum.Error.html) as the error type.
* IDL attributes of type `T` that are described as "_may be `null`_", or IDL functions that "_may
  return `T` or `null`_" instead return `Option<T>`.

## Primitive Type Mapping

| IDL Type         | Rust Type      | Usage                                |
|------------------|----------------|--------------------------------------|
| `boolean`        | `bool`         | all                                  |
| `DOMString`      | `String`       | all                                  |
| `unsigned short` | `Error`, `u16` | as representation of exception code  |
| `unsigned long`  | `usize`        | list/string indexes and lengths      |

## Extensions

The following extensions are provided beyond the DOM Level 2 specification.

1. The [`get_implementation`](fn.get_implementation.html) function returns an instance of
   `DOMImplementation` to allow bootstrapping the creation of documents.
1. The [`get_implementation_version`](fn.get_implementation_version.html) function returns a
   vendor-specific version identifier for the `DOMImplementation`.
1. The trait [`Namespaced`](trait.Namespaced.html) extends `Element` with the ability to look
   up namespace mappings (using the standard `xmlns` attribute).

*/

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

#[macro_use]
extern crate log;

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

pub mod convert;

mod dom_impl;
pub use dom_impl::{get_implementation, get_implementation_version};

mod error;
pub use error::*;

mod name;
pub use name::*;

mod namespaced;
pub use namespaced::Namespaced;

mod node_impl;
pub use node_impl::RefNode;

mod options;
pub use options::ProcessingOptions;

mod traits;
pub use traits::*;

mod trait_impls;
pub use trait_impls::*;

// ------------------------------------------------------------------------------------------------
// Private Modules
// ------------------------------------------------------------------------------------------------

mod rc_cell;

mod syntax;

mod text;
