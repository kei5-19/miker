//! Provides the abilities associated with paging.

/// Represents an access right to a page.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessRight {
    /// A page is not accessible.
    #[default]
    None,
    /// A page is readable.
    Read,
    /// A page is readable and writable.
    ReadWrite,
    /// A page is executable.
    Execute,
    /// A page is readable and executable.
    ReadExecute,
    /// A page is readable, writable and executable.
    ReadWriteExecute,
}

/// Represents a visibility scope of a page.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MappingScope {
    /// A page is accessible only from kernel space but a translation is global.
    GlobalKernel,
    /// A page is accessible only from kernel space and a translation is local.
    LocalKernel,
    /// A page is accessible from user space.
    #[default]
    User,
}

/// Represents a page-level cache policy.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CachePolicy {
    /// A normal cache policy.
    #[default]
    Normal,
    // TODO: Add other poilcies
}

/// Represents an attribute of a page.
pub struct PageAttributes {
    /// An access right.
    pub right: AccessRight,
    /// A scope.
    pub scope: MappingScope,
    /// A cahce policy.
    pub cache: CachePolicy,
}

impl PageAttributes {
    /// Creates a new page attribute for kernel data.
    pub const fn kernel_data() -> Self {
        Self {
            right: AccessRight::ReadWrite,
            scope: MappingScope::GlobalKernel,
            cache: CachePolicy::Normal,
        }
    }

    /// Creates a new page attribute for kernel code.
    pub const fn kernel_code() -> Self {
        Self {
            right: AccessRight::ReadExecute,
            scope: MappingScope::GlobalKernel,
            cache: CachePolicy::Normal,
        }
    }
}
