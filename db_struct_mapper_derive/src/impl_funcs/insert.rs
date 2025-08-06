use proc_macro2::TokenStream;
use quote::quote;
use crate::internals::common::get_dollars;
use crate::internals::parsed_field::ParsedField;
use crate::internals::parsed_struct::ParsedStruct;

pub fn generate_tokenstream(parsed_struct: ParsedStruct) -> TokenStream {
    let parsed_fields = parsed_struct.parsed_fields;
    let insert_table = parsed_struct.table_name;
    let insert_fields: Vec<ParsedField> = parsed_fields
        .clone()
        .into_iter()
        .filter(|x| !x.no_insert & !x.foreign_key & !x.get_values)
        .collect();

    let insert_idents = insert_fields
        .iter()
        .map(|x| &x.ident);

    let insert_columns = insert_fields
        .iter()
        .map(|x|x.name.clone())
        .collect::<Vec<_>>()
        .join(",");

    let insert_dollars = get_dollars(insert_fields.len());
    quote! {
        fn insert_query(&self, table: &str) -> String
        {
            let sqlquery = format!("insert into {} ( {} ) values ( {} ) returning *", table, #insert_columns, #insert_dollars);
            sqlquery
        }
        
        pub async fn insert<'e, E, T>(&self, executor: E) -> sqlx::Result<T>
        where
            T: Send,
            T: for<'c> sqlx::FromRow<'c, sqlx::postgres::PgRow>,
            T: std::marker::Unpin,
            E: sqlx::Executor<'e,Database = sqlx::Postgres>
        {
            let sql = self.insert_query(#insert_table);
            
            let res: T = sqlx::query_as::<_,T>(&sql)
            #(
                .bind(&self.#insert_idents)
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