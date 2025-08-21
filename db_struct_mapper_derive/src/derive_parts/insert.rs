use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use crate::internals::utils::{get_dollars, get_variable_names};
use crate::internals::parsed_field::ParsedField;
use crate::internals::struct_fully_parsed::StructFullyParsed;

pub fn generate_tokenstream(parsed_struct: StructFullyParsed) -> TokenStream {
    let parsed_fields = parsed_struct.parsed_fields;
    let insert_table = parsed_struct.table_name;

    let pk_type = parsed_struct.primary_key_struct.ident.clone();
    let orig_struct = parsed_struct.ident.clone();
    let insert_helper_struct_ident = parsed_struct.insert_helper_struct.ident.clone();
    
    let direct_insert_fields = parsed_fields
        .clone()
        .into_iter()
        .filter(|x| !x.no_insert & !x.primary_key & !x.associated_values)
        .collect::<Vec<ParsedField>>();
    let direct_insert_idents = direct_insert_fields
        .iter()
        .map(|x| &x.ident);
    let direct_insert_columns = direct_insert_fields
        .iter()
        .map(|x|x.name.clone())
        .collect::<Vec<_>>();

    let associated_insert_fields = parsed_fields
        .clone()
        .into_iter()
        .filter(|x| !x.no_insert & x.associated_values)
        .collect::<Vec<ParsedField>>();
    
    let insert_columns = direct_insert_columns
        .join(", ");
    let insert_dollars = get_dollars(direct_insert_fields.len());
    let primary_key_string = parsed_struct.primary_key_struct.field_idents
        .iter()
        .map(|x|x.to_string())
        .collect::<Vec<_>>()
        .join(", ");    
    
    
    let insert_sql = format!("insert into {} ( {} ) values ( {} ) returning {}", insert_table, insert_columns, insert_dollars, primary_key_string);
    quote! {
        
        impl #insert_helper_struct_ident {
            pub async fn insert(self, pool: &sqlx::PgPool) -> sqlx::Result<#pk_type> {
                let mut transaction = pool.begin().await?;
                let res = self.insert_ex(&mut *transaction).await;
                res
            }
            
            pub async fn insert_ex<'e, E>(self, executor: E) -> sqlx::Result<#pk_type>
            where:
                E: sqlx::Executor<'e,Database = sqlx::Postgres>
            {
                
            }
            
            
        }

        // pub async fn insert<T>(&self, pool: &sqlx::PgPool) -> sqlx::Result<T>
        // where
        //     T: Send,
        //     T: for<'c> sqlx::FromRow<'c, sqlx::postgres::PgRow>,
        //     T: std::marker::Unpin
        // {
        //     let mut transaction = pool.begin().await?;
        //     let res = self.insert_ex(&mut *transaction).await;
        //     res
        // }
        
        pub async fn insert_ex<'e, E, T>(&self, executor: E) -> sqlx::Result<T>
        where
            T: Send,
            T: for<'c> sqlx::FromRow<'c, sqlx::postgres::PgRow>,
            T: std::marker::Unpin,
            E: sqlx::Executor<'e,Database = sqlx::Postgres>
        {
            // First insert all foreign key values
            #(let #foreign_key_variables: i64 = self.#foreign_key_idents.insert_ex(executor).await?;)*
            // Then insert
            
            let res: T = sqlx::query_as::<_,T>(#insert_sql)
            #(
                .bind(&self.#normal_idents)
            )*
            #(
                .bind(#foreign_key_variables)
            )*
                .fetch_one(executor)
                .await?;

            Ok(res)
        }
            
        // pub async fn insert<T>(&self, pool: &sqlx::PgPool, table: &str) -> sqlx::Result<T>
        // where
        //     T: Send,
        //     T: for<'c> sqlx::FromRow<'c, sqlx::postgres::PgRow>,
        //     T: std::marker::Unpin
        // {
        //     let sql = self.insert_query(table);
        //     
        //     let res: T = sqlx::query_as::<_,T>(&sql)
        //     #(
        //         .bind(&self.#insert_idents)
        //     )*
        //         .fetch_one(pool)
        //         .await?;
        // 
        //     Ok(res)
        // }

        
        
    }
}