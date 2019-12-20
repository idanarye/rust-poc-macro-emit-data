extern crate proc_macro;

#[proc_macro]
pub fn declaration(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let data = syn::parse_macro_input!(tokens as syn::Expr);
    quote::quote!(
        macro_rules! run_set_the_default {
            ($( $tokens:tt )*) => {
                poc_macro_emit_data::set_the_default!(#data $( $tokens )*);
            }
        }
    ).into()
}

#[derive(Debug)]
struct SetTheDefaultInput {
    the_default: syn::Expr,
    derive_input: syn::DeriveInput,
}

impl syn::parse::Parse for SetTheDefaultInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            the_default: input.parse()?,
            derive_input: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn set_the_default(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let SetTheDefaultInput { the_default, derive_input } = syn::parse_macro_input!(tokens as SetTheDefaultInput);
    let struct_name = derive_input.ident;
    quote::quote!(
        impl Default for #struct_name {
            fn default() -> Self {
                Self(#the_default)
            }
        }
    ).into()
}

#[proc_macro_derive(PresetDefault)]
pub fn preset_default(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens = proc_macro2::TokenStream::from(tokens);
    quote::quote!(
        run_set_the_default!{#tokens}
    ).into()
}
