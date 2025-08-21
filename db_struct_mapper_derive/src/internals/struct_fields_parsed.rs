use proc_macro2::Ident;
use syn::{Data, DataStruct, DeriveInput, Fields};
use crate::internals::parsed_field::ParsedField;
use crate::internals::struct_attributes_parsed::StructWithAttributesParsed;
use crate::internals::struct_fully_parsed::StructFullyParsed;

#[derive(Clone, Debug)]
pub struct StructWithFieldsParsed {
    pub ident: Ident,
    pub table_name: String,
    pub parsed_fields : Vec<ParsedField>,
    pub derive_input: DeriveInput
}

impl From<StructWithAttributesParsed> for StructWithFieldsParsed {
    fn from(input_struct: StructWithAttributesParsed) -> Self {
        let fields = match &input_struct.derive_input.data {
            Data::Struct(DataStruct {
                             fields: Fields::Named(fields),
                             ..
                         }) => &fields.named,
            _ => panic!("expected a struct with named fields"),
        };
        let parsed_fields : Vec<ParsedField> = fields
            .iter()
            .map(|x| x.clone().into())
            .collect();
        
        Self {
            ident: input_struct.ident,
            table_name: input_struct.table_name,
            parsed_fields,
            derive_input: input_struct.derive_input,
        }
    }
}

impl StructWithFieldsParsed {
    pub fn to_struct_fully_parsed(self) -> StructFullyParsed {
        self.into()
    }
}