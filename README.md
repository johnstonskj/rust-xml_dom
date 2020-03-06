# Crate xml_dom

A Rust crate providing a reasonably faithful implementation of the  W3C 
[Document Object Model Core, Level 2](https://www.w3.org/TR/DOM-Level-2-Core).

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.38-green.svg)
[![crates.io](https://img.shields.io/crates/v/upnp-rs.svg)](https://crates.io/crates/xml_dom)
[![docs.rs](https://docs.rs/xml_dom/badge.svg)](https://docs.rs/xml_dom)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-xml_dom.svg)](https://github.com/johnstonskj/rust-xml_dom/stargazers)

This crate provides a trait-based implementation of the DOM with minimal changes to the style
and semantics defined in the Level 2 specification. The specific mapping from the IDL in the
specification is described in the documentation, however from a purely style point of
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
let mut document_node =
    implementation.create_document("uri:urn:simons:thing:1", "root", None).unwrap();

let document = as_document(&document_node).unwrap();
let root = document.create_element("root").unwrap();

let mut root_node = document_node.append_child(root).unwrap();
let root = as_element_mut(&mut root_node).unwrap();
root.set_attribute("version", "1.0");
root.set_attribute("something", "else");

let xml = document_node.to_string();
println!("document 2: {}", xml);
```

## Changes

**Version 0.1.1**

* Focus on examples, fleshing out more of the common methods.

**Version 0.1.0**

* Focus on modeling as traits, not all methods actually implemented.
* Note, this is NOT YET ready for production usage.

## TODO

1. Currently does not support `DocumentFragment`, `Entity`, `EntityReference`, or `Notation`.
1. Not intending to be schema-aware, so `Document::get_element_by_id` always returns `None`.
1. A lot of required methods are still `unimplemented!()`.
1. Intend to add reader features to de-serialize using crate `quick_xml`.