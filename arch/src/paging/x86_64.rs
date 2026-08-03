pub mod size {
    use primitives::PageSize;

    /// Represents a 4 KiB page size.
    #[derive(Debug, Clone, Copy)]
    pub struct PageSize4KB;

    impl PageSize for PageSize4KB {
        const SIZE: usize = 1 << 12;
    }

    /// Represents a 2 MiB page size.
    #[derive(Debug, Clone, Copy)]
    pub struct PageSize2MB;

    impl PageSize for PageSize2MB {
        const SIZE: usize = 1 << 21;
    }

    /// Represents a 1 GiB page size.
    #[derive(Debug, Clone, Copy)]
    pub struct PageSize1GB;

    impl PageSize for PageSize1GB {
        const SIZE: usize = 1 << 30;
    }
}
