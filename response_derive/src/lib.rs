use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro that automatically implements [`From`] for [`vercel_runtime::Response`] and a helper function to convert an Axum [`axum::response::Response`] to Vercel [`vercel_runtime::Response`]
#[proc_macro_derive(OthiResponse)]
pub fn response_derive_macro(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let name = &ident;
    let gen = quote! {
        impl From<#name> for vercel_runtime::Response<Body> {
            fn from(value: #name) -> Self {
                Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body::<Body>(serde_json::to_string(&value).unwrap().into())
                    .unwrap()
                }
        }
        impl FromAxumResponse<WorkerError> for #name {}
    };
    gen.into()
}