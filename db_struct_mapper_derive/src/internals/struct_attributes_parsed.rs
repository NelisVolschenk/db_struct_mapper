use proc_macro2::Ident;
use syn::{DeriveInput, Expr, Lit};
use syn::parse::Parser;
use crate::internals::struct_fields_parsed::StructWithFieldsParsed;
use crate::internals::symbols::{DB_STRUCT, TABLE_NAME};

#[derive(Clone, Debug)]
pub struct StructWithAttributesParsed {
    pub ident: Ident,
    pub table_name: String,
    pub derive_input: DeriveInput
}

impl From<DeriveInput> for StructWithAttributesParsed {
    fn from(derive_input: DeriveInput) -> Self {
        let ident = derive_input.ident.clone();
        let mut table_name = String::new();
        for attr in derive_input.attrs.clone() {
            if attr.path() != DB_STRUCT {continue}
            if let syn::Meta::List(meta) = &attr.meta {
                if meta.tokens.is_empty() {
                    continue;
                }
                if let Ok(namevalue_args) = syn::punctuated::Punctuated::<syn::MetaNameValue, syn::Token![,]>::parse_terminated
                    .parse2(meta.tokens.clone()) {
                    for namevalue in namevalue_args {
                        if namevalue.path == TABLE_NAME {
                            if let Expr::Lit(exprlit) = namevalue.value {
                                if let Lit::Str(litstr) = exprlit.lit {
                                    table_name = litstr.value()
                                }
                            }
                            continue
                        }
                    }
                    continue
                }
                panic!("Only one type of identifier allowed per attribute")
            }
        }
        Self {
            ident,
            table_name,
            derive_input
        }
    }
}

impl StructWithAttributesParsed {
    
    pub fn from_derive_input(derive_input: DeriveInput) -> Self {
        derive_input.into()
    }
    pub fn to_struct_with_fields_parsed(self) -> StructWithFieldsParsed {
        self.into()
    }
}