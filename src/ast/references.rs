use super::{AstStatement, DirectAccessType, SourceRange, AstId};

pub struct RefNode {
    pub element: AstReference,
    pub location: SourceRange,
    pub id: AstId,
}

#[derive(Clone, PartialEq, Debug)]
pub enum AstReference {
    /// a single name/reference. e.g. `a`
    Name(String),
    /// a pointer access. e.g. `a^`
    PointerAccess(Box<AstReference>),
    /// an array access. e.g. `a[0]`
    ArrayAccess(ArrayAccess),
    /// a direct access. e.g. `%X1`
    DirectAccess(DirectAccess),
}

impl AstReference {
    /// creates a new AstReference::Name
    pub fn new_name(name: String) -> Self {
        Self::Name(name)
    }

    /// creates a new AstReference::PointerAccess
    pub fn new_pointer_access(access: AstReference) -> Self {
        Self::PointerAccess(Box::new(access))
    }

    /// creates a new AstReference::ArrayAccess
    pub fn new_array_access(reference: AstReference, access: AstStatement) -> Self {
        Self::ArrayAccess(ArrayAccess::new(reference, access))
    }

    /// creates a new AstReference::DirectAccess
    pub fn new_direct_access(access: DirectAccessType, index: AstStatement) -> Self {
        Self::DirectAccess(DirectAccess::new(access, index))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ArrayAccess {
    reference: Box<AstReference>,
    access: Box<AstStatement>,
}

impl ArrayAccess {
    pub fn new(reference: AstReference, access: AstStatement) -> Self {
        Self { reference: Box::new(reference), access: Box::new(access) }
    }

    pub fn get_reference(&self) -> &AstReference {
        &self.reference
    }

    pub fn get_access(&self) -> &AstStatement {
        &self.access
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct DirectAccess {
    access: DirectAccessType,
    index: Box<AstStatement>,
}

impl DirectAccess {
    pub fn new(access: DirectAccessType, index: AstStatement) -> Self {
        Self { access, index: Box::new(index) }
    }

    pub fn get_access(&self) -> &DirectAccessType {
        &self.access
    }

    pub fn get_index(&self) -> &AstStatement {
        &self.index
    }
}
