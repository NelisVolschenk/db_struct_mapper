extern crate core;
use sqlx::FromRow;
use db_struct_mapper::DbStruct;
use sqlx::Row;

#[test]
fn test_derive() -> () {

    let _a = TestStruct::default();
    
    
}



async fn testfunc() {
    let _a = TestStruct::default();
    let pool = sqlx::PgPool::connect("DATABASE_URL").await.unwrap();
    let _b = 1i64;
    let _c: i32 = sqlx::query("").fetch_one(&pool).await.unwrap().try_get("sub").unwrap();
}


#[derive(Debug, Default, DbStruct)]
#[dbstruct(table_name = "TestTable")]
struct TestStruct {
    #[dbstruct(primary_key)]
    id: i64,
    name: String,
    password: String,
    #[dbstruct(foreign_key)]
    sub: ForeignKeyStruct,
    #[dbstruct(associated_values)]
    sub2: Vec<MultiValueStruct>,
    // #[dbstruct(foreign_key)]
    // sub3: NonDbStruct,
    // #[dbstruct(associated_values)]
    // sub4: Vec<NonDbStruct>,
}



#[derive(Debug, Default, DbStruct)]
#[dbstruct(table_name="ForeignkeyTable")]
struct ForeignKeyStruct {
    #[dbstruct(primary_key)]
    id: i64,
    val1: i32,
}


#[derive(Debug, Default, DbStruct)]
#[dbstruct(table_name="ReferenceTable")]
struct MultiValueStruct {
    #[dbstruct(primary_key)]
    id: i64,
    val1: String,
    val2: String,
    #[dbstruct(foreign_key)]
    val3: ForeignKeyStruct,
    #[dbstruct(backref_key)]
    testtable_id: i64
}
#[derive(Debug, Default)]
struct NonDbStruct {
    id: i64,
    val1: i32
}


// impl MultiValueStruct {
//     
//     pub async fn insert<T>(&self, pool: &sqlx::PgPool, table: &str) -> sqlx::Result<T>
//     where
//         T: Send,
//         T: for<'c> sqlx::FromRow<'c, sqlx::postgres::PgRow>,
//         T: std::marker::Unpin,
//     {
//         let mut transaction = pool.begin().await?;
//         self.insert_ex(&mut *transaction).await
//     }
//     pub async fn insert_ex<'e, E, T>(&self, executor: E) -> sqlx::Result<T>
//     where
//         T: Send,
//         T: for<'c> sqlx::FromRow<'c, sqlx::postgres::PgRow>,
//         T: std::marker::Unpin,
//         E: sqlx::Executor<'e, Database = sqlx::Postgres>,
//     {
//         let sql = self.insert_query("ReferenceTable");
//         let res: T = sqlx::query_as::<_, T>(&sql)
//             .bind(&self.id)
//             .bind(&self.testtable_id)
//             .fetch_one(executor)
//             .await?;
//         Ok(res)
//     }
// }
