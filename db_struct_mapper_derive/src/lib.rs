mod internals;
mod derive_parts;

extern crate proc_macro;
use self::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemMod};
use crate::derive_parts::{select, trait_checks};
use crate::internals::struct_attributes_parsed::StructWithAttributesParsed;

#[proc_macro_attribute]
pub fn db_struct_module(outer: TokenStream, inner: TokenStream) -> TokenStream {
    let input = parse_macro_input!(inner as ItemMod);
    println!("{:#?}", outer);
    println!("{:#?}", input);
    quote! {struct Check {id: i64}}.into()
}

#[proc_macro_derive(DbStruct, attributes(dbstruct))]
pub fn derive_from_struct_psql(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
        
    let parsed_struct = StructWithAttributesParsed::from_derive_input(input)
        .to_struct_with_fields_parsed()
        .to_struct_fully_parsed();
    
    // let struct_name = parsed_struct.ident.clone();
    
    let trait_impl = trait_checks::generate_trait_impl(parsed_struct.clone());
    let pk_struct = parsed_struct.primary_key_struct.tokens.clone();
    let fetch_helper_struct = parsed_struct.fetch_helper_struct.tokens.clone();
    let insert_helper_struct = parsed_struct.insert_helper_struct.tokens.clone();
    
    let mut impl_funcs: Vec<proc_macro2::TokenStream> = Vec::new();
    // impl_funcs.push(insert::generate_tokenstream(parsed_struct.clone()));
    impl_funcs.push(select::generate_tokenstream(parsed_struct.clone()));
    
    TokenStream::from(quote! {
        #pk_struct
        #fetch_helper_struct
        #insert_helper_struct
        #trait_impl
        //
        #(#impl_funcs)*
    })
    
    // // INSERT Attributes -> field names
    // let attributes = fields.iter().map(|field| &field.ident);
    // let attributes_ex = fields.iter().map(|field| &field.ident);
    // let attributes_vec: Vec<String> = fields
    //     .iter()
    //     .map(|field| {
    //         field
    //             .ident
    //             .as_ref()
    //             .map(ToString::to_string)
    //             .unwrap_or_default()
    //     })
    //     .collect();
    // 
    // // ( id, name, hostname .. )
    // let columns = attributes_vec.join(",");
    // // ( $1, $2)
    // let dollars = dollar_values(attributes_vec.len());
    // 
    // // UPDATE Attributes -> field names for
    // let attributes_update = fields.iter().map(|field| &field.ident);
    // let attributes_update_ex = fields.iter().map(|field| &field.ident);
    // // name = $2, hostname = $3
    // let pairs: String = attributes_vec
    //     .iter()
    //     .enumerate()
    //     .skip(1) // Skip the first element
    //     .map(|(index, value)| {
    //         let number = index + 1; // Start with $2
    //         format!("{} = ${}", value, number)
    //     })
    //     .collect::<Vec<String>>()
    //     .join(",");
    // 
    // TokenStream::from(quote! {
    //     impl #struct_name {
    //         fn insert_query(&self, table: &str) -> String
    //         {
    //             let sqlquery = format!("insert into {} ( {} ) values ( {} ) returning *", table, #columns, #dollars); // self.value_list()); //self.values );
    //             sqlquery
    //         }
    // 
    //         pub async fn insert<T>(&self, pool: &sqlx::PgPool, table: &str) -> sqlx::Result<T>
    //         where
    //             T: Send,
    //             T: for<'c> sqlx::FromRow<'c, sqlx::postgres::PgRow>,
    //             T: std::marker::Unpin
    //         {
    //             let sql = self.insert_query(table);
    // 
    //             // let mut pool = pool;
    //             let res: T = sqlx::query_as::<_,T>(&sql)
    //             #(
    //                 .bind(&self.#attributes) //         let #field_name: #field_type = Default::default();
    //             )*
    //                 .fetch_one(pool)
    //                 .await?;
    // 
    //             Ok(res)
    //         }
    // 
    //         pub async fn insert_ex<'e,E>(&self, executor: E, table: &str) -> sqlx::Result<()>
    //         where
    //             E: sqlx::Executor<'e,Database = sqlx::Postgres>
    //         {
    //             let sql = self.insert_query(table);
    // 
    //             // let mut pool = pool;
    //             sqlx::query(&sql)
    //             #(
    //                 .bind(&self.#attributes_ex) //         let #field_name: #field_type = Default::default();
    //             )*
    //                 .execute(executor)
    //                 .await?;
    // 
    //             Ok(())
    //         }
    // 
    //         fn update_query(&self, table: &str) -> String
    //         {
    //             let sqlquery = format!("update {} set {} where id = $1 returning *", table, #pairs);
    //             sqlquery
    //         }
    // 
    //         pub async fn update<T>(&self, pool: &sqlx::PgPool, table: &str) -> sqlx::Result<T>
    //         where
    //             T: Send,
    //             T: for<'c> sqlx::FromRow<'c, sqlx::postgres::PgRow>,
    //             T: std::marker::Unpin
    //         {
    //             let sql = self.update_query(table);
    // 
    //             // let mut pool = pool;
    //             let res: T = sqlx::query_as::<_,T>(&sql)
    //             #(
    //                 .bind(&self.#attributes_update)//         let #field_name: #field_type = Default::default();
    //             )*
    //                 .fetch_one(pool)
    //                 .await?;
    // 
    //             Ok(res)
    //         }
    // 
    // 
    //         pub async fn update_ex<'e,E>(&self, executor: E, table: &str) -> sqlx::Result<()>
    //         where
    //             E: sqlx::Executor<'e,Database = sqlx::Postgres>
    //         {
    //             let sql = self.update_query(table);
    // 
    //             sqlx::query(&sql)
    //             #(
    //                 .bind(&self.#attributes_update_ex)
    //             )*
    //                 .execute(executor)
    //                 .await?;
    // 
    //             Ok(())
    //         }
    //     }
    // })
}