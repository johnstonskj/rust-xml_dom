/*!
One-line description.

More detailed description, with

# Example

*/

use crate::level2::convert::{as_document, as_document_type};
use crate::level2::{Name, Node, NodeType, RefNode};
use crate::shared::text::EntityResolver;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl EntityResolver for RefNode {
    fn resolve(&self, entity: &str) -> Option<String> {
        let doc_type = match self.node_type() {
            NodeType::DocumentType => Some(self.clone()),
            NodeType::Document => {
                let document = as_document(self).unwrap();
                document.doc_type()
            }
            _ => match self.owner_document() {
                None => None,
                Some(document_node) => {
                    let document = as_document(&document_node).unwrap();
                    document.doc_type()
                }
            },
        };
        match doc_type {
            None => None,
            Some(doc_type) => {
                let doc_type = as_document_type(&doc_type).unwrap();
                let name = Name::from_str(entity).unwrap();
                match doc_type.entities().get(&name) {
                    None => None,
                    Some(entity) => entity.node_value(),
                }
            }
        }
    }
}
