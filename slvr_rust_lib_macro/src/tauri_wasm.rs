use proc_macro::{TokenStream};
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{Expr, ImplItem, ItemImpl, ItemStruct, parse_macro_input, parse_quote, parse2, ExprLit, Lit, LitStr, Type};

pub(crate) fn command_attribute(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemStruct);

    input.attrs.push(parse_quote! {#[derive(Debug, slvr_rust_lib::deps::serde::Serialize, slvr_rust_lib::deps::serde::Deserialize)]});

    let name = input.ident.clone();

    quote! {
        #input
        impl slvr_rust_lib::tauri_wasm::MsgSafe for #name {}
    }
    .into()
}

pub(crate) fn command_impl_attribute(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);

    let struct_name: Ident = match &*input.self_ty {
        Type::Path(path) => {
            if let Some(name) = path.path.get_ident() {
                name.clone()
            } else {
                panic!("called impl macro on type with no identifier?");
            }
        },
        _ => panic!("called impl macro on non impl type?")
    };

    let command_name: Ident = {
        let mut try_name = None;
        for item in &input.items {
            if let ImplItem::Const(impl_const) = item {
                if &impl_const.ident == "COMMAND_NAME" {
                    if let Expr::Lit(lit) = impl_const.expr.clone() {
                        if let Lit::Str(lit_str) = lit.lit {
                            let value = lit_str.value();
                            try_name = Some(Ident::new(&value, Span::call_site()));
                        }
                    }
                    break;
                }
            }
        };
        if let Some(name) = try_name {
            name
        } else {
            panic!("Could not find command name in impl block, is it a string literal?");
        }
    };

    quote! {
        #input
        #[tauri::command]
        async fn #command_name(args: #struct_name) -> CheckScreenshotResp {
            args.handle().await
        }
    }
    .into()
}

pub(crate) fn response_attribute(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemStruct);

    input.attrs.push(parse_quote! {#[derive(Debug, slvr_rust_lib::deps::serde::Serialize, slvr_rust_lib::deps::serde::Deserialize)]});

    let name = input.ident.clone();

    quote! {
        #input
        impl slvr_rust_lib::tauri_wasm::MsgSafe for #name {}
        impl slvr_rust_lib::tauri_wasm::command::TauriResult for #name {}
    }
    .into()
}
