use crate::{AssociatedDbStruct, DbStruct};

impl<T> DbStruct for &T
where
    T: DbStruct,
{
    type PrimaryKeyStruct = T::PrimaryKeyStruct;

    fn new_primary_key(pk: i64) -> Self::PrimaryKeyStruct {
        T::new_primary_key(pk)
    }

    fn check_field_traits(&self) -> () {
        T::check_field_traits(self)
    }
}

impl<T> AssociatedDbStruct for &T
where
    T: AssociatedDbStruct,
{}

impl<T> AssociatedDbStruct for Vec<T>
where
    T: AssociatedDbStruct,
{}