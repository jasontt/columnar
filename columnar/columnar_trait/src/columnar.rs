use super::OptionMap;
use cache_size::{l1_cache_line_size, l1_cache_size};
use std::iter::IntoIterator;
use std::ops::Deref;
use std::ops::Index;
use std::{mem::MaybeUninit, time::Duration};

mod group {
    use super::super::pointer::PointerGroup;
    use std::{marker::PhantomData, ptr::NonNull};

    pub struct AB {
        a: f64,
        b: f64,
    }

    pub struct ABC {
        ab: AB,
        c: f64,
    }

    #[derive(Clone, Copy)]
    pub struct ABPointers {
        a: NonNull<f64>,
        b: NonNull<f64>,
    }

    #[derive(Clone, Copy)]
    pub struct ABPointersV2<const N: usize> {
        a: NonNull<()>,
        _n: PhantomData<[(); N]>,
    }

    #[derive(Clone, Copy)]
    pub struct ABCPointers {
        ab: ABPointers,
        c: NonNull<f64>,
    }

    // impl<'a> PointerGroup<'a> for ABPointers<'a> {
    //     type Data = AB;

    //     unsafe fn read(self) -> Self::Data {
    //         let a = self.a.as_ptr().read();
    //         let b = self.b.as_ptr().read();
    //         Self::Data { a, b }
    //     }

    //     unsafe fn offset(self, count: isize) -> Self {
    //         Self {
    //             a: NonNull::new_unchecked(self.a.as_ptr().offset(count)),
    //             b: NonNull::new_unchecked(self.b.as_ptr().offset(count)),
    //             _lt: PhantomData::default(),
    //         }
    //     }
    // }

    // impl<'a> PointerGroup<'a> for ABCPointers<'a> {
    //     type Data = ABC;

    //     unsafe fn read(&self, offset: isize) -> Self::Data {
    //         let ab = self.ab.read(offset);
    //         let c = self.c.as_ptr().offset(offset).read();
    //         Self::Struct { ab, c }
    //     }
    // }

    // pub struct StructOfArrays<'a, A: ArrayPtr> {
    //     a: A,
    //     len: usize,
    //     _lt: PhantomData<&'a ()>,
    // }

    // impl<'a, A: ArrayPtr> StructOfArrays<'a, A> {
    //     fn read(&self, offset: isize) -> A::Struct {
    //         assert!(offset >= 0 && offset as usize <= self.len);
    //         unsafe { self.a.read(offset) }
    //     }
    // }

    // pub struct StructOfArraysIter<'a, A: ArrayPtr> {
    //     a: StructOfArrays<'a, A>,
    //     idx: usize,
    // }

    // impl<'a, A: ArrayPtr> Iterator for StructOfArraysIter<'a, A> {
    //     type Item = A::Struct;

    //     fn next(&mut self) -> Option<Self::Item> {
    //         // TODO: consider if this inlines out the assert
    //         if self.idx <= self.a.len {
    //             let e = self.a.read(self.idx as isize);
    //             self.idx += 1;
    //             Some(e)
    //         } else {
    //             None
    //         }
    //     }
    // }

    // impl<'a, A: ArrayPtr> IntoIterator for StructOfArrays<'a, A> {
    //     type Item = A::Struct;
    //     type IntoIter = StructOfArraysIter<'a, A>;

    //     fn into_iter(self) -> Self::IntoIter {
    //         Self::IntoIter { a: self, idx: 0 }
    //     }
    // }

    // impl<'a, A: ArrayPtr> Sequence for StructOfArrays<'a, A> {
    //     type Element = A::Struct;
    // }
}

mod sequence {
    use std::{marker::PhantomData, ptr::NonNull};

    pub trait Sequence: IntoIterator<Item = Self::Element> {
        type Element;
        fn at(&self, index: usize) -> Self::Element {
            todo!()
        }
    }

    pub trait ArrayPtr {
        type Struct;
        unsafe fn read(&self, offset: isize) -> Self::Struct;
    }

    pub struct AB {
        a: f64,
        b: f64,
    }

    pub struct ABC {
        ab: AB,
        c: f64,
    }

    pub struct ABPointers<'a> {
        a: NonNull<f64>,
        b: NonNull<f64>,
        _lt: PhantomData<&'a ()>,
    }

    impl<'a> ArrayPtr for ABPointers<'a> {
        type Struct = AB;
        unsafe fn read(&self, offset: isize) -> Self::Struct {
            let a = self.a.as_ptr().offset(offset).read();
            let b = self.b.as_ptr().offset(offset).read();
            Self::Struct { a, b }
        }
    }

    pub struct ABCPointers<'a> {
        ab: ABPointers<'a>,
        c: NonNull<f64>,
        _lt: PhantomData<&'a ()>,
    }

    impl<'a> ArrayPtr for ABCPointers<'a> {
        type Struct = ABC;
        unsafe fn read(&self, offset: isize) -> Self::Struct {
            let ab = self.ab.read(offset);
            let c = self.c.as_ptr().offset(offset).read();
            Self::Struct { ab, c }
        }
    }

    pub struct StructOfArrays<'a, A: ArrayPtr> {
        a: A,
        len: usize,
        _lt: PhantomData<&'a ()>,
    }

    impl<'a, A: ArrayPtr> StructOfArrays<'a, A> {
        fn read(&self, offset: isize) -> A::Struct {
            assert!(offset >= 0 && offset as usize <= self.len);
            unsafe { self.a.read(offset) }
        }
    }

    pub struct StructOfArraysIter<'a, A: ArrayPtr> {
        a: StructOfArrays<'a, A>,
        idx: usize,
    }

    impl<'a, A: ArrayPtr> Iterator for StructOfArraysIter<'a, A> {
        type Item = A::Struct;

        fn next(&mut self) -> Option<Self::Item> {
            // TODO: consider if this inlines out the assert
            if self.idx <= self.a.len {
                let e = self.a.read(self.idx as isize);
                self.idx += 1;
                Some(e)
            } else {
                None
            }
        }
    }

    impl<'a, A: ArrayPtr> IntoIterator for StructOfArrays<'a, A> {
        type Item = A::Struct;
        type IntoIter = StructOfArraysIter<'a, A>;

        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter { a: self, idx: 0 }
        }
    }

    impl<'a, A: ArrayPtr> Sequence for StructOfArrays<'a, A> {
        type Element = A::Struct;
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    impl<'a, T: Copy> Sequence for &'a [T] {
        type Element = &'a T;
    }

    // impl<'a> IntoIterator for ABArrays<'a> {
    //     type Item = AB;
    //     type IntoIter = ArraysIter<'a, Self>;

    //     fn into_iter(self) -> Self::IntoIter {
    //         ArraysIter {
    //             a: self,
    //             idx: 0,
    //             _lt: Default::default(),
    //         }
    //     }
    // }

    // impl<'a> Sequence for ABArrays<'a> {
    //     type Element = AB;
    // }

    // impl<'a> IntoIterator for ABArrays<'a> {
    //     fn into_iter(self) -> Self::IntoIter {}
    // }
}

mod refs {
    use std::iter::Copied;
    use std::iter::{FlatMap, Flatten};
    use std::marker::PhantomData;
    use std::slice::Iter;

    pub trait ArrayRef<'a>: Clone {
        type Row;
        type Iter: Iterator<Item = Self::Row>;
        //type Row: ArrayRow;
        //fn new(len: usize) -> Self;
        fn iter(&self) -> Self::Iter;
        fn row(&self, idx: usize) -> Self::Row;
        //unsafe fn chunk<const N: usize>(&self, idx: usize) -> [Self::Row; N];
    }

    impl<'a, T: Copy> ArrayRef<'a> for &'a [T] {
        type Row = T;
        type Iter = Copied<Iter<'a, T>>;

        #[inline]
        fn row(&self, idx: usize) -> Self::Row {
            self[idx]
        }

        #[inline]
        fn iter(&self) -> Self::Iter {
            self.into_iter().copied()
        }
    }

    #[derive(Clone)]
    pub struct ChunkedArrayRef<'a, A: ArrayRef<'a>>(&'a [A]);

    impl<'a, A: ArrayRef<'a>> Copy for ChunkedArrayRef<'a, A> {}

    fn foo() {
        let inner = vec![0.0, 0.0];
        let refs = vec![inner.as_slice()];
        let ca = ChunkedArrayRef(refs.as_slice());

        let refs = ca.0;
        let iter = refs.into_iter().flat_map(|inner| inner.into_iter());
    }

    impl<'a, A: ArrayRef<'a>> ArrayRef<'a> for ChunkedArrayRef<'a, A> {
        type Row = A::Row;
        type Iter = ChunkedArrayIter<'a, A>;

        #[inline]
        fn row(&self, idx: usize) -> Self::Row {
            todo!()
        }

        #[inline]
        fn iter(&self) -> Self::Iter {
            Self::Iter {
                a: *self,
                chunk: 0,
                idx: 0,
            }
        }
    }

    #[derive(Clone)]
    pub struct ChunkedArrayIter<'a, A: ArrayRef<'a>> {
        a: ChunkedArrayRef<'a, A>,
        chunk: usize,
        idx: usize,
    }

    impl<'a, A: ArrayRef<'a>> Iterator for ChunkedArrayIter<'a, A> {
        type Item = A::Row;

        fn next(&mut self) -> Option<Self::Item> {
            todo!()
        }
    }

    //impl<'b, 'a, A: ArrayRef<'a>> ArrayRef<'b> for &'b [&'a A] {}

    // pub trait ArrayRow {
    //     type Ptr: ArrayPtr;
    // }
    #[derive(Debug, Clone, PartialEq)]
    pub struct StructOfArrays<'a, T: ArrayRef<'a>> {
        pub inner: T,
        pub len: usize,
        _lt: PhantomData<&'a T>,
    }

    impl<'a, T: ArrayRef<'a>> StructOfArrays<'a, T> {
        // pub fn new(len: usize) -> Self {
        //     let inner = T::new(len);
        //     Self { inner, len }
        // }

        #[inline]
        pub fn row(&self, idx: usize) -> Option<T::Row> {
            if idx < self.len {
                unsafe { Some(self.inner.row(idx)) }
            } else {
                None
            }
        }

        #[inline]
        pub fn iter(
            &self,
        ) -> impl Iterator<Item = T::Row> + ExactSizeIterator + DoubleEndedIterator + '_ {
            (0..self.len).map(|i| unsafe { self.inner.row(i) })
        }
    }

    #[cfg(test)]
    mod test {
        use super::ArrayRef;

        #[derive(Default, Debug, Clone, Copy, PartialEq)]
        pub struct XStruct {
            a: u64,
            b: f64,
        }

        // #[derive(Debug, Clone, PartialEq)]
        // pub struct XColumnarInner<'array> {
        //     a: &'array [u64],
        //     b: &'array [f64],
        // }

        // impl<'a> ArrayRef<'a> for XColumnarInner<'a> {
        //     type Row = XStruct;

        //     fn row(&self, idx: usize) -> Self::Row {
        //         Self::Row {
        //             a: self.a.row(idx),
        //             b: self.b.row(idx),
        //         }
        //     }
        // }

        //impl

        // #[test]
        // fn single_column() {
        //     let xs = XColumnar { a: vec![0; 100] };
        //     let x = xs.row(0);
        //     let xs_a: Vec<_> = xs.iter().map(|x| x.a).collect();
        //     let xs_a_consuming: Vec<_> = xs.clone().into_iter().map(|x| x.a).collect();
        //     let xs_transpose = xs.clone().transpose();

        //     assert_eq!(xs.len(), 100);
        //     assert_eq!(x, Some(XStruct { a: 0 }));
        //     assert_eq!(xs_a, vec![0; 100]);
        //     assert_eq!(xs_a_consuming, xs_a);
        //     assert_eq!(xs_transpose, vec![XStruct { a: 0 }; 100]);
        // }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructOfArrays<T: ArrayPtr> {
    pub inner: T,
    pub len: usize,
}

impl<T: ArrayPtr> StructOfArrays<T> {
    pub fn new(len: usize) -> Self {
        let inner = T::new(len);
        Self { inner, len }
    }

    #[inline]
    pub fn row(&self, idx: usize) -> Option<T::Row> {
        if idx < self.len {
            unsafe { Some(self.inner.row(idx)) }
        } else {
            None
        }
    }

    #[inline]
    pub fn chunk<const N: usize>(&self, idx: usize) -> [T::Row; N] {
        unsafe { self.inner.chunk(idx) }
    }

    #[inline]
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = T::Row> + ExactSizeIterator + DoubleEndedIterator + '_ {
        (0..self.len).map(|i| unsafe { self.inner.row(i) })
    }

    #[inline]
    pub fn iter_chunked<const N: usize>(
        &self,
    ) -> impl Iterator<Item = T::Row> + DoubleEndedIterator + '_ {
        (0..self.len / N).flat_map(|i| {
            let chunk: [_; N] = unsafe { self.inner.chunk(i * N) };
            chunk.into_iter()
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructOfMaybeArrays<T: ArrayPtr> {
    inner: T,
    nones: OptionMap,
    len: usize,
}

impl<T: ArrayPtr> StructOfMaybeArrays<T> {
    pub fn new(len: usize) -> Self {
        Self {
            inner: T::new(0),
            nones: OptionMap::new_empty(len),
            len,
        }
    }

    pub fn row(&self, idx: usize) -> Option<T::Row> {
        if idx < self.len {
            None
        } else {
            None
        }
    }
}

impl<T: ArrayPtr> From<StructOfMaybeArrays<T>> for StructOfArrays<T> {
    fn from(value: StructOfMaybeArrays<T>) -> Self {
        assert!(!value.nones.contains_nones());
        Self {
            inner: value.inner,
            len: value.len,
        }
    }
}

pub trait ArrayPtr {
    type Row: ArrayRow;
    fn new(len: usize) -> Self;
    unsafe fn row(&self, idx: usize) -> Self::Row;
    unsafe fn chunk<const N: usize>(&self, idx: usize) -> [Self::Row; N];
}

pub trait ArrayRow {
    type Ptr: ArrayPtr;
}

unsafe fn read_array<T: Copy, const N: usize>(ptr: *const T) -> [T; N] {
    let slice: &[T] = unsafe { std::slice::from_raw_parts(ptr.cast(), N) };
    slice.try_into().unwrap_unchecked()
}

macro_rules! impl_prim_array_ptr {
    ($prim:ty) => {
        impl ArrayRow for $prim {
            type Ptr = *const $prim;
        }

        impl ArrayPtr for *const $prim {
            type Row = $prim;

            fn new(len: usize) -> Self {
                unsafe {
                    std::alloc::alloc(std::alloc::Layout::array::<$prim>(len).unwrap()).cast()
                }
            }

            #[inline]
            unsafe fn row(&self, idx: usize) -> Self::Row {
                self.offset(idx as isize).read()
            }

            #[inline]
            unsafe fn chunk<const N: usize>(&self, idx: usize) -> [Self::Row; N] {
                read_array(self.offset(idx as isize))
            }
        }
    };
}

impl_prim_array_ptr!(f32);
impl_prim_array_ptr!(f64);
impl_prim_array_ptr!(i16);
impl_prim_array_ptr!(i32);
impl_prim_array_ptr!(i64);
impl_prim_array_ptr!(i128);
impl_prim_array_ptr!(u16);
impl_prim_array_ptr!(u32);
impl_prim_array_ptr!(u64);
impl_prim_array_ptr!(usize);
impl_prim_array_ptr!(isize);
impl_prim_array_ptr!(bool);
impl_prim_array_ptr!(char);

//pub trait ColumnIndex2 {
//    type Struct;
//    unsafe fn row(&self, idx: usize) -> Self::Struct;
//}

// pub trait Columnar {
//     type Struct;
//     fn row(&self, idx: usize) -> Option<Self::Struct>;
//     unsafe fn row_unchecked(&self, idx: usize) -> Self::Struct {
//         self.row(idx).unwrap_unchecked()
//     }
//     fn len(&self) -> usize;
//     fn iter(&self) -> Iter<'_, Self>
//     where
//         Self: Sized,
//     {
//         Iter {
//             idx: 0,
//             columnar: self,
//         }
//     }
//     fn into_iter(self) -> IntoIter<Self>
//     where
//         Self: Sized,
//     {
//         IntoIter {
//             idx: 0,
//             columnar: self,
//         }
//     }
//     fn transpose(self) -> Vec<Self::Struct>
//     where
//         Self: Sized,
//     {
//         self.into_iter().collect()
//     }
// }

// pub struct Iter<'a, T: Columnar> {
//     idx: usize,
//     columnar: &'a T,
// }

// impl<T: Columnar> Iterator for Iter<'_, T> {
//     type Item = T::Struct;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.idx < self.columnar.len() {
//             let next = unsafe { Some(self.columnar.row_unchecked(self.idx)) };
//             self.idx += 1;
//             next
//         } else {
//             None
//         }
//     }
// }

// pub struct IntoIter<T: Columnar> {
//     idx: usize,
//     columnar: T,
// }

// impl<T: Columnar> Iterator for IntoIter<T> {
//     type Item = T::Struct;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.idx < self.columnar.len() {
//             let next = unsafe { Some(self.columnar.row_unchecked(self.idx)) };
//             self.idx += 1;
//             next
//         } else {
//             None
//         }
//     }
// }

// impl<T: Columnar> ExactSizeIterator for IntoIter<T> {
//     fn len(&self) -> usize {
//         self.columnar.len() - self.idx
//     }
// }

#[cfg(test)]
mod test {
    //use super::Columnar;
    //use super::Iter;
    //use super::{ColumnIndex, Sequence};
    // use std::ptr::NonNull;

    fn box_contig() -> Box<[f32]> {
        vec![0.0; 1024].into()
    }

    // unsafe fn box_contig3() -> Box<[f32]> {
    //     //let layout = std::alloc::Layout::array::<f32>(1024).expect("foo");
    //     Box::new_zeroed_slice(1024)
    //     //let ptr: *mut[f32; ] = std::alloc::alloc_zeroed(layout).cast();
    //     //Box::f
    // }

    #[test]
    fn test_size() {
        let a = std::vec::from_elem(0.0, 1023);

        let v = vec![0.0; 1023];
        assert_eq!(v.capacity(), 1023);

        let bv: Box<[_]> = Box::from(v);
        assert_eq!(bv.len(), 1023);
    }

    // #[derive(Default, Debug, Clone, Copy, PartialEq)]
    // pub struct XStruct {
    //     a: u64,
    //     b: f64,
    // }

    // #[derive(Debug, Clone, PartialEq)]
    // pub struct XColumnarInner {
    //     a: NonNull<u64>,
    //     b: NonNull<f64>,
    // }

    // impl ColumnIndex for XColumnarInner {
    //     type Struct = XStruct;
    //     unsafe fn row(&self, idx: usize) -> Self::Struct {
    //         todo!()
    //     }
    // }

    // impl<A: Sequence<u64>> Columnar for XColumnar<A> {
    //     type Struct = XStruct;

    //     #[inline]
    //     fn row(&self, idx: usize) -> Option<Self::Struct> {
    //         if idx < self.a.len() {
    //             Some(Self::Struct {
    //                 a: *self.a.index(idx),
    //             })
    //         } else {
    //             None
    //         }
    //     }

    //     #[inline]
    //     fn len(&self) -> usize {
    //         self.a.len()
    //     }
    // }

    // #[test]
    // fn single_column() {
    //     let xs = XColumnar { a: vec![0; 100] };
    //     let x = xs.row(0);
    //     let xs_a: Vec<_> = xs.iter().map(|x| x.a).collect();
    //     let xs_a_consuming: Vec<_> = xs.clone().into_iter().map(|x| x.a).collect();
    //     let xs_transpose = xs.clone().transpose();

    //     assert_eq!(xs.len(), 100);
    //     assert_eq!(x, Some(XStruct { a: 0 }));
    //     assert_eq!(xs_a, vec![0; 100]);
    //     assert_eq!(xs_a_consuming, xs_a);
    //     assert_eq!(xs_transpose, vec![XStruct { a: 0 }; 100]);
    // }
}
