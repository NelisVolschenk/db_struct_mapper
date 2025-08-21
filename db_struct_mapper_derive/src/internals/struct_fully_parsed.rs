use proc_macro2::Ident;
use crate::internals::derived_fetch_helper_struct::DerivedFetchHelperStruct;
use crate::internals::derived_insert_helper_struct::DerivedInsertHelperStruct;
use crate::internals::parsed_field::ParsedField;
use crate::internals::derived_primary_key_struct::DerivedPrimaryKeyStruct;
use crate::internals::struct_fields_parsed::StructWithFieldsParsed;

#[derive(Clone, Debug)]
pub struct StructFullyParsed {
    pub ident: Ident,
    pub table_name: String,
    pub parsed_fields : Vec<ParsedField>,
    pub is_db_struct:bool,
    pub is_associated_db_struct: bool,
    pub primary_key_struct: DerivedPrimaryKeyStruct,
    pub fetch_helper_struct: DerivedFetchHelperStruct,
    pub insert_helper_struct: DerivedInsertHelperStruct
}

impl From<StructWithFieldsParsed> for StructFullyParsed {
    fn from(input_struct: StructWithFieldsParsed) -> Self {
        // Check if DbStruct should be implemented
        let has_table_name = !input_struct.table_name.is_empty();
        let has_primary_key = input_struct.parsed_fields.iter().any(|x| x.primary_key);
        let is_db_struct = has_table_name & has_primary_key;
        if !is_db_struct {panic!("DB Struct needs at least a table name and a primary key")}

        // Check if AssociatedDbStruct should be implemented
        let has_backref_key = input_struct.parsed_fields.iter().any(|x| x.backref_key);
        let is_associated_db_struct = is_db_struct & has_backref_key;
        
        let primary_key_struct = input_struct.clone().into();
        let fetch_helper_struct = input_struct.clone().into();
        let insert_helper_struct = input_struct.clone().into();

        Self {
            ident: input_struct.ident,
            table_name: input_struct.table_name,
            parsed_fields: input_struct.parsed_fields,
            is_db_struct,
            is_associated_db_struct,
            primary_key_struct,
            fetch_helper_struct,
            insert_helper_struct
        }
    }
}