use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use crate::internals::parsed_field::ParsedField;


#[derive(Clone, Debug)]
pub struct FieldsStruct {
    pub ident: Ident,
    pub tokens: TokenStream
}

impl FieldsStruct{
    pub fn from_ident_and_parsed_fields(ident: Ident, parsed_fields: Vec<ParsedField>) -> Self {
        let parsed_fields = parsed_fields;
        let primary_key_struct_name = ident.to_string() + "Fields";
        let primary_key_struct_name_ident = Ident::new(&primary_key_struct_name, Span::mixed_site());
        let primary_key_fields = parsed_fields
            .clone()
            .into_iter()
            .filter(|x| !x.primary_key)
            .collect::<Vec<_>>();
        let primary_key_idents = primary_key_fields
            .iter()
            .map(|x| Ident::new(&x.name, Span::mixed_site()))
            // .map(|x| x.name.clone())
            .collect::<Vec<_>>();
        let primary_key_types = primary_key_fields
            .iter()
            .map(|x| x.orig_field.ty.clone())
            .collect::<Vec<_>>();

        let tokens = quote! {
            pub struct #primary_key_struct_name_ident {
                #(
                    pub #primary_key_idents: #primary_key_types
                ),*
            }
        };

        Self {
            ident: primary_key_struct_name_ident,
            tokens,
        }

    }
}
