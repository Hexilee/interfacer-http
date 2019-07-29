use proc_macro::Diagnostic;
use proc_macro2::Ident;
use std::collections::HashMap;
use syn::{Expr, FnArg};

const VAL: &str = "val";
const HEADER: &str = "header";
const BODY: &str = "body";

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
