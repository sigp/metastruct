# `metastruct`

Metastruct is Rust library for metaprogramming with struct fields.

Some of the things you can do with `metastruct` include:

- Iterate over a struct's fields.
- Map a closure over all or some of a struct's fields.
- Access the number of fields in a struct at compile-time via a `const`.

This is achieved by a procedural macro, which generates `macro_rules!` macros.

One way of understanding `metastruct` is as a shortcut to writing your own derive macros. If
you have a trait that you'd like to implement on a one-off basis, metastruct can help you write
that implementation without a derive macro.

## :construction: Under Construction :construction:

This library is currently under construction and should not be considered stable.

There's currently no documentation aside from a few scant code comments and examples/tests.

## License

Apache 2.0
