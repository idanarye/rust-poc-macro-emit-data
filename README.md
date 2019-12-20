With Rust 1.40.0, procedural macros are allowed to declare `macro_rules!`
macros. This repository demonstrates how this new feature can be used to emit
data from one procedural macro and use it in another.

(For simplicity, the implementation assumes the exact structure of the
struct. Real world usages are going to be more complex)

The `poc_macro_emit_data::declaration!` macro is declaring another macro -
`run_set_the_default!`. `example.rs` is not using this macro directly -
instead, the `proc_macro_derive::PresetDefault` custom derive macro is. Instead
of emitting the `Default` impl directly, it emits:

```rust
run_set_the_default!{struct Foo(u32);}
```

Which expands to:

```rust
poc_macro_emit_data::set_the_default!(42 struct Foo(u32););
```

Because 42 is the value we fed to `poc_macro_emit_data::declaration!` and
`struct Foo(u32);` is the derive input of `PresetDefault`. `set_the_default!`
is doing the actual derive work (instead of `PresetDefault`), and receives both
the struct it needs to derive and the data set in `declaration!` - a totally
different macro invocation.
