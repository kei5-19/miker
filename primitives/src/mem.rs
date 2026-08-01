use core::marker::PhantomData;

/// Represents a physical address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysAddr(u64);

impl PhysAddr {
    /// Creates a new physical address from the 64-bit address.
    pub const fn new(addr: u64) -> Self {
        Self(addr)
    }

    /// Acquires the 64-bit representation of the address.
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    /// Returns whether the address is aligned.
    ///
    /// `align` must be a power of two.
    pub const fn is_aligned(self, align: u64) -> bool {
        // We use debug_assert instead of assert for optimization.
        debug_assert!(align.is_power_of_two());

        self.0 & (align - 1) == 0
    }
}

impl From<u64> for PhysAddr {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<PhysAddr> for u64 {
    fn from(value: PhysAddr) -> Self {
        value.as_u64()
    }
}

/// Represents a virtual address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtAddr(usize);

impl VirtAddr {
    /// Creates a new virtual address from the pointer-sized address.
    pub const fn new(addr: usize) -> Self {
        Self(addr)
    }

    /// Acquires the pointer-sized representation of the address.
    pub const fn as_usize(self) -> usize {
        self.0
    }

    /// Returns whether the address is aligned.
    ///
    /// `align` must be a power of two.
    pub const fn is_aligned(self, align: usize) -> bool {
        // We use debug_assert instead of assert for optimization.
        debug_assert!(align.is_power_of_two());

        self.0 & (align - 1) == 0
    }

    /// Returns the address rounded down to align as specified.
    ///
    /// `align` must be a power of two.
    pub const fn align_down(self, align: usize) -> Self {
        // We use debug_assert instead of assert for optimization.
        debug_assert!(align.is_power_of_two());

        Self(self.0 & !(align - 1))
    }

    /// Returns the address rounded up to align as specified.
    ///
    /// `align` must be a power of two.
    pub const fn align_up(self, align: usize) -> Self {
        // We use debug_assert instead of assert for optimization.
        debug_assert!(align.is_power_of_two());

        #[cfg(debug_assertions)]
        {
            Self(self.0.checked_add(align - 1).unwrap() & !(align - 1))
        }
        #[cfg(not(debug_assertions))]
        {
            Self((self.0 + align - 1) & !(align - 1))
        }
    }

    /// Acquires the `*mut T` at the address.
    pub const fn as_ptr<T>(self) -> *mut T {
        self.0 as _
    }

    /// Returns a shared reference to the value placed at the address.
    ///
    /// # Safety
    ///
    /// When calling this method, you have to ensure that the pointer at the address is
    /// [convertible to reference](https://doc.rust-lang.org/stable/std/ptr/index.html#pointer-to-reference-conversion).
    pub const unsafe fn as_ref<'a, T>(self) -> &'a T {
        // Safety: The caller must guarantee that the pointer at `self` is valid for reference.
        unsafe { self.as_ptr::<T>().as_ref_unchecked() }
    }

    /// Returns a exclusive reference to the value placed at the address.
    ///
    /// # Safety
    ///
    /// When calling this method, you have to ensure that the pointer at the address is
    /// [convertible to reference](https://doc.rust-lang.org/stable/std/ptr/index.html#pointer-to-reference-conversion).
    pub const unsafe fn as_mut<'a, T>(self) -> &'a mut T {
        // Safety: The caller must guarantee that the pointer at `self` is valid for reference.
        unsafe { self.as_ptr::<T>().as_mut_unchecked() }
    }
}

impl From<usize> for VirtAddr {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl From<VirtAddr> for usize {
    fn from(value: VirtAddr) -> Self {
        value.as_usize()
    }
}

/// Provides a size of a page.
pub trait PageSize: Clone + Copy + Send + Sync {
    /// A size of a page. It must be a power of two.
    const SIZE: usize;
}

/// Represents a page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Page<S> {
    addr: VirtAddr,
    _size: PhantomData<S>,
}

impl<S: PageSize> Page<S> {
    /// Creates a new page.
    ///
    /// `S::SIZE` must be a power of two and `addr` must be properly aligned to `S::SIZE`.
    pub const fn new(addr: VirtAddr) -> Self {
        const {
            assert!(
                S::SIZE.is_power_of_two(),
                "The specified page size is not a power of two",
            )
        };
        // We use debug_assert instead of assert for optimization.
        debug_assert!(addr.is_aligned(S::SIZE));

        Self {
            addr,
            _size: PhantomData,
        }
    }

    /// Returns the address of the page.
    pub const fn start(self) -> VirtAddr {
        self.addr
    }

    /// Returns the size of the page, which is equal to `<S as PageSize>::SIZE`.
    pub const fn size(self) -> usize {
        S::SIZE
    }
}

/// Represents a page frame that is mapped a page into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PageFrame<S> {
    addr: PhysAddr,
    _size: PhantomData<S>,
}

impl<S: PageSize> PageFrame<S> {
    /// Creates a new page frame.
    ///
    /// `S::SIZE` must be a power of two and `addr` must be properly aligned to `S::SIZE`.
    pub const fn new(addr: PhysAddr) -> Self {
        const {
            assert!(
                S::SIZE.is_power_of_two(),
                "The specified page size is not a power of two",
            )
        };
        // We use debug_assert instead of assert for optimization.
        debug_assert!(addr.is_aligned(S::SIZE as _));

        Self {
            addr,
            _size: PhantomData,
        }
    }

    /// Returns the address of the page frame.
    pub const fn addr(self) -> PhysAddr {
        self.addr
    }

    /// Returns the size of the page frame, which is equal to `<S as PageSize>::SIZE`.
    pub const fn size(self) -> usize {
        S::SIZE
    }
}
