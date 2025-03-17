use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_attribute]
pub fn default_value(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as DeriveInput);

    quote!().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
