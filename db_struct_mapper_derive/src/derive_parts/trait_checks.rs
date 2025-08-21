use proc_macro2::{Ident, TokenStream};
use quote::quote;
use crate::internals::struct_fully_parsed::StructFullyParsed;

pub fn generate_trait_impl(parsed_struct: StructFullyParsed) -> TokenStream {
    let struct_name = parsed_struct.ident.clone();
    let pk_type = parsed_struct.primary_key_struct.ident.clone();
    let pk_field_ident = parsed_struct.primary_key_struct.field_idents.first().unwrap().clone();
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
    
    
    if parsed_struct.is_db_struct {
        db_struct_impl = quote! {
            impl db_struct_mapper_internal::DbStruct for #struct_name {
                
                type PrimaryKeyStruct = #pk_type;
                
                fn new_primary_key(pk: i64) -> Self::PrimaryKeyStruct {
                    Self::PrimaryKeyStruct {
                        #pk_field_ident: pk
                    }
                }
                
                fn check_field_traits(&self) -> ()
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
    
    if parsed_struct.is_associated_db_struct {
        associated_db_struct_impl = quote! {
            impl db_struct_mapper_internal::AssociatedDbStruct for #struct_name {}
        }
    }
    
    quote! {
        #db_struct_impl
        #associated_db_struct_impl
    }
}
