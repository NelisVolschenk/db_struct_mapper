use proc_macro2::Ident;
use syn::{Field};
use crate::internals::symbols::*;

#[derive(Clone)]
#[derive(Debug)]
pub struct ParsedField {
    pub orig_field: Field,
    pub name: String,
    pub ident: Option<Ident>,
    pub no_insert: bool,
    pub primary_key: bool,
    pub foreign_key: bool,
    pub associated_values: bool,
    pub backref_key: bool
}

impl From<Field> for ParsedField {
    fn from(orig_field: Field) -> Self {
        let name = orig_field
            .ident
            .clone()
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default();
        let ident = orig_field.ident.clone();
        let mut no_insert = false;
        let mut primary_key = false;
        let mut foreign_key = false;
        let mut associated_values = false;
        let mut backref_key = false;
        
        let attrs = orig_field.attrs.clone();
        for attr in attrs {
            if attr.path() != DB_STRUCT {continue}
            if let syn::Meta::List(meta) = &attr.meta {
                if meta.tokens.is_empty() {continue}
                let a  = attr.parse_nested_meta(|meta| {
                    if meta.path == NO_INSERT {no_insert = true}
                    if meta.path == PRIMARY_KEY {primary_key = true}
                    if meta.path == FOREIGN_KEY {foreign_key = true}
                    if meta.path == ASSOCIATED_VALUES {associated_values = true}
                    if meta.path == BACKREF_KEY {backref_key = true}
                    Ok(())
                }
                );
                a.unwrap()
            }
        }
        
        Self {
            orig_field,
            name,
            ident,
            no_insert,
            primary_key,
            foreign_key,
            associated_values,
            backref_key
        }
    }
}