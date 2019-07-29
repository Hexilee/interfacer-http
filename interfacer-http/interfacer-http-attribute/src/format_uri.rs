use crate::parse::try_parse;
use lazy_static::lazy_static;
use proc_macro::Diagnostic;
use proc_macro2::{Ident, Span};
use quote::quote;
use regex::Regex;
use syn::{parse_quote, punctuated::Punctuated, Expr, Macro, Token};

const DYN_URI_PATTERN: &str = r#"(?P<pattern>\{\w+})"#;

pub fn gen_uri_format_expr(raw_uri: &str) -> Result<Macro, Diagnostic> {
    lazy_static! {
        static ref URI_REGEX: Regex = Regex::new(DYN_URI_PATTERN).unwrap();
    };
    let mut uri_template = raw_uri.to_owned();
    let mut format_expr = try_parse::<Macro>(quote!(format!()).into())?;
    let mut varables = Vec::new();
    let mut param_list = Punctuated::<Expr, Token![,]>::new();
    for capture in URI_REGEX.captures_iter(raw_uri) {
        let pattern: &str = &capture["pattern"];
        varables.push(
            pattern
                .trim_start_matches('{')
                .trim_end_matches('}')
                .to_owned(),
        );
        uri_template = uri_template.replace(pattern, "{}");
    }
    param_list.push(parse_quote!(#uri_template));
    for variable in varables {
        let ident = Ident::new(&variable, Span::call_site());
        param_list.push(parse_quote!(#ident));
    }
    format_expr.tokens = quote!(#param_list);
    Ok(format_expr)
}

// TODO: complete test
#[cfg(test)]
mod test {
    use super::DYN_URI_PATTERN;
    use regex::Regex;

    fn parse_dyn_uri(raw_uri: &str) {
        for capture in Regex::new(DYN_URI_PATTERN).unwrap().captures_iter(raw_uri) {
            println!("captured {}", &capture["var"]);
        }
    }

    #[test]
    fn parse_dyn_uri_test() {
        parse_dyn_uri("/api/user/{id}?age={age}");
    }
}
