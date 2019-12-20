poc_macro_emit_data::declaration! {
    42
}

#[derive(Debug, poc_macro_emit_data::PresetDefault)]
struct Foo(u32);

fn main() {
    println!("The item set in the default is {}", Foo::default().0);
}
