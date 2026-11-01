// Regression test for #149156: a layout error found by the ABI check must be
// reported even when codegen never runs.
//@ build-fail
//@ compile-flags: -Zno-codegen
//@ only-64bit

#![crate_type = "lib"]
#![allow(private_interfaces)]

struct Struct([u8; 0xffff_ffff_ffff_ffff]);

pub fn function(value: Struct) -> u8 { //~ ERROR are too big for the target architecture
    value.0[0]
}
