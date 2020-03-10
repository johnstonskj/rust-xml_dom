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
let mut document_node = implementation
    .create_document("http://www.w3.org/1999/xhtml", "html", None)
    .unwrap();

let document = as_document_mut(&mut document_node).unwrap();
let mut root_node = document.document_element().unwrap();

let root = as_element_mut(&mut root_node).unwrap();
root.set_attribute("lang", "en");
let _head = root.append_child(document.create_element("head").unwrap());
let _body = root.append_child(document.create_element("body").unwrap());

let xml = document_node.to_string();
println!("HTML: {}", xml);
```

## Changes

**Version 0.1.2** (_in progress_)

* Focus on feature completion:
  * implement all core trait features
  * implement extended trait features for currently supported traits
  * unescaping text
  
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

1. Currently does not support `DocumentFragment`, `Entity`, `EntityReference`, or `Notation`.
1. Not intending to be schema-aware, so `Document::get_element_by_id` always returns `None`.
1. A lot of required methods are still `unimplemented!()`.
1. Intend to add `reader` feature to de-serialize using crate `quick_xml`.
