//mod columnar;
//#![feature(return_position_impl_trait_in_trait)]
#![feature(slice_as_chunks)]
mod columnar;
mod optional;
mod pointer;
//mod sequence;

//pub use columnar::{ArrayPtr, ArrayRow, StructOfArrays, StructOfMaybeArrays};
pub use columnar::{ArrayPtr, ArrayRow, StructOfArrays};
//pub use bitmap::BitMap;
pub use optional::OptionMap;
//pub use sequence::Sequence;
