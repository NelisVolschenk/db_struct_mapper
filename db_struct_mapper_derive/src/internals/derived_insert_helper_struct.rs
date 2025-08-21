use syn::punctuated::Punctuated;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{GenericArgument, Path, PathArguments, PathSegment, Type, TypePath};
use crate::internals::struct_fields_parsed::StructWithFieldsParsed;
use crate::internals::utils::{get_inner_type_as_string, get_vec_for_type};

#[derive(Clone, Debug)]
pub struct DerivedInsertHelperStruct {
    pub ident: Ident,
    pub tokens: TokenStream,
    pub field_idents: Vec<Ident>,
    pub field_types: Vec<Type>
}

impl From<StructWithFieldsParsed> for DerivedInsertHelperStruct {
    fn from(input_struct: StructWithFieldsParsed) -> Self {
        let parsed_fields = input_struct.parsed_fields;
        let insert_helper_struct_name = input_struct.ident.to_string() + "InsertHelper";
        let insert_helper_struct_name_ident = Ident::new(&insert_helper_struct_name, Span::mixed_site());
        let insert_helper_fields = parsed_fields
            .clone()
            .into_iter()
            .filter(|x| !x.primary_key)
            .collect::<Vec<_>>();
        let insert_helper_idents = insert_helper_fields
            .iter()
            .map(|x| Ident::new(&x.name, Span::mixed_site()))
            .collect::<Vec<_>>();
        let insert_helper_types = insert_helper_fields
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
                else if x.associated_values {
                    println!("{:#?}", x.orig_field.ty);
                    let inner = get_inner_type_as_string(x.orig_field.ty.clone()).unwrap();
                    let new_inner = inner + "InsertHelper";
                    let new_type = get_vec_for_type(&new_inner).unwrap();
                    new_type
                    // let Type::Path(a) = x.orig_field.ty.clone() else { todo!() };
                    // let b = a.path.segments.first().unwrap().clone().arguments;
                    // let PathArguments::AngleBracketed(c) = b else { todo!() };
                    // let d = c.args.first().unwrap().clone();
                    // let GenericArgument::Type(e) = d else { todo!() };
                    // let Type::Path(f) = e else { todo!() };
                    // let g = f.path.segments.first().unwrap().clone().ident.to_string();
                    // let p = Type::Path(
                    //     TypePath{
                    //         qself: None,
                    //         path: Path {
                    //             leading_colon: None,
                    //             segments: {
                    //                 let mut s = Punctuated::new();
                    //                 let typename = g + "InsertHelper";
                    //                 let ident = Ident::new(&typename, Span::mixed_site());
                    //                 let pathseg: PathSegment = ident.into();
                    //                 s.push(pathseg);
                    //                 s
                    //             }
                    //         },
                    //     }
                    // );
                    // p
                }
                else {
                    orig_type
                }
            })
            .collect::<Vec<_>>();

        let tokens = quote! {
            pub struct #insert_helper_struct_name_ident {
                #(
                    pub #insert_helper_idents: #insert_helper_types
                ),*
            }
        };

        Self {
            ident: insert_helper_struct_name_ident,
            tokens,
            field_idents: insert_helper_idents,
            field_types: insert_helper_types
        }
    }
}