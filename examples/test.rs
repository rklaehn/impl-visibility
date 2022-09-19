use public_impl::public_impl;

struct Foo;

trait Bar {
    fn baz(&self);
}

#[public_impl]
impl Foo {
    fn test() {}

    const SIZE: usize = 1;
}

// #[public_impl]
// impl Bar for Foo {
//     fn baz(&self) {}
// }

fn main() {
}
