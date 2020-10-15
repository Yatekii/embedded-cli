extern crate proc_macro;
use std::iter::FromIterator;

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
    token::Paren,
    token::Pub,
    Error, Expr, ExprLit, ExprPath, Field, Fields, FieldsUnnamed, Ident, Lit, LitStr,
    PathArguments, PathSegment, Token, Type, TypePath, VisPublic, Visibility,
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

    let e = generate_enum("Command", &cli.paths);

    dbg!(quote! {
        #e


    })
    .into()
}

struct Variant {
    name: Ident,
    values: Vec<syn::Path>,
}

fn generate_enum(name: &str, paths: &Vec<Path>) -> proc_macro2::TokenStream {
    let name = Ident::new(name, Span::call_site());
    let mut variants = vec![];
    let mut subs = vec![];
    for path in paths {
        for segment in &path.segments {
            match segment {
                Segment::Name(name) => variants.push(Variant {
                    name: Ident::new(&name.value().to_camel_case(), name.span()),
                    values: vec![],
                }),
                Segment::Value(value) => {
                    variants.last_mut().map(|v| v.values.push(value.clone()));
                }
                Segment::Sub(paths) => {
                    // TODO: propagate the returns properly
                    variants.last_mut().map(|v| {
                        v.values.push(syn::Path {
                            leading_colon: None,
                            segments: Punctuated::from_iter(vec![PathSegment {
                                ident: v.name.clone(),
                                arguments: PathArguments::None,
                            }]),
                        })
                    });
                    subs.push(generate_enum(
                        &variants.last().unwrap().name.to_string(),
                        paths,
                    ));
                }
            }
        }
    }
    let variants = variants.iter().map(|variant| syn::Variant {
        attrs: vec![],
        ident: variant.name.clone(),
        fields: Fields::Unnamed(FieldsUnnamed {
            paren_token: Paren {
                span: Span::call_site(),
            },
            unnamed: Punctuated::from_iter(variant.values.iter().cloned().map(|path| Field {
                attrs: vec![],
                vis: Visibility::Inherited,
                ident: None,
                colon_token: None,
                ty: Type::Path(TypePath { qself: None, path }),
            })),
        }),
        discriminant: None,
    });
    quote! {
        enum #name {
            #(#variants,)*
        }

    #(#subs)*
    }
}
