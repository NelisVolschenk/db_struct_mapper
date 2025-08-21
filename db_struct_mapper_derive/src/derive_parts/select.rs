use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{GenericArgument, PathArguments, Type};
use crate::internals::utils::{get_variable_names, sql_for_select_query};
use crate::internals::parsed_field::ParsedField;
use crate::internals::struct_fully_parsed::StructFullyParsed;

pub fn generate_tokenstream(parsed_struct: StructFullyParsed) -> TokenStream {
    let pk_type = parsed_struct.primary_key_struct.ident.clone();
    let pk_field_ident = parsed_struct.primary_key_struct.field_idents.first().unwrap().clone();
    let orig_struct = parsed_struct.ident.clone();
    let select_by_pk_sql = sql_for_select_query(
        parsed_struct.fetch_helper_struct.field_idents.iter().map(|x|x.to_string()).collect(),
        &parsed_struct.table_name,
        parsed_struct.primary_key_struct.field_idents.iter().map(|x|x.to_string()).collect()
    );
    let fetch_helper_struct_ident = parsed_struct.fetch_helper_struct.ident.clone();
    
    let direct_copy_idents = parsed_struct.parsed_fields
        .clone()
        .into_iter()
        .filter(|x| !x.foreign_key && !x.associated_values)
        .map(|x| Ident::new(&x.name, Span::mixed_site()))
        .collect::<Vec<Ident>>();

    let foreign_key_fields: Vec<ParsedField> = parsed_struct.parsed_fields
        .clone()
        .into_iter()
        .filter(|x| x.foreign_key)
        .collect();
    let foreign_key_idents = foreign_key_fields
        .iter()
        .map(|x| x.ident.clone().unwrap())
        .collect::<Vec<Ident>>();
    let foreign_key_variables = get_variable_names("foreign_key", foreign_key_fields.len())
        .into_iter()
        .map(|x| Ident::new(&x, Span::mixed_site()))
        .collect::<Vec<Ident>>();
    let foreign_key_pk_variables = get_variable_names("foreign_key_pk", foreign_key_fields.len())
        .into_iter()
        .map(|x| Ident::new(&x, Span::mixed_site()))
        .collect::<Vec<Ident>>();
    let foreign_key_types = foreign_key_fields
        .clone()
        .into_iter()
        .map(|x| x.orig_field.ty)
        .collect::<Vec<Type>>();

    let associated_value_fields: Vec<ParsedField> = parsed_struct.parsed_fields
        .clone()
        .into_iter()
        .filter(|x| x.associated_values)
        .collect();
    let associated_value_idents = associated_value_fields
        .iter()
        .map(|x| x.ident.clone().unwrap())
        .collect::<Vec<Ident>>();
    let associated_value_variables = get_variable_names("associated_value", associated_value_fields.len())
        .into_iter()
        .map(|x| Ident::new(&x, Span::mixed_site()))
        .collect::<Vec<Ident>>();
    let associated_value_types = associated_value_fields
        .clone()
        .into_iter()
        .map(|x| { 
            // println!("{:#?}", x.orig_field.ty);
            let Type::Path(path) = x.orig_field.ty else{todo!()};
            let a = path.path.segments.first().unwrap().clone().arguments;
            let PathArguments::AngleBracketed(b) = a
            else { todo!() };
            let c = b.args.first().unwrap().clone();
            let GenericArgument::Type(ret) = c else { todo!() };
            ret
        })
        .collect::<Vec<Type>>();
    
    let mut assoc_val_tokens = TokenStream::new();
    if parsed_struct.parsed_fields.iter().any(|x|x.backref_key) {
        let select_by_backref_key_sql = sql_for_select_query(
            parsed_struct.fetch_helper_struct.field_idents.iter().map(|x|x.to_string()).collect(),
            &parsed_struct.table_name,
            parsed_struct.primary_key_struct.field_idents.iter().map(|x|x.to_string()).collect()
        );
        assoc_val_tokens = quote! {
            pub async fn fetch_by_assocated_value(backref_key: i64, pool: &sqlx::PgPool) -> sqlx::Result<Vec<Self>> {
                let mut ret: Vec<Self> = Vec::new();
                let helper_vec: Vec<#fetch_helper_struct_ident> = sqlx::query_as(#select_by_backref_key_sql)
                    .bind(backref_key)
                    .fetch_all(pool)
                    .await?;
                for helper in helper_vec {
                    let s = Self::from_helper_struct(helper, pool).await;
                    match s {
                        Ok(t) => {ret.push(t)}
                        Err(e) => {return sqlx::Result::Err(e)}
                    }
                }
                Ok(ret)
            }
        }
    }
    
    
    quote! {
        impl #orig_struct {
            
            pub async fn from_helper_struct(helper: #fetch_helper_struct_ident, pool: &sqlx::PgPool) -> sqlx::Result<Self> {
                // Get Foreign Key Values
                #(let #foreign_key_pk_variables = #foreign_key_types::new_primary_key(helper.#foreign_key_idents);)*
                #(let #foreign_key_variables = #foreign_key_types::fetch_by_primary_key(#foreign_key_pk_variables, pool).await?;)*
                // Get Assocated Values
                let assoc_pk = helper.#pk_field_ident.clone();
                #(let #associated_value_variables = #associated_value_types::fetch_by_assocated_value(assoc_pk, pool).await?;)*
                let ret = Self {
                    #(#direct_copy_idents : helper.#direct_copy_idents,)*
                    #(#foreign_key_idents: #foreign_key_variables,)*
                    #(#associated_value_idents: #associated_value_variables,)*
                };
                Ok(ret)
            }
            
            pub async fn fetch_by_primary_key(pk: #pk_type, pool: &sqlx::PgPool) -> sqlx::Result<Self> {
                let primary_key = pk.#pk_field_ident.clone();
                let helper: #fetch_helper_struct_ident = sqlx::query_as(#select_by_pk_sql)
                    .bind(primary_key)
                    .fetch_one(pool)
                    .await?;
                Self::from_helper_struct(helper, pool).await
            }
            #assoc_val_tokens
        }
    }
}