struct Foo;

trait Bar {
    fn baz(&self);
}

#[impl_visibility::make(pub)]
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
