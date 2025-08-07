use crate::internals::parsed_struct::ParsedStruct;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn generate_trait_impl(parsed_struct: ParsedStruct) -> TokenStream {
    let struct_name = parsed_struct.ident.clone();
    let db_struct_idents: Vec<Option<Ident>> = parsed_struct
        .parsed_fields
        .clone()
        .into_iter()
        .filter(|x|x.foreign_key)
        .map(|x| x.ident)
        .collect();
    let associated_db_struct_idents: Vec<Option<Ident>> = parsed_struct
        .parsed_fields
        .clone()
        .into_iter()
        .filter(|x|x.associated_values)
        .map(|x| x.ident)
        .collect();

    let mut db_struct_impl = TokenStream::new();
    let mut associated_db_struct_impl = TokenStream::new();
    
    // Check if DbStruct should be implemented
    let has_table_name = !parsed_struct.table_name.is_empty();
    let has_primary_key = parsed_struct.parsed_fields.iter().any(|x| x.primary_key);
    let is_db_struct = has_table_name & has_primary_key;
    
    if is_db_struct {
        db_struct_impl = quote! {
            impl db_struct_mapper_internal::DbStruct for #struct_name {
                fn check_isdbstruct(&self) -> ()
                {
                    #(
                    db_struct_mapper_internal::is_db_struct(&self.#db_struct_idents);
                    )*
                    #(
                    db_struct_mapper_internal::is_associated_db_struct(&self.#associated_db_struct_idents);
                    )*
                }
            }
        };
    }
    
    // Check if AssociatedDbStruct should be implemented
    let has_backref_key = parsed_struct.parsed_fields.iter().any(|x| x.backref_key);
    let is_associated_db_struct = is_db_struct & has_backref_key;
    if is_associated_db_struct {
        associated_db_struct_impl = quote! {
            impl db_struct_mapper_internal::AssociatedDbStruct for #struct_name {}
        }
    }
    
    
    quote! {
        #db_struct_impl
        #associated_db_struct_impl
    }
}
