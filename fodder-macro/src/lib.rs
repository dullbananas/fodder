use proc_macro::TokenStream;


#[proc_macro_attribute]
pub fn fodder(args: TokenStream, body: TokenStream) -> TokenStream {
    tokio::runtime::Runtime
        ::new()
        .unwrap()
        .block_on(
            fodder_utils::compiler
                ::main(
                    args.into(),
                    body.into(),
                )
        )
        .into()
}
