/*!
A reasonably faithful implementation of the W3C [Document Object Model Core, Level
2](https://www.w3.org/TR/DOM-Level-2-Core).

This crate provides a trait-based implementation of the DOM with minimal changes to the style
and semantics defined in the Level 2 specification. The specific mapping from the IDL in the
specification is described [below](#idl-to-rust-mapping), however from a purely style point of
view the implementation has the following characteristics:

1. It maintains a reasonable separation between the node type traits and the tree implementation
   using opaque `Node` reference types.
1. Where possible the names from IDL are used with minimal conversion, however some redundant
   suffixes (`_data`, `_node`) have been reduced for brevity/clarity.
1. This leads to a replication of the typical programmer experience where casting between the
   node traits is required. This is supported by the [`xml_dom::level2::convert`](level2/convert/index.html)
   module.

# Example

```rust
use xml_dom::level2::*;
use xml_dom::level2::convert::*;

let implementation = get_implementation();
let mut document_node = implementation
    .create_document(Some("http://www.w3.org/1999/xhtml"), Some("html"), None)
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
* [Extensible Markup Language (XML) 1.0 (Fifth Edition)](https://www.w3.org/TR/REC-xml/), W3C
  Recommendation 26 November 2008. Especially §3.3.3 _Attribute-Value Normalization_.
* [Namespaces in XML 1.1 (Second Edition)](https://www.w3.org/TR/xml-names11/), W3C Recommendation
  16 August 2006.
* [xml:id Version 1.0](https://www.w3.org/TR/xml-id), W3C Recommendation 9 September 2005.
  Especially §7.1 _Conformance to xml:id_.
* [XML Base (Second Edition)](https://www.w3.org/TR/xmlbase/), W3C Recommendation 28 January 2009.
* [The "xml" Namespace](https://www.w3.org/XML/1998/namespace), W3C 26 October 2009.

## Conformance

TBD

The `has_feature` method [`DOMImplementation`](struct.DOMImplementation.html) and `is_supported` on
[`Node`](trait.Node.html) will return true when the request is for support of the Core or XML
feature and supports both version 1.0 and version 2.0 of Core and version 1.0 of XML.

```rust
use xml_dom::level2::{DOMImplementation, get_implementation};

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

The actual concrete types used in the DOM tree are [`RefNode`](type.RefNode.html) and `WeakRefNode`
which in turn are references an opaque `NodeImpl struct. Only `RefNode` implements all of the DOM
interfaces and client programmers should never need to interact with `WeakRefNode`.

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

* The exception type `DOMException` and associated constants are represented by the enumeration
  `Error`.
* IDL Interface attributes are represented by functions;
  * readonly attributes simply have an `attribute_name` getter,
  * writeable attributes also have a `set_attribute_name` setter,
  * some attributes allow null in which case they have an `unset_attribute_name` setter.
* IDL function names are altered from `lowerCamelCase` to `snake_case`.
* IDL functions that are marked `raises(DOMException)` return [`Result`](level2/type.Result.html) with
  [`Error`](level2/enum.Error.html) as the error type.
* IDL attributes of type `T` that are described as "_may be `null`_", or IDL functions that "_may
  return `T` or `null`_" instead return `Option<T>`.

## Primitive Type Mapping

| IDL Type         | Rust Type      | Usage                                |
|------------------|----------------|--------------------------------------|
| `boolean`        | `bool`         | all                                  |
| `DOMString`      | `String`       | all                                  |
| `unsigned short` | `Error`, `u16` | as representation of exception code  |
| `unsigned long`  | `usize`        | list/string indexes and lengths      |

# Extensions

The following extensions are provided beyond the DOM Level 2 specification.

1. The [`get_implementation`](level2/dom_impl/fn.get_implementation.html) function returns an instance of
   `DOMImplementation` to allow bootstrapping the creation of documents. This satisfies the
   requirement from the specification: _"The DOM Level 2 API does not define a standard way to
   create DOMImplementation objects; DOM implementations must provide some proprietary way of
   bootstrapping these DOM interfaces, and then all other objects can be built from there."_.
1. The [`get_implementation_version`](level2/dom_impl/fn.get_implementation_version.html) function in the
   [`dom_impl`](level2/dom_impl/index.html) module returns a vendor-specific version identifier for the
   `DOMImplementation`.
1. The standard `DOMImplementation` trait also has an additional member
   [`create_document_with_options`](level2/trait.DOMImplementation.html#tymethod.create_document_with_options),
   and associated [`ProcessingOptions`](level2/options/struct.ProcessingOptions.html) structure, that can set
   optional behavior for a given `Document` instance.
1. The trait [`DocumentDecl`](level2/trait.DocumentDecl.html) extends `Document` with the ability to set
   and retrieve the XML declaration from the document's prolog.
1. The trait [`Namespaced`](level2/trait.Namespaced.html) extends `Element` with the ability to look-up
   namespace mappings (using the standard `xmlns` attribute).
1. The functiions [`create_entity`](level2/dom_impl/fn.create_entity.html),
   [`create_internal_entity`](level2/dom_impl/fn.create_internal_entity.html), and
   [`create_notation`](level2/dom_impl/fn.create_notation.html) in the
   [`dom_impl`](level2/dom_impl/index.html) module provide the ability to create instances of these
   Level 2 extended interfaces. In general most clients using the DOM do not need to create these
   however parsers constructing the DOM may.

# Logging

As of this time the only dependency `xml_dom` has is the [`log`](https://crates.io/crates/log)
crate and only `warn!` and `error!` macros are used to provide more information than the set of
error conditions defined by the DOM.

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

//#[cfg(feature = "quick_parser")]
//pub mod parser;

pub mod level2;

// ------------------------------------------------------------------------------------------------
// Private Modules
// ------------------------------------------------------------------------------------------------

mod shared;
