// Regression test for #138008: the lane count of a `#[repr(simd)]` type given by a
// `const` item must be evaluated when computing the layout.
//@ build-pass
//@ compile-flags: --crate-type=lib -Copt-level=0

#![feature(repr_simd)]

const C: usize = 16;

#[repr(simd)]
pub struct Foo([u8; C]);

pub unsafe fn foo(a: Foo) {}

pub const SIZE: usize = std::mem::size_of::<Foo>();
const _: () = assert!(SIZE == 16);
