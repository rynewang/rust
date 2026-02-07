// Regression test for #151537: ICE when const-evaluating SIMD types with
// non-power-of-two element counts.
//
// The `check_simd_ptr_alignment` function assumed all SIMD types use
// `BackendRepr::SimdVector`, but non-power-of-two counts like `Simd<_, 3>`
// use `BackendRepr::Memory { sized: true }`.

//@ check-pass

#![feature(portable_simd, core_intrinsics)]

use std::intrinsics::simd::SimdAlign;
use std::{ptr::null, simd::prelude::*};

const _: () = {
    let c = Simd::from_array([0; 3]);
    unsafe {
        std::intrinsics::simd::simd_masked_store::<_, _, _, { SimdAlign::Element }>(
            c,
            null::<i32>(),
            c,
        )
    };
};

fn main() {}
