use std::collections::HashMap;

#[derive(Clone, Debug)]
pub(crate) struct Namespaces {
    scopes: Vec<HashMap<Option<String>, String>>,
}

impl Default for Namespaces {
    fn default() -> Self {
        Self { scopes: vec![] }
    }
}

impl Namespaces {
    pub(crate) fn push_scope(&mut self) {
        self.scopes.push(HashMap::default());
    }
    pub(crate) fn pop_scope(&mut self) {
        self.scopes.push(HashMap::default());
    }
    pub(crate) fn current_scope(&mut self) -> &HashMap<Option<String>, String> {
        self.scopes.last().unwrap()
    }
    pub(crate) fn insert(&mut self, prefix: Option<&str>, namespace_uri: &str) {
        match self.scopes.pop() {
            None => (),
            Some(mut current) => {
                let _old_value =
                    current.insert(prefix.map(String::from), namespace_uri.to_string());
                self.scopes.push(current);
            }
        }
    }
    pub(crate) fn get(&mut self, prefix: Option<&str>) -> Option<&String> {
        let current = self.scopes.last().unwrap();
        current.get(&prefix.map(String::from))
    }
    pub(crate) fn contains(&mut self, prefix: Option<&str>) -> bool {
        let current = self.scopes.last().unwrap();
        current.contains_key(&prefix.map(String::from))
    }
    pub(crate) fn resolve(&mut self, _prefix: Option<&str>) -> Option<&String> {
        let _current = self.scopes.last().unwrap();
        None
    }
    pub(crate) fn get_prefix_for(&mut self, namespace_uri: &str) -> Option<&String> {
        let current = self.scopes.last().unwrap();
        let ns = namespace_uri.to_string();
        current.values().find(|v| **v == ns)
    }
    pub(crate) fn contains_namespace(&mut self, namespace_uri: &str) -> bool {
        self.get_prefix_for(namespace_uri).is_some()
    }
    pub(crate) fn resolve_prefix_for(&mut self, _namespace_uri: &str) -> Option<&String> {
        let _current = self.scopes.last().unwrap();
        None
    }
}
