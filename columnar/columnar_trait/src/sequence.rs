use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ops::Index;
use std::rc::Rc;
use std::slice::Iter;
use std::slice::SliceIndex;

pub trait Sequence<T>: Index<usize, Output = T> + IntoIterator<Item = T> {
    //type Iter: Iterator + ExactSizeIterator + DoubleEndedIterator;

    // fn iter<'a, I: Iterator<Item = &'a T>>(&self) -> I
    // where
    //     T: 'static;
    fn len(&self) -> usize;
    fn num_chunks(&self) -> usize;
    //fn chunk(&self, idx: usize) -> &[T];
}

//pub struct SequenceIter<'a, T, S: Sequence<T>> {
//
//}

//impl<T, S: Sequence<T>> ExactSizeIterator for S::IntoIter {}
//impl<I> ExactSizeIterator for I
//where {
//
//}

impl<C, T> Sequence<T> for C
where
    C: Deref<Target = [T]> + IntoIterator<Item = T> + Index<usize, Output = T>,
    C::IntoIter: ExactSizeIterator + DoubleEndedIterator,
{
    fn len(&self) -> usize {
        self.deref().len()
    }

    fn num_chunks(&self) -> usize {
        1
    }

    // fn chunk(&self, idx: usize) -> &[T] {
    //     assert!(idx == 0);
    //     self.deref()
    // }
}

pub struct A<S: Sequence<u64>> {
    a: S,
}

#[cfg(test)]
mod test {
    use super::Sequence;
    use super::A;

    fn foo<T, S: Sequence<T>>(s: S) {
        let len = s.len();
        let num_chunks = s.num_chunks();
        //let v = s.iter().collect::<Vec<_>>();
        //let v = s.into_iter().collect::<Vec<_>>();
    }

    #[test]
    fn single_col() {
        let a1 = A { a: vec![0_u64] };
        assert_eq!(a1.a.len(), 1);
        assert_eq!(a1.a.num_chunks(), 1);
        assert_eq!(a1.a[0], 0);
        assert_eq!(a1.a.into_iter().collect::<Vec<_>>(), vec![0_u64]);
        //assert_eq!(a1.a[0], 0);
        //let a2 = A { a: ArrayVec::<u64, 10>::default() };
        //let xs = &[0.0, 0.0];
        //let i = xs.into_iter();
        // let xs = vec![0.0; 100];
        // let chunks = xs.num_chunks();
        // assert_eq!(chunks, 1);
        // assert_eq!(xs.len(), 100)
    }
}
