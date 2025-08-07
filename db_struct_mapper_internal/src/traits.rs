
#[diagnostic::on_unimplemented(
    note = "for local types consider adding `#[derive(db_struct_mapper::DbStruct)]` to your `{Self}` type",
    note = "for types from other crates check whether the crate offers a `db_struct_mapper` feature flag",
)]
pub trait DbStruct {
    fn check_isdbstruct(&self) -> ();
}

impl<T> DbStruct for &T
where
    T: DbStruct,
{
    fn check_isdbstruct(&self) -> () {
        ()
    }
}

pub fn is_db_struct(_field: impl DbStruct) {}

pub trait AssociatedDbStruct {}

impl<T> AssociatedDbStruct for &T
where
    T: AssociatedDbStruct,
{}

impl<T> AssociatedDbStruct for Vec<T>
where
    T: AssociatedDbStruct,
{}


pub fn is_associated_db_struct(_field: impl AssociatedDbStruct) {}