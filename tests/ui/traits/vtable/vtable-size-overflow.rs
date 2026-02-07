// Regression test for #152030: ICE when building vtable for type with overflowing size
//
// Previously, `get_vtable_alloc` would panic with `.expect()` when
// `layout_of` returned `Err(SizeOverflow(...))`. Now it emits a proper
// fatal error instead.

#![feature(try_as_dyn)]

trait Trait {}
impl Trait for [u8; 1<<63] {}

pub fn foo(x: &[u8; 1<<63]) {
    let _ = std::any::try_as_dyn::<[u8; 1<<63], dyn Trait>(x);
    //~^ ERROR values of the type `[u8; 9223372036854775808]` are too big for the target architecture
}

fn main() {}
