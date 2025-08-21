#[diagnostic::on_unimplemented(
    note = "for local types consider adding `#[derive(db_struct_mapper::DbStruct)]` to your `{Self}` type",
    note = "for types from other crates check whether the crate offers a `db_struct_mapper` feature flag",
)]
pub trait DbStruct {
    type PrimaryKeyStruct;
    fn new_primary_key(pk: i64) -> Self::PrimaryKeyStruct;
    fn check_field_traits(&self) -> ();
}

pub trait PrimaryKeyStruct {
    fn new(pk: i64) -> Self;
}
pub fn is_db_struct(_field: impl DbStruct) {}


#[diagnostic::on_unimplemented(
    note = "for local types consider adding `#[derive(db_struct_mapper::DbStruct)]` to your `{Self}` type with one field marked as a backref key",
    note = "for types from other crates check whether the crate offers a `db_struct_mapper` feature flag",
)]
pub trait AssociatedDbStruct {}


pub fn is_associated_db_struct(_field: impl AssociatedDbStruct) {}