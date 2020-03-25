#[doc(hidden)]
#[macro_export]
macro_rules! make_ref_type {
    ($ref_t:ident, $trait_t:ident) => {
        /// **Ref** type for dynamic trait cast
        pub type $ref_t<'a> = &'a dyn $trait_t<NodeRef = RefNode>;
    };
    ($ref_t:ident, $mut_t:ident, $trait_t:ident) => {
        /// **Ref** type for dynamic trait cast
        pub type $ref_t<'a> = &'a dyn $trait_t<NodeRef = RefNode>;
        /// Mutable **Ref** type for mutable dynamic trait cast
        pub type $mut_t<'a> = &'a mut dyn $trait_t<NodeRef = RefNode>;
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! make_is_as_functions {
    ($is_f:ident, $is_t:expr, $as_f:ident, $as_t:ident) => {
        ///
        /// Determines if the specified node is of the correct node type.
        ///
        #[inline]
        pub fn $is_f(ref_node: &RefNode) -> bool {
            ref_node.borrow().i_node_type == $is_t
        }

        ///
        /// Safely _cast_ the specified `RefNode` into a **Ref** type.
        ///
        #[inline]
        pub fn $as_f(ref_node: &RefNode) -> Result<$as_t<'_>> {
            if ref_node.borrow().i_node_type == $is_t {
                Ok(ref_node as $as_t<'_>)
            } else {
                warn!("{}", MSG_INVALID_NODE_TYPE);
                Err(Error::InvalidState)
            }
        }
    };
    ($is_f:ident, $is_t:expr, $as_f:ident, $as_t:ident, $as_mut_f:ident, $as_mut_t:ident) => {
        ///
        /// Determines if the specified node is of the correct node type.
        ///
        #[inline]
        pub fn $is_f(ref_node: &RefNode) -> bool {
            ref_node.borrow().i_node_type == $is_t
        }

        ///
        /// Safely _cast_ the specified `RefNode` into a **Ref** type.
        ///
        #[inline]
        pub fn $as_f(ref_node: &RefNode) -> Result<$as_t<'_>> {
            if ref_node.borrow().i_node_type == $is_t {
                Ok(ref_node as $as_t<'_>)
            } else {
                warn!("{}", MSG_INVALID_NODE_TYPE);
                Err(Error::InvalidState)
            }
        }

        ///
        /// Safely _cast_ the specified `RefNode` into a mutable **Ref** type.
        ///
        #[inline]
        pub fn $as_mut_f(ref_node: &mut RefNode) -> Result<$as_mut_t<'_>> {
            if ref_node.borrow().i_node_type == $is_t {
                Ok(ref_node as $as_mut_t<'_>)
            } else {
                warn!("{}", MSG_INVALID_NODE_TYPE);
                Err(Error::InvalidState)
            }
        }
    };
}
