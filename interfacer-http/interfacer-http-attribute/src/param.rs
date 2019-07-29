use proc_macro::Diagnostic;
use proc_macro2::Ident;
use std::collections::HashMap;
use syn::Expr;

const VAL: &'static str = "val";
const HEADER: &'static str = "header";
const BODY: &'static str = "body";

pub struct Parameters {
    pub values: HashMap<String, Ident>,
    pub headers: Vec<(Expr, Ident)>,
    pub body: Option<Ident>,
}

impl Parameters {
    pub fn from_attr() -> Result<Self, Diagnostic> {
        unimplemented!()
    }
}
