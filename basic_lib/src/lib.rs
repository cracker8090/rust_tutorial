// extern crate proc_macro;
// use proc_macro::TokenStream;
// use quote::quote;

// // using proc_macro_attribute to declare an attribute like procedural macro
// #[proc_macro_attribute]
// // _metadata is argument provided to macro call and _input is code to which attribute like macro attaches
// pub fn my_custome_attribute(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
//     // returing a simple TokenStream for Struct
//     TokenStream::from(quote! {struct H{}})
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod basic_data;
pub mod my_control;
pub mod my_error;
pub mod my_hashmap;
pub mod my_match;
pub mod my_method;
pub mod my_ownership;
pub mod my_panic;
pub mod my_smartpoints;
pub mod my_string;
pub mod my_struct;
pub mod my_variable;
pub mod my_file;