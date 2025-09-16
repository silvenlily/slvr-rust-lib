use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::parse::Parser;
use syn::{DeriveInput, parse_macro_input};
use crate::to_snake_case::ToSnakeCase;

pub fn encapsulate(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);

    let struct_name = ast.ident.clone();
    let target_name_str = &args.to_string();
    let target_name = Ident::new(target_name_str, Span::call_site());

    let field_name_str = String::from("encapsulated_") + &target_name_str.to_snake_case();
    let field_identifier = Ident::new(&field_name_str, Span::call_site());

    match &mut ast.data {
        syn::Data::Struct(struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote! { pub(self) #field_identifier : #target_name })
                            .expect("could not create encapsulated field"),
                    );
                }
                _ => (),
            }

            quote! {
                #ast

                impl Encapsulates<#target_name> for #struct_name {
                    fn encapsulated_get(&self) -> &#target_name {
                        &self.#field_identifier
                    }
                    fn encapsulated_get_mut(&mut self) -> &mut #target_name {
                        &mut self.#field_identifier
                    }
                    fn encapsulated_set(&mut self, value: #target_name)  {
                        self.#field_identifier = value;
                    }
                }
            }
            .into()
        }
        _ => panic!("encapsulate macro only works on structs"),
    }
}
