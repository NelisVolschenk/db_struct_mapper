
use proc_macro2::Ident;
use syn::{parse2, Data, DataStruct, DeriveInput, Expr, Fields};
use syn::parse::Parser;
use crate::internals::parsed_field::ParsedField;
use crate::internals::symbols::{DB_STRUCT, TABLE_NAME};

#[derive(Clone)]
pub struct ParsedStruct {
    pub ident: Ident,
    pub table_name: String,
    pub parsed_fields : Vec<ParsedField>
}

impl From<DeriveInput> for ParsedStruct {
    fn from(input: DeriveInput) -> Self {
        
        let ident = input.ident.clone();
        let mut table_name = String::new();
        for attr in input.attrs.clone() {
            if attr.path() != DB_STRUCT {continue}
            if let syn::Meta::List(meta) = &attr.meta {
                if meta.tokens.is_empty() {
                    continue;
                }
                if let Ok(namevalue_args) = syn::punctuated::Punctuated::<syn::MetaNameValue, syn::Token![,]>::parse_terminated
                    .parse2(meta.tokens.clone()) {
                    for namevalue in namevalue_args {
                        if namevalue.path == TABLE_NAME {
                            if let Expr::Lit(val) = namevalue.value {
                                table_name = val.lit.suffix().to_string();
                            }
                            continue
                        }
                    }
                    continue
                }
                panic!("Only one type of identifier allowed per attribute")
            }
        }
        let fields = match &input.data {
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
            ident,
            table_name,
            parsed_fields
        }
    }
}