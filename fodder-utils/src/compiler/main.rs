use proc_macro2::TokenStream;
use quote::quote;
use crate::{
    project::Application,
};
use std::{
    path::PathBuf,
};


pub async fn main(args: TokenStream) -> TokenStream {
    /*std::panic::set_hook(Box::new(|info| {
        crate::Error::report_panic(info);
    }));*/

    match main_result(args).await {
        Ok(ok) => ok,
        Err(err) => {
            err.report().await;
            quote! {
                compile_error("");
            }
        },
    }
}


async fn main_result(args: TokenStream) -> crate::Result<TokenStream> {
    let app_config = get_app_config(args).await?;
    let pkg_dir = get_pkg_dir().await?;
    let client = reqwest::Client::new();
    app_config
        .install_dependencies(&pkg_dir, client)
        .await?;
    
    let ast = app_config
        .create_ast()
        .await?;
    
    let result = quote! {
        mod elm {}
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


async fn get_pkg_dir() -> crate::Result<PathBuf> {
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
