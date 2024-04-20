#![feature(portable_simd)]
#![feature(slice_as_chunks)]
#![feature(maybe_uninit_array_assume_init)]
use arrayvec::ArrayVec;
use bitvec::prelude::*;
use bitvec::slice::BitSlice;
use cache_size::{l1_cache_line_size, l1_cache_size};
use columnar_derive::Columnar;
//#![feature(const_for)]
//#![feature(const_mut_refs)]
//#![feature(const_trait_impl)]
//use columnar_trait::BitMap;
use columnar_trait::{ArrayPtr, OptionMap, StructOfArrays};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::alloc::alloc;
use std::alloc::Layout;
// use std::simd::SimdFloat;
use std::slice::Chunks;
use std::{mem::MaybeUninit, simd::Simd, time::Duration};

fn bench_option_map(c: &mut Criterion, inputs: &[&[usize]]) {
    let name = std::any::type_name::<OptionMap>();
    for input in inputs {
        let options = OptionMap::from_slice(input);
        let len = input.len();
        let ones: u64 = input.into_iter().map(|&v| v as u64).sum();
        let frac = (ones as f64 / len as f64) * 100.0;
        // Check impl is correct
        let actual = options.contains_nones();
        let expected = ones > 0;
        assert_eq!(
            actual, expected,
            "{name}({len}, {frac}): {actual} {expected} {frac}"
        );
        c.bench_function(
            format!("{name}({len}, {frac:.2}% ones ({ones}/{len}))").as_str(),
            |b| b.iter(|| options.contains_nones()),
        );
    }
}

const fn gen_mix<const N: usize>(r: usize) -> [usize; N] {
    let mut array = [0_usize; N];
    let mut i = r;
    while i < array.len() {
        array[i] = 1;
        i += 1 + r;
    }
    array
}

#[derive(Columnar)]
pub struct FourFloats {
    f01: f32,
    f02: f32,
    f03: f32,
    f04: f32,
    f05: f32,
    f06: f32,
    f07: f32,
    f08: f32,
    f09: f32,
    f10: f32,
    f11: f32,
    f12: f32,
    f13: f32,
    f14: f32,
    f15: f32,
    f16: f32,
    f17: f32,
    f18: f32,
    f19: f32,
    f20: f32,
    f21: f32,
    f22: f32,
    f23: f32,
    f24: f32,
    f25: f32,
    f26: f32,
    f27: f32,
    f28: f32,
    f29: f32,
    f30: f32,
    f31: f32,
    f32: f32,
    f33: f32,
    f34: f32,
    f35: f32,
    f36: f32,
    f37: f32,
    f38: f32,
    f39: f32,
    f40: f32,
    f41: f32,
    f42: f32,
    f43: f32,
    f44: f32,
    f45: f32,
    f46: f32,
    f47: f32,
    f48: f32,
    f49: f32,
    f50: f32,
    f51: f32,
    f52: f32,
    f53: f32,
    f54: f32,
    f55: f32,
    f56: f32,
    f57: f32,
    f58: f32,
    f59: f32,
}

unsafe fn read_array<T: Copy, const N: usize>(ptr: *const T) -> [T; N] {
    let mut array = unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() };
    let slice: &[MaybeUninit<T>] = unsafe { std::slice::from_raw_parts(ptr.cast(), N) };
    array.copy_from_slice(slice);
    unsafe { MaybeUninit::array_assume_init(array) }
}

fn bench_struct_of_arrays(c: &mut Criterion) {
    let arrays = StructOfArrays::<FourFloatsPtrs>::new(128_000);
    c.bench_function("StructOfArrays::<FourFloatPtrs>::row", |b| {
        b.iter(|| black_box(arrays.row(0)))
    });
    c.bench_function("StructOfArrays::<FourFloatPtrs>::chunk<10>", |b| {
        b.iter(|| black_box(arrays.chunk::<10>(0)))
    });
    c.bench_function("StructOfArrays::<FourFloatPtrs>::iter_all", |b| {
        b.iter(|| {
            arrays
                .iter()
                .take(10240)
                .collect::<ArrayVec<FourFloats, 10240>>()
        })
    });
    c.bench_function(
        "StructOfArrays::<FourFloatPtrs>::iter_chunked<10>_all",
        |b| {
            b.iter(|| {
                arrays
                    .iter_chunked::<256>()
                    .take(1024)
                    .collect::<ArrayVec<FourFloats, 1024>>()
            })
        },
    );
    // c.bench_function("StructOfArrays::<FourFloatPtrs>::iter_all", |b| {
    //     b.iter(|| {
    //         arrays.iter().for_each(|e| {
    //             black_box(e);
    //         })
    //     })
    // });
}

fn bench_fib(c: &mut Criterion) {
    //c.warm_up_time(Duration::from_millis(500));
    //assert_eq!(gen_mix::<20>(2), [1_u8; 20]);
    const INPUTS: [&[usize]; 3] = [&[0; 12_800], &[0; 12_800_000], &gen_mix::<12_800_000>(8096)];

    bench_struct_of_arrays(c);
    //bench_option_map(c, &INPUTS);
    //let MIX = vec![0; 128_000];
    //let INPUTS = [vec![]];

    //const INPUTS: [&[u8]; 3] = [&[0; 10_000], &[1; 10_000], &[1; 100_000_000]];
    //const INPUTS: [&[u8]; 1] = [&[0; 10_000]];

    //bench_method(c, slice_wide_iter_equal, &inputs);
    //bench_method(c, slice_wide_sum_equal, &inputs);
    //bench_method(c, simd_sum, &INPUTS);
    //bench_method(c, simd_xor, &INPUTS);
    //bench_method(c, simd_and, &INPUTS);
    //bench_method(c, slice_iter_equal, &INPUTS);
    //bench_method(c, simd_u8_64, &INPUTS);
    //bench_method(c, simd_f64_64, &INPUTS);
    //bench_method(c, simd_wide_i16_16, &INPUTS);
    //bench_method(c, chunk_cmp_64, &INPUTS);
    //bench_method(c, simd_128_sum, &INPUTS);

    //bench_method(c, chunk_cmp_u8_64, &INPUTS);
    //bench_method(c, chunk_sum_equal, &INPUTS);
    //bench_method(c, slice_sum_equal, &INPUTS);
    //bench_method(c, bslice_set, &INPUTS);
    // bench_method(c, slice_128_sum, &INPUTS);
    // bench_method(c, slice_64_sum, &INPUTS);
    // bench_method(c, slice_32_sum, &INPUTS);
    //bench_method(c, chunk_cmp_u64_64, &INPUTS);

    //bench_method(c, simd_u8_64, &INPUTS);
    //bench_method(c, simd_u64_64, &INPUTS);
}

criterion_group!(benches, bench_fib);
criterion_main!(benches);
