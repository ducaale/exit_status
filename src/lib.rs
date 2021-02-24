extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn main(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::Item);
    let item_fn = match &ast {
        syn::Item::Fn(ref item_fn) => item_fn,
        _ => panic!("expected function, found something else")
    };

    let mut fn_output = match item_fn.sig.output.clone() {
        syn::ReturnType::Type(_, fn_output) => *fn_output,
        _ => panic!("expected function to return result")
    };
    let inner_fn_output = fn_output.clone();
    let inner_fn_body = &item_fn.block;

    if let syn::Type::Path(ref mut type_path) = fn_output {
        if let Some(last_segment) = type_path.path.segments.last_mut() {
            let arguments = &mut last_segment.arguments;
            if let syn::PathArguments::AngleBracketed(ref mut angle_bracketed) = arguments {
                let unit_type = syn::parse_quote!{ () };
                angle_bracketed.args[0] = unit_type;
            }
        }
    }

    let output = quote!{
        fn main() -> #fn_output {
            fn inner_main() -> #inner_fn_output {
                #inner_fn_body
            }

            std::process::exit(inner_main()?);
        }
    };

    output.into()
}
