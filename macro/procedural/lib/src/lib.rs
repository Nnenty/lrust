use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, ItemFn};

// functional macro
#[proc_macro]
pub fn var_name(inp: TokenStream) -> TokenStream {
    let inp_parse = parse_macro_input!(inp as Ident);
    let name = inp_parse.to_string();

    let ret = quote! {
        {
            println!("Name: {}, value: {}", #name, #inp_parse);
        }
    };

    TokenStream::from(ret)
}

// user-derive macro
#[proc_macro_derive(Descripe)]
pub fn derive_descripe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let gen = quote! {
        impl Description for #name {
            fn descripe() -> String{
                format!("Name of this variable: {}", stringify!(#name))
            }
        }
    };

    gen.into()
}

// attribute macro
#[proc_macro_attribute]
pub fn log_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block;

    let result = quote! {
        fn #fn_name() {
            println!("Entering {}", stringify!(#fn_name));
            #fn_body
            println!("Exiting {}", stringify!(#fn_name));
        }
    };

    result.into()
}
