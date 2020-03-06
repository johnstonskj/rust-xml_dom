use xml_dom::convert::*;
use xml_dom::*;

#[allow(unused_must_use)]
fn main() {
    let implementation = get_implementation();
    let mut document_node = implementation
        .create_document("uri:urn:simons:thing:1", "root", None)
        .unwrap();
    println!("document 1: {:#?}", document_node);

    let document = as_document(&document_node).unwrap();
    let root = document.create_element("root").unwrap();
    println!("element 1: {:#?}", root);

    let mut root_node = document_node.append_child(root).unwrap();
    let root = as_element_mut(&mut root_node).unwrap();
    root.set_attribute("version", "1.0");
    root.set_attribute("something", "else");

    let xml = document_node.to_string();
    println!("document 2: {}", xml);
    assert!(xml.starts_with("<root"));
    assert!(xml.ends_with("</root>"));
    assert!(xml.contains("\"version\"=\"1.0\""));
    assert!(xml.contains("\"something\"=\"else\""));
}
