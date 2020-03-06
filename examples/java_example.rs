use xml_dom::convert::*;
use xml_dom::*;

#[allow(unused_must_use)]
fn main() {
    let implementation = get_implementation();
    let mut document_node = implementation
        .create_document("uri:urn:simons:thing:1", "root", None)
        .unwrap();

    let mut root_node = {
        let document = as_document(&document_node).unwrap();
        let root = document.create_element_ns("zetcode.com", "users").unwrap();
        document_node.append_child(root).unwrap()
    };

    let document = as_document(&document_node).unwrap();
    root_node.append_child(create_user(document, "1", "Robert", "Brown", "programmer"));
    root_node.append_child(create_user(document, "2", "Pamela", "Kyle", "writer"));
    root_node.append_child(create_user(document, "3", "Peter", "Smith", "teacher"));

    let xml = document_node.to_string();
    println!("{}", xml);
}

fn create_user<'a>(
    doc: &'a dyn Document,
    id: &str,
    first_name: &str,
    last_name: &str,
    occupation: &str,
) -> RefNode {
    let mut user = doc
        .create_element("user")
        .expect("could not create a new element");
    user.set_attribute("id", id);
    user.append_child(create_user_element(doc, "firstname", first_name));
    user.append_child(create_user_element(doc, "lastname", last_name));
    user.append_child(create_user_element(doc, "occupation", occupation));
    user
}

fn create_user_element(doc: &dyn Document, name: &str, value: &str) -> RefNode {
    let mut node = doc
        .create_element(name)
        .expect("could not create a new element");
    node.append_child(doc.create_text_node(value));
    node
}
