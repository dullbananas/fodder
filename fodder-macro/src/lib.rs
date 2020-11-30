use proc_macro::TokenStream;


#[proc_macro]
pub fn fodder(args: TokenStream) -> TokenStream {
    tokio::runtime::Runtime
        ::new()
        .unwrap()
        .block_on(async {
            fodder_utils::compiler
                ::main(args.into())
                .await
                .into()
        })
}
