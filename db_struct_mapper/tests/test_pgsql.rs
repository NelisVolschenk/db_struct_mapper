use db_struct_mapper::DbStruct;

#[test]
fn test_derive() -> () {

    let _a = TestStruct::default();
}

#[derive(Default, Debug, DbStruct)]
#[dbstruct(table_name="TestTable")]
struct TestStruct {
    #[dbstruct(primary_key)]
    id: i64,
    name: String,
    password: String,
    #[dbstruct(foreign_key)]
    sub: ForeignKeyStruct,
    #[dbstruct(associated_values)]
    sub2: Vec<MultiValueStruct>,
}

#[derive(Debug, Default, DbStruct)]
#[dbstruct(table_name="ForeignKeyTable")]
struct ForeignKeyStruct {
    #[dbstruct(primary_key)]
    id: i64,
    val1: i32
}

#[derive(Debug, Default, DbStruct)]
#[dbstruct(table_name="ReferenceTable")]
struct MultiValueStruct {
    #[dbstruct(primary_key)]
    id: i64,
    // val1: String,
    // val2: String,
    #[dbstruct(backref_key)]
    testtable_id: i64
}


// impl Tester {
//     fn insert_query(&self, table: &str) -> String {
//         let sqlquery = format!("insert into {} ( {} ) values ( {} ) returning *", table, "name,password", "$1,$2");
//         sqlquery
//     }
//     pub async fn insert<'e, E, T>(&self, executor: E, table: &str) -> sqlx::Result<T>
//     where
//         T: Send,
//         T: for<'c> sqlx::FromRow<'c, sqlx::postgres::PgRow>,
//         T: std::marker::Unpin,
//         E: sqlx::Executor<'e, Database=sqlx::Postgres>,
//     {
//         let sql = self.insert_query(table);
//         let res: T = sqlx::query_as::<_, T>(&sql).bind(&self.name).bind(&self.password).fetch_one(pool).await?;
//         Ok(res)
//     }
//     pub async fn insert_ex<'e, E>(&self, executor: E, table: &str) -> sqlx::Result<()>
//     where
//         E: sqlx::Executor<'e, Database=sqlx::Postgres>,
//     {
//         let sql = self.insert_query(table);
//         sqlx::query(&sql).bind(&self.name).bind(&self.password).execute(executor).await?;
//         Ok(())
//     }
// }