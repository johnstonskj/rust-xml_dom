# Crate xml_dom

A Rust crate providing a reasonably faithful implementation of the  W3C 
[Document Object Model Core, Level 2](https://www.w3.org/TR/DOM-Level-2-Core).

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.40-green.svg)
[![crates.io](https://img.shields.io/crates/v/xml_dom.svg)](https://crates.io/crates/xml_dom)
[![docs.rs](https://docs.rs/xml_dom/badge.svg)](https://docs.rs/xml_dom)
![Build](https://github.com/johnstonskj/rust-xml_dom/workflows/Rust/badge.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-xml_dom.svg)](https://github.com/johnstonskj/rust-xml_dom/stargazers)

This crate provides a trait-based implementation of the DOM with minimal changes to the style
and semantics defined in the Level 2 specification. The specific mapping from the IDL in the
specification is described in the documentation, however from a purely style point of
view the implementation has the following characteristics:

1. It maintains a reasonable separation between the node type traits and the tree implementation
   using opaque an `RefNode` reference type.
1. Where possible the names from IDL are used with minimal conversion.
1. All IDL attributes become trait functions (attribute "foo" becomes `foo()`, `set_foo()`, and `unset_foo()`).

This leads to a replication of the typical DOM programmer experience where casting between the
node traits is required. This is supported by the `xml_dom::convert` module.

## Example

```rust
use xml_dom::level2::*;
use xml_dom::level2::convert::*;

// Bootstrap; get an instance of `DOMImplementation`. The mechanism for this is
// intentionally undefined by the specification.
let implementation = get_implementation();

// Create a `DocumentType` instance.
let document_type = implementation
    .create_document_type(
        "html",
        Some("-//W3C//DTD XHTML 1.0 Transitional//EN"),
        Some("http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd"),
    )
    .unwrap();

// Create a new `Document` using the document type defined above. Note that this 
// also has the side-effect of creating the document's root element named "html".
let mut document_node = implementation
    .create_document(Some("http://www.w3.org/1999/xhtml"), Some("html"), Some(document_type))
    .unwrap();

// Cast the returned document `RefNode` into a `RefDocument` trait reference
let document = as_document_mut(&mut document_node).unwrap();

// Fetch the document's root element as a node, then cast to `RefElement`.
let mut root_node = document.document_element().unwrap();
let root = as_element_mut(&mut root_node).unwrap();

// Create an `Attribute` instance on the root element.
root.set_attribute("lang", "en");

// Create two child `Element`s of "html".
let _head = root.append_child(document.create_element("head").unwrap());
let _body = root.append_child(document.create_element("body").unwrap());

// Display as XML.
let xml = document_node.to_string();
println!("document 2: {}", xml);
```

This should result in the following XML; note that formatting was added for this document, the provided 
implementation of `Display` for `RefNode` does not format the output.
 
```xml
<!DOCTYPE 
  html 
  PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" 
  SYSTEM "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html lang="en">
  <head></head>
  <body></body>
</html>
```

## Features

Currently only one feature, `quick_parser`, is provided which provides a new module `parser` with the
single public function. This feature is enabled by default.

``` rust
pub fn read_xml(xml: &str) -> Result<RefNode>;
```

This will parse the document and return a new `RefNode` that corresponds to the `Document` trait.

## Changes

**Version 0.2.3**

* Mostly documentation updates/fixes.
* Made namespace support respect `ProcessingOptions`.

**Version 0.2.2**

* Mostly documentation updates/fixes.
* Fixed a defect in handling of `xml:id`.

**Version 0.2.1**

* Bug Fixes:
  * Fixed a publishing error in Travis
  * Separated `Attribute::owner_element` from `Node::parent_node`, they aren't the same.
  * Fixed handling of `owner_element` in `Attribute` tests
  * Fixed the implementation of `WrongDocument` error in `Node::insert_before` and used the same in 
    `Element::set_attribute_node`.
  * Fixed escaping of values to happen on get not set.
  * Implemented _attribute value normalization_ and _end-of-line handling_ from the XML 1.1 spec.
    * Required added a dependency on [`regex`](https://crates.io/crates/regex)
    * Added `EntityResolver` trait for callback into DOM.
    * Expansion of `Entity` node children is not yet complete.
* Code clean-up:
  * Added some macros in `trait_impls` for a number of common patterns.
  * Simplified some existing `match` expressions.
  
**Version 0.2.0** 

* Cleaned up documentation.
* Stable release.

**Version 0.1.4**

* **BREAKING** refactored to add a `level2` module, allowing other levels to be added at a later time. Also
  moved extensions into `level2::ext` module.
* **BREAKING** renamed methods to conform with DOM names:
  * `Node::name` to `Node::node_name`;
  * `CharacterData::substring` to `CharacterData::substring_data`;
  * `CharacterData::append` to `CharacterData::append_data`;
  * `CharacterData::insert` to `CharacterData::insert_data`;
  * `CharacterData::delete` to `CharacterData::delete_data`;
  * `CharacterData::replace` to `CharacterData::replace_data`.
* Implemented the following methods:
  * `Node::clone_node`;
  * `Node::normalize`;
  * `Namespaced::normalize_mappings`.
* Added the following DOM methods:
  * `Attribute::owner_element`;
  * `Node::local_name`;
  * `Node::namespace_uri`;
  * `Node::prefix`.
* CI builds now working with Travis, [rust-xml_dom](https://travis-ci.org/github/johnstonskj/rust-xml_dom).
* Added `quick_xml` based text parser.
* Make this the 0.2.0 candidate.

**Version 0.1.3**

* More unit tests overall, especially for append/insert/replace child
* Add support for xml declaration (`XmlDecl`, `XmlVersion`), not reusing processing instruction
* Support the last Level 2 _extended interfaces_ (`Entity`, `EntityReference`, and `Notation`).
  * Also, add `create_notation`, `create_entity`, and `create_internal_entity` to `dom_impl`.
* Implement an options (`ProcessingOptions` and `DOMImplementation::create_document_with_options`) capability to turn 
  on extended processing behaviors.
* Fixed some nested borrow issues.

**Version 0.1.2**

* Focus on feature completion:
  * implement all core trait features
  * implement extended trait features for currently supported traits
  * unescaping text
  * refactor `NodeImpl` for extended traits
* Unit tests, lot's of unit tests

**Version 0.1.1**

* Focus on API, separate the traits from implementation more cleanly.
* Better `Display` formatting
* Better `append_child` rule support
* Have support for namespace resolution
* Have support for text escaping on setting values
* More examples, fleshing out more of the common methods.
* Note, this is NOT YET ready for production usage.

**Version 0.1.0**

* Focus on modeling as traits, not all methods actually implemented.
* Note, this is NOT YET ready for production usage.

## TODO

1. Need to build more tests, especially doc-type.
1. Notation replacement, model correctness.

