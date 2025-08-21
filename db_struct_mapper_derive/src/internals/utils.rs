use std::iter::zip;
use proc_macro2::{Ident, Span};
use quote::format_ident;
use syn::{Error, GenericArgument, Path, PathArguments, PathSegment, Type, TypePath};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

/// 2 -> ( $1,$2 )
pub fn get_dollars(max: usize) -> String {
    get_dollars_vec(max).join(", ")
}

pub fn get_dollars_vec(max:usize) -> Vec<String> {
    let itr = 1..max + 1;
    itr.into_iter()
        .map(|s| format!("${}", s))
        .collect::<Vec<String>>()
}

pub fn get_variable_names(prefix: &str, max:usize) -> Vec<String> {
    let itr = 1..max + 1;
    itr.into_iter()
        .map(|x| format!("{}_{}", prefix, x))
        .collect()
}

pub fn sql_for_select_query(columns: Vec<String>, tablename: &str, match_keys: Vec<String>) -> String {
    let column_str = columns.join(", ");
    let pk_dollars = get_dollars_vec(match_keys.len());
    let pk_str = zip(match_keys, pk_dollars)
        .map(|x| format!("{} = {}",x.0, x.1))
        .collect::<Vec<String>>()
        .join(" AND ");
    format!("SELECT ({column_str}) FROM {tablename} WHERE {pk_str}")
}


pub fn get_type_string(ty: Type) -> syn::Result<String> {
    let sp = ty.span().clone();
    let Type::Path(f) = ty else { return Err(Error::new(sp, "Should be a path type")) };
    let g = f.path.segments.first().unwrap().clone().ident.to_string();
    Ok(g)
}

pub fn get_inner_type(ty: Type) -> syn::Result<Type> {
    let sp = ty.span().clone();
    let Type::Path(a) = ty else { return Err(Error::new(sp, "Should be a path type")) };
    let b = a.path.segments.first().unwrap().clone().arguments;
    let PathArguments::AngleBracketed(c) = b else { return Err(Error::new(sp, "Should be a container type")) };
    let d = c.args.first().unwrap().clone();
    let GenericArgument::Type(e) = d else { return Err(Error::new(sp, "Not sure what this should be")) };
    Ok(e)
}

pub fn get_inner_type_as_string(ty: Type) -> syn::Result<String> {
    let inner = get_inner_type(ty)?;
    get_type_string(inner)
}

pub fn get_outer_type(ty: Type) -> syn::Result<String> {
    let sp = ty.span().clone();
    let Type::Path(a) = ty else { return Err(Error::new(sp, "Should be a path type")) };
    let b = a.path.segments.first().unwrap().clone().ident;
    Ok(b.to_string())
}

pub fn get_vec_for_type(input_type: &str) -> syn::Result<Type> {
    let input_string = "Vec<".to_string() + input_type + ">";
    let parsed_type: Result<Type, syn::Error> = syn::parse_str(&input_string);
    parsed_type
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     #[test]
//     fn range_test() {
//         let itr = 1..4;
//         let res = itr
//             .into_iter()
//             .map(|s| format!("${}", s))
//             .collect::<Vec<String>>()
//             .join(",");
// 
//         assert_eq!(res, "$1,$2,$3");
//     }
// 
//     #[test]
//     fn dollar_value_tes() {
//         let res = get_dollars(3);
//         assert_eq!(res, "$1,$2,$3");
//     }
// }