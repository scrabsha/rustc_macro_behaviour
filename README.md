# A weird macro expansion behaviour

Consider the following snippet of code:

```rust
quote! {
    struct Foo {
        // Zero or more fields
    };  // Notice the semicolon here

    impl Trait for Foo {
        // ...
    }
}
```

This is more or less the content of a proc\_macro in this crate. The `;` after
the `}` is a syntax-error in Rust (see [a playground], and [the references]).
For some reasons, no error is triggered for this `;`, but the `impl` block
following it is discarded.

This block is not discarded by quote (see [another playground]), and it also
keeps the `;`, so it seems that it is a bug in rustc.

This behaviour happens on stable, beta and nightly rust.

## Expected result:

"You should remove the `;` here"-like error.

## Current result:

Next impl block removed, no error raised.

## Prove it!

This repo contains a workspace with several crates:

  - `core` contains the trait definition of `Cake`,
  - `macro_generator` contains a proc\_macro which declares a struct and
implements `Cake` for it,
  - `exporter` re-exports the trait defined in core, and the proc\_macro, so
that they are avalaible for the end-user,
  - the `user` crate is responsible for calling the proc\_macro, just as an user
would do.

In order to test this crate, you can run `cargo check` on either the stable,
beta or nightly releases of rust.


[a playground]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=5c5a37542564f2a0467dfb271db722e5
[the references]: https://doc.rust-lang.org/reference/items/structs.html
[another playground]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=71edd34b15377745ed0be659a8aaefc6
