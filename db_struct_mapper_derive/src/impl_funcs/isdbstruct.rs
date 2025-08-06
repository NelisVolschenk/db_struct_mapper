use crate::internals::parsed_struct::ParsedStruct;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn generate_trait_impl(parsed_struct: ParsedStruct) -> TokenStream {
    let struct_name = parsed_struct.ident;
    let db_struct_idents: Vec<Option<Ident>> = parsed_struct
        .parsed_fields
        .into_iter()
        .filter(|x|x.foreign_key | x.get_values)
        .map(|x| x.ident)
        .collect();

    quote! {
        impl db_struct_mapper_internal::IsDbStruct for &#struct_name {
            fn check_isdbstruct(&self) -> ()
            {
                #(
                db_struct_mapper_internal::isdbstruct(&self.#db_struct_idents);
                )*
                ()
            }
        }
    }
}
