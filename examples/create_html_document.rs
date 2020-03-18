use xml_dom::convert::*;
use xml_dom::*;

#[allow(unused_must_use)]
fn main() {
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
        .create_document("http://www.w3.org/1999/xhtml", "html", Some(document_type))
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
    println!("{}", xml);

    assert!(xml.starts_with("<!DOCTYPE html "));
    assert!(xml.contains("<html lang=\"en\">"));
    assert!(xml.contains("<head></head>"));
    assert!(xml.contains("<body></body>"));
    assert!(xml.ends_with("</html>"));
}
