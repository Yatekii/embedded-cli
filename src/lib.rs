extern crate proc_macro;
use heck::CamelCase;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
    punctuated::Punctuated,
    token::Brace,
    Error, Expr, ExprLit, ExprPath, Ident, Lit, LitStr, Token,
};

#[derive(Debug)]
struct Cli {
    paths: Vec<Path>,
}

impl Parse for Cli {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Cli {
            paths: parse_paths(input)?,
        })
    }
}

fn parse_paths(input: ParseStream) -> Result<Vec<Path>> {
    let paths = Punctuated::<Path, Token![,]>::parse_terminated(input)?;
    Ok(paths.into_iter().collect())
}

#[derive(Debug)]
struct Path {
    segments: Vec<Segment>,
}

impl Parse for Path {
    fn parse(input: ParseStream) -> Result<Self> {
        let punctuation = Punctuated::<Segment, Token![>]>::parse_separated_nonempty(input)?;

        Ok(Path {
            segments: punctuation.into_iter().collect(),
        })
    }
}

#[derive(Debug)]
enum Segment {
    Name(LitStr),
    Value(syn::Path),
    Sub(Vec<Path>),
}

impl Parse for Segment {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            Ok(Segment::Name(input.parse::<LitStr>()?))
        } else if input.peek(Ident) {
            Ok(Segment::Value(input.parse::<syn::Path>()?))
        } else if input.peek(Brace) {
            let sub;
            braced!(sub in input);
            Ok(Segment::Sub(parse_paths(&sub)?))
        } else {
            Err(input.error("Expected a valid path segment"))
        }
    }
}

#[proc_macro]
pub fn cli(input: TokenStream) -> TokenStream {
    let cli = parse_macro_input!(input as Cli);

    println!("{:#?}", cli);

    let l = LitStr::new("KEK", Span::call_site());

    let e = generate_enum("Command", &cli.paths);

    dbg!(quote! {
        #e


    })
    .into()
}

fn generate_enum(name: &str, paths: &Vec<Path>) -> proc_macro2::TokenStream {
    let name = Ident::new(name, Span::call_site());
    let mut variants = vec![];
    for path in paths {
        match &path.segments[0] {
            Segment::Name(name) => {
                variants.push(Ident::new(&name.value().to_camel_case(), name.span()))
            }
            _ => (),
        }
    }
    quote! {
        enum #name {
            #(#variants,)*
        }
    }
}
