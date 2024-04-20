use std::marker::PhantomData;
use std::ptr;

/// A trait for dealing with groups of pointers.
///
/// This trait allows for basic operations on a group of related pointers.
/// This is particularly useful for logically grouping physically unrelated data.
pub trait PointerGroup<'a>: Copy {
    /// The data described by this group of pointers.
    type Data;

    /// Reads grouped data described by `self`.
    unsafe fn read(self) -> Self::Data;

    /// Offsets all pointers in the group.
    unsafe fn offset(self, count: isize) -> Self;
}

impl<'a, T> PointerGroup<'a> for ptr::NonNull<T> {
    type Data = T;

    unsafe fn read(self) -> Self::Data {
        self.as_ptr().read()
    }

    unsafe fn offset(self, count: isize) -> Self {
        ptr::NonNull::new_unchecked(self.as_ptr().offset(count))
    }
}

impl<'a, T> PointerGroup<'a> for *const T {
    type Data = T;

    unsafe fn read(self) -> Self::Data {
        self.read()
    }

    unsafe fn offset(self, count: isize) -> Self {
        self.offset(count)
    }
}

impl<'a, T> PointerGroup<'a> for *mut T {
    type Data = T;

    unsafe fn read(self) -> Self::Data {
        self.read()
    }

    unsafe fn offset(self, count: isize) -> Self {
        self.offset(count)
    }
}
