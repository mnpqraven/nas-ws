use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro that automatically implements [`From`] for [`vercel_runtime::Response`] and a helper function to convert an Axum [`axum::response::Response`] to Vercel [`vercel_runtime::Response`]
#[proc_macro_derive(JsonResponse)]
pub fn response_derive_macro(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let name = &ident;
    let gen = quote! {
        impl From<&#name> for vercel_runtime::Response<Body> {
            fn from(value: &#name) -> Self {
                Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body::<Body>(serde_json::to_string(&value).unwrap().into())
                    .unwrap()
                }
        }
        impl From<#name> for vercel_runtime::Response<Body> {
            fn from(value: #name) -> Self {
                Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body::<Body>(serde_json::to_string(&value).unwrap().into())
                    .unwrap()
                }
        }

        impl FromAxumResponse<#name, WorkerError, vercel_runtime::Error>  for Result<Json<#name>, WorkerError> {
            type TFrom = Json<#name>;
            type TTo = Response<Body>;

            fn as_axum(&self) -> Result<Response<Body>, vercel_runtime::Error> {
                match self {
                    Ok(Json(val)) => Ok(Response::builder()
                                        .status(StatusCode::OK)
                                        .header("Content-Type", "application/json")
                                        .body(serde_json::to_string(val)?.into())?),
                    Err(e) => Err(e.to_string().into()),
                }
            }
        }
    };
    gen.into()
}
