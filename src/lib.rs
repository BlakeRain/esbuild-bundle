use proc_macro::TokenStream;

mod javascript;

#[proc_macro]
pub fn javascript(input: TokenStream) -> TokenStream {
    javascript::process(input)
}
