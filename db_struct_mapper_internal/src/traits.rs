
#[diagnostic::on_unimplemented(
    note = "for local types consider adding `#[derive(db_struct_mapper::DbStruct)]` to your `{Self}` type",
    note = "for types from other crates check whether the crate offers a `db_struct_mapper` feature flag",
)]
pub trait IsDbStruct {
    fn check_isdbstruct(&self) -> ();
}

impl<T> IsDbStruct for &T
where
    T: IsDbStruct,
{
    fn check_isdbstruct(&self) -> () {
        ()
    }
}



pub fn isdbstruct(field: impl IsDbStruct) {}