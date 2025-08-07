use std::fmt;
use std::fmt::Display;
use proc_macro2::Ident;
use syn::Path;

#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

pub const DB_STRUCT: Symbol = Symbol("dbstruct");
pub const TABLE_NAME: Symbol = Symbol("table_name");
pub const NO_INSERT: Symbol = Symbol("no_insert");
// pub const RETURN_VALUE: Symbol = Symbol("return_value");
pub const PRIMARY_KEY: Symbol = Symbol("primary_key");
pub const FOREIGN_KEY: Symbol = Symbol("foreign_key");
pub const ASSOCIATED_VALUES: Symbol = Symbol("associated_values");
pub const BACKREF_KEY: Symbol = Symbol("backref_key");

impl PartialEq<Symbol> for Ident {
    fn eq(&self, word: &Symbol) -> bool {
        self == word.0
    }
}

impl PartialEq<Symbol> for &Ident {
    fn eq(&self, word: &Symbol) -> bool {
        *self == word.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl PartialEq<Symbol> for &Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.0)
    }
}