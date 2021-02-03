use proc_macro2::TokenStream;
use quote::quote;
use crate::{
    project::Application,
};
use std::{
    path::PathBuf,
};
use syn::{
    ItemMod,
};


pub async fn main(args: TokenStream, body: TokenStream) -> TokenStream {
    std::panic::set_hook(
        Box::new(crate::Error::report_panic)
    );

    match main_result(args, body).await {
        Ok(ok) => ok,
        Err(err) => {
            err.report().await;
            quote! {
                compile_error("");
            }
        },
    }
}


async fn main_result(args: TokenStream, body: TokenStream) -> crate::Result<TokenStream> {
    let ItemMod {
        ident: mod_ident,
        ..
    } =
        syn::parse2::<ItemMod>(body)
        .unwrap();

    let app_config = get_app_config(args).await?;
    let pkg_dir = get_pkg_dir()
        .await
        .map_err(crate::Error::Io)?;
    let client = reqwest::Client::new();
    app_config
        .install_dependencies(&pkg_dir, client)
        .await?;
    
    let ast = app_config
        .create_ast(&pkg_dir)
        .await?;
    
    let result = quote! {
        mod #mod_ident {}
    };
    
    println!("{}", result);
    Ok(result)
}


async fn get_app_config(args: TokenStream) -> crate::Result<Application> {
    let rel_path: String = syn
        ::parse2::<syn::LitStr>(args)
        .unwrap()
        .value();
    let mut path: PathBuf = std::env
        ::var("CARGO_MANIFEST_DIR")
        .unwrap()
        .into();
    path.push(rel_path);
    Application
        ::from_path(&mut path)
        .await
}


async fn get_pkg_dir() -> std::io::Result<PathBuf> {
    // TODO: read ELM_HOME environment variable
    let mut dir = dirs_next
        ::home_dir()
        .unwrap();
    dir.push(".elm/0.19.1/packages");
    tokio::fs::DirBuilder
        ::new()
        .recursive(true)
        .create(&dir)
        .await?;
    Ok(dir)
}
