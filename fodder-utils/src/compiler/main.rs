use proc_macro2::TokenStream;
use quote::quote;
use crate::{
    project::{ElmJson, FromReader},
};
use std::{
    path::PathBuf,
};


pub async fn main(args: TokenStream) -> TokenStream {
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
    let app_config = match get_elm_json(args).await? {
        ElmJson::Application(c) => dbg!(c),
        ElmJson::Package(_) => todo!(),
    };
    let pkg_dir = get_pkg_dir().await?;
    let client = reqwest::Client::new();
    app_config
        .install_dependencies(&pkg_dir, client)
        .await?;
    
    let result = quote! {
        mod elm {}
    };
    
    println!("{}", result);
    Ok(result)
}


async fn get_elm_json(args: TokenStream) -> crate::Result<ElmJson> {
    let path: String = syn
        ::parse2::<syn::LitStr>(args)
        .unwrap()
        .value();
    let mut file = tokio::fs::File
        ::open(path)
        .await?;
    ElmJson
        ::from_reader(&mut file)
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
