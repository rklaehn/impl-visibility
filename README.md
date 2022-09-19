# public_impl

A simple procedural macro to make everything in an impl section public.

The purpose is to make impl sections public depending on a feature flag, using [cfg_attr](https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute). See example below.

E.g. you have a crate with experimental or advanced features that you want
to hide by default to avoid confusion.

```rust
#[cfg_attr(feature = "experimental", public_impl)]
impl Foo {
    fn experimental1(&self) {
        ...
    }

    const EXPERIMENTAL_CONFIG_1: usize = 1;
}
```

You can also use this macro to avoid having to write `pub` in front of all members, but in that case I would recommend just writing `pub` a few times and avoiding the dependency.
