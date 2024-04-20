#[cfg(feature = "bitvec")]
use bitvec::{slice::BitSlice, view::AsBits};
use std::ops::Deref;
use std::rc::Rc;

pub type BitContainer = usize;

#[derive(Debug, Clone, PartialEq)]
pub struct OptionMap(Rc<[BitContainer]>);

impl OptionMap {
    pub const NONES_CHUNK_SIZE: usize = 2560;
    pub const STORAGE_BITS: u32 = BitContainer::BITS;

    /// Constructs a new `OptionMap` where every element is `Some`.
    ///
    /// # Examples
    ///
    /// ```
    /// use columnar_trait::OptionMap;
    /// let mut options = OptionMap::new_full(1024);
    /// ```
    pub fn new_full(len: usize) -> Self {
        Self(vec![0; len].into())
    }

    /// Constructs a new `OptionMap` where every element is `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use columnar_trait::OptionMap;
    /// let mut options = OptionMap::new_empty(1024);
    /// ```
    pub fn new_empty(len: usize) -> Self {
        Self(vec![1; len].into())
    }

    /// Constructs a new `OptionMap` using the provided BitContainer slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use columnar_trait::OptionMap;
    /// let mut options = OptionMap::from_slice(&[0; 10]);
    /// ```
    pub fn from_slice(slice: &[BitContainer]) -> Self {
        Self(slice.into())
    }

    #[cfg(feature = "bitvec")]
    /// Provides a BitSlice as a view over this `OptionMap`.
    ///
    /// # Examples
    ///
    /// ```
    /// use columnar_trait::OptionMap;
    /// let options = OptionMap::from_slice(&[0; 10]);
    /// let bits = options.as_bits();
    /// assert!(!bits.any());
    /// assert_eq!(*bits.get(0).unwrap().as_ref(), false);
    /// ```
    pub fn as_bits(&self) -> &BitSlice {
        self.0.as_bits()
    }

    #[inline]
    /// Checks if there is any 'None' contained in the OptionMap.
    ///
    /// # Examples
    ///
    /// ```
    /// use columnar_trait::OptionMap;
    /// assert!(OptionMap::new_empty(OptionMap::NONES_CHUNK_SIZE + 1).contains_nones());
    /// assert!(!OptionMap::new_full(OptionMap::NONES_CHUNK_SIZE + 1).contains_nones());
    /// ```
    pub fn contains_nones(&self) -> bool {
        let bytes: &[u8] = unsafe { std::mem::transmute(self.0.deref()) };
        let (chunks, rem) = bytes.as_chunks::<2560>();
        rem.into_iter().sum::<u8>() != 0
            || chunks
                .into_iter()
                .any(|chunk| chunk.into_iter().sum::<u8>() != 0)
    }

    // pub fn storage_len(&self) -> usize {
    //     self.0.len() / (Self::STORAGE_BITS as usize)
    //         + ((self.0.len() % (Self::STORAGE_BITS as usize)) > 1) as usize
    // }
}

#[cfg(feature = "bitvec")]
impl AsRef<BitSlice> for OptionMap {
    fn as_ref(&self) -> &BitSlice {
        self.as_bits()
    }
}

//impl Deref for OptionMap {
//    type Target = Rc<[BitContainer]>;
//    fn deref(&self) -> &Self::Target {
//        &self.0
//    }
//}

#[cfg(test)]
mod test {

    #[test]
    fn test_ff() {
        assert_eq!(1, 1);
    }

    #[cfg(feature = "bitvec")]
    mod bitvec {

        #[test]
        fn test_foo() {
            //foo()
        }
    }
}

// impl AsRef<[usize]> for OptionMap {
//     fn as_ref(&self) -> &[usize] {
//         unsafe { std::slice::from_raw_parts(self.as_bitptr().pointer(), self.storage_len()) }
//     }
// }

// #[inline(always)]
// pub const fn mask_range(byte: u8, start: u8, end: u8) -> u8 {
//     assert!(
//         start < (u8::BITS as u8) && end < (u8::BITS as u8) && start < end,
//         "Start and End must be in range 0 -> 7 and start must be before end."
//     );
//     let len = end - start;

//     const ONES: u8 = 0xFFFF;
//     0
//     //(byte >> start)
//     //(byte >> bit) & 0xF
// }

// #[inline(always)]
// pub fn get_bit(byte: u8, bit: u8) -> u8 {
//     (byte >> bit) & 0xF
// }

// #[inline(always)]
// pub const fn get_byte_and_bit_offset(offset: usize) -> (usize, u8) {
//     let byte_offset = offset / 8;
//     let bit_offset = (offset - (byte_offset * 8)) as u8;
//     (byte_offset, bit_offset)
// }
