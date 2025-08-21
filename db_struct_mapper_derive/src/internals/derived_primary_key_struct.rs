use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::Type;
use crate::internals::struct_fields_parsed::StructWithFieldsParsed;

#[derive(Clone, Debug)]
pub struct DerivedPrimaryKeyStruct {
    pub ident: Ident,
    pub tokens: TokenStream,
    pub field_idents: Vec<Ident>,
    pub field_types: Vec<Type>,
}

impl From<StructWithFieldsParsed> for DerivedPrimaryKeyStruct {
    fn from(input_struct: StructWithFieldsParsed) -> Self {
        
        let parsed_fields = input_struct.parsed_fields;
        let primary_key_struct_name = input_struct.ident.to_string() + "PrimaryKey";
        let primary_key_struct_name_ident = Ident::new(&primary_key_struct_name, Span::mixed_site());
        let primary_key_fields = parsed_fields
            .clone()
            .into_iter()
            .filter(|x| x.primary_key)
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
            #[derive(FromRow)]
            pub struct #primary_key_struct_name_ident {
                #(
                    pub #primary_key_idents: #primary_key_types
                ),*
            }
        };

        Self {
            ident: primary_key_struct_name_ident,
            tokens,
            field_idents: primary_key_idents,
            field_types: primary_key_types
        }
    }
}
