use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ImplItem, ItemFn, ItemImpl};

#[proc_macro_attribute]
pub fn add_1_to_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let attrs = &input.attrs;
    let sig = &input.sig;
    let body = &input.block.stmts[0];
    TokenStream::from(quote! {
        #(#attrs)*
        #sig {
            #body + 1
        }
    })
}

#[proc_macro_attribute]
pub fn add_1_to_implementation(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let attrs = &input.attrs;
    let name = &input.self_ty;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let stms = &input
        .items
        .iter()
        .map(|i| match i {
            ImplItem::Fn(func) => {
                let func_attrs = &func.attrs;
                let sig = &func.sig;
                let body = &func.block.stmts[0];
                quote! {
                    #(#func_attrs)*
                    #sig {
                        #body + 1
                    }
                }
            }
            _ => panic!("Expected a method"),
        })
        .collect::<Vec<_>>();
    TokenStream::from(quote! {
        #(#attrs)*
        impl #impl_generics #name #ty_generics #where_clause {
            #(#stms)*
        }
    })
}
