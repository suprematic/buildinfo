use std::io::{Error, ErrorKind};

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn buildinfo(_input: TokenStream) -> TokenStream {
    const FILE_NAME: &str = "buildinfo.json";

    let buildinfo = slurp::read_all_to_string(FILE_NAME)
        .and_then(|input| {
            serde_json::from_str::<buildinfo::BuildInfo>(&input)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))
        })
        .map_err(|e| {
            let message = format!("failed to read {}: {}", FILE_NAME, e);
            quote! (compile_error!(#message);)
        });

    match buildinfo {
        Ok(_) => {
            quote! {
                //use buildinfo::v1::BuildInfo;

                static BUILDINFO: std::sync::LazyLock<buildinfo::BuildInfo> = std::sync::LazyLock::new(|| {
                    buildinfo::BuildInfo {
                        x: "Hello, world!".to_string()
                    }
                });
            }
        }
        Err(e) => e,
    }
    .into()
}
