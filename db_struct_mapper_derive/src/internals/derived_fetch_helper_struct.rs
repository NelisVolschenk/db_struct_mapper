use syn::punctuated::Punctuated;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Path, PathSegment, Type, TypePath};
use crate::internals::struct_fields_parsed::StructWithFieldsParsed;

#[derive(Clone, Debug)]
pub struct DerivedFetchHelperStruct {
    pub ident: Ident,
    pub tokens: TokenStream,
    pub field_idents: Vec<Ident>, 
    pub field_types: Vec<Type>
}

impl From<StructWithFieldsParsed> for DerivedFetchHelperStruct {
    fn from(input_struct: StructWithFieldsParsed) -> Self {
        let parsed_fields = input_struct.parsed_fields;
        let fetch_helper_struct_name = input_struct.ident.to_string() + "FetchHelper";
        let fetch_helper_struct_name_ident = Ident::new(&fetch_helper_struct_name, Span::mixed_site());
        let fetch_helper_fields = parsed_fields
            .clone()
            .into_iter()
            .filter(|x| !x.associated_values)
            .collect::<Vec<_>>();
        let fetch_helper_idents = fetch_helper_fields
            .iter()
            .map(|x| Ident::new(&x.name, Span::mixed_site()))
            .collect::<Vec<_>>();
        let fetch_helper_types = fetch_helper_fields
            .iter()
            .map(|x| {
                let orig_type = x.orig_field.ty.clone();
                if x.foreign_key {
                    let p = Type::Path(
                        TypePath{
                            qself: None,
                            path: Path { 
                                leading_colon: None,
                                segments: { 
                                    let mut s = Punctuated::new();
                                    let ident = Ident::new("i64", Span::mixed_site());
                                    let pathseg: PathSegment = ident.into();
                                    s.push(pathseg);
                                    s
                                }
                            },
                        }
                    );
                    p
                }
                else {
                    orig_type 
                }
            })
            .collect::<Vec<_>>();

        let tokens = quote! {
            #[derive(FromRow)]
            pub struct #fetch_helper_struct_name_ident {
                #(
                    pub #fetch_helper_idents: #fetch_helper_types
                ),*
            }
        };

        Self {
            ident: fetch_helper_struct_name_ident,
            tokens,
            field_idents: fetch_helper_idents,
            field_types: fetch_helper_types
        }
    }
}