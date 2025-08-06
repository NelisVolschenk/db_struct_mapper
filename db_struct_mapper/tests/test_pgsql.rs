use db_struct_mapper::DbStruct;

#[test]
fn test_derive() -> () {
    #[derive(Default, Debug, DbStruct)]
    #[dbstruct(table_name="TestTable")]
    struct TestStruct {
        #[dbstruct(no_insert)]
        id: i64,
        name: String,
        password: String,
        #[dbstruct(foreign_key)]
        sub: SubTestStruct,
        #[dbstruct(get_values)]
        sub2: SubTestStruct,
    }

    #[derive(Debug, Default, DbStruct)]
    struct SubTestStruct {
        id: i64
    }
    let a = TestStruct::default();
}


// struct Tester {
//     
// }
// 
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