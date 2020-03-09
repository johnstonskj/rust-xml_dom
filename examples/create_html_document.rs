use xml_dom::convert::*;
use xml_dom::*;

#[allow(unused_must_use)]
fn main() {
    let implementation = get_implementation();
    let document_type = implementation
        .create_document_type(
            "html",
            "-//W3C//DTD XHTML 1.0 Transitional//EN",
            "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd",
        )
        .unwrap();
    let mut document_node = implementation
        .create_document("http://www.w3.org/1999/xhtml", "html", Some(document_type))
        .unwrap();
    println!("document 1: {:#?}", document_node);

    let document = as_document_mut(&mut document_node).unwrap();
    let root = document.document_element().unwrap();

    let mut root_node = document.append_child(root).unwrap();
    let root = as_element_mut(&mut root_node).unwrap();
    root.set_attribute("lang", "en");

    let _head = root.append_child(document.create_element("head").unwrap());

    let _body = root.append_child(document.create_element("body").unwrap());

    let xml = document_node.to_string();
    println!("document 2: {}", xml);
    assert!(xml.starts_with("<!DOCTYPE html "));
    assert!(xml.contains("<html \"lang\"=\"en\">"));
    assert!(xml.contains("<head></head>"));
    assert!(xml.contains("<body></body>"));
    assert!(xml.ends_with("</html>"));
}
