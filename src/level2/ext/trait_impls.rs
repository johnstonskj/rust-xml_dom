use crate::level2::dom_impl::Implementation;
use crate::level2::ext::decl::*;
use crate::level2::ext::options::ProcessingOptions;
use crate::level2::ext::traits::*;
use crate::level2::node_impl::*;
use crate::level2::trait_impls::create_document_with_options;
use crate::shared::error::*;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl DocumentDecl for RefNode {
    fn xml_declaration(&self) -> Option<XmlDecl> {
        let ref_self = self.borrow();
        if let Extension::Document {
            i_xml_declaration, ..
        } = &ref_self.i_extension
        {
            i_xml_declaration.clone()
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            None
        }
    }

    fn set_xml_declaration(&mut self, xml_decl: XmlDecl) -> Result<()> {
        let mut mut_self = self.borrow_mut();
        if let Extension::Document {
            i_xml_declaration, ..
        } = &mut mut_self.i_extension
        {
            *i_xml_declaration = Some(xml_decl);
            Ok(())
        } else {
            warn!("{}", MSG_INVALID_EXTENSION);
            Err(Error::InvalidState)
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl DOMImplementation for Implementation {
    fn create_document_with_options(
        &self,
        namespace_uri: Option<&str>,
        qualified_name: Option<&str>,
        doc_type: Option<Self::NodeRef>,
        options: ProcessingOptions,
    ) -> Result<Self::NodeRef> {
        create_document_with_options(namespace_uri, qualified_name, doc_type, options)
    }
}
