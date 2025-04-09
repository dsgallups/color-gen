use std::sync::LazyLock;

use anyhow::Result;
use fnv::FnvHashMap;
use proc_macro2::Span;
use quote::quote;
use regex::Regex;
use serde::Deserialize;
use syn::{Attribute, Expr, ExprLit, Ident, Lit, LitStr, Meta, MetaNameValue, Path};

use crate::ir::TokenColor;

#[derive(Deserialize)]
pub struct TailwindMap(FnvHashMap<String, TailwindColor>);

impl TailwindMap {
    pub fn to_token_colors(&self) -> Vec<TokenColor> {
        let mut token_colors = Vec::new();
        for (color_name, tailwind_color) in self.0.iter() {
            token_colors.extend(tailwind_color.to_token_colors(color_name));
        }

        token_colors
    }
    // we're going to have to manually parse an input since we may
    // be accepting a js object, not json.
    pub fn from_json(json: &str) -> Result<Self> {
        static RE_SINGLE_QUOTED: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"'([#A-Za-z0-9_\-]+)'").unwrap());
        static RE_UNQUOTED_KEY: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r#"(?P<key>[A-Za-z0-9_\-]+)\s*:"#).unwrap());

        let json = RE_SINGLE_QUOTED.replace_all(json, r#""$1""#);
        let json = RE_UNQUOTED_KEY.replace_all(&json, r#""$key":"#);

        let result = serde_json::from_str(&json)?;
        Ok(result)
    }
}

#[derive(Deserialize)]
pub struct TailwindColor(FnvHashMap<String, String>);

impl TailwindColor {
    pub fn to_token_colors<'a>(&'a self, color_name: &'a str) -> Vec<TokenColor<'a>> {
        let mut colors = Vec::with_capacity(self.0.len());
        for (lighting, hex) in self.0.iter() {
            let mut ident = quote! {Color};
            let err_initializer = quote! {()};
            let hex_trimmed = hex.trim_start_matches('#');
            let (initializer, comment) = match hex_trimmed.len() {
                6 => {
                    match (
                        u8::from_str_radix(&hex_trimmed[0..2], 16),
                        u8::from_str_radix(&hex_trimmed[2..4], 16),
                        u8::from_str_radix(&hex_trimmed[4..6], 16),
                    ) {
                        (Ok(r), Ok(g), Ok(b)) => {
                            let r = r as f32 / 255.0;
                            let g = g as f32 / 255.0;
                            let b = b as f32 / 255.0;
                            let comment = gen_comment(&format!("Original hex: {}", hex));

                            let initializer = quote! {
                                Color::srgb(#r, #g, #b)
                            };

                            (initializer, comment)
                        }
                        (r, g, b) => {
                            ident = quote! {()};
                            let comment = gen_comment(&format!(
                                "Error parsing hex {}\nr: {:?}\ng: {:?}\nb: {:?}",
                                hex, r, g, b
                            ));
                            (err_initializer.clone(), comment)
                        }
                    }
                }
                8 => {
                    match (
                        u8::from_str_radix(&hex_trimmed[0..2], 16),
                        u8::from_str_radix(&hex_trimmed[2..4], 16),
                        u8::from_str_radix(&hex_trimmed[4..6], 16),
                        u8::from_str_radix(&hex_trimmed[6..8], 16),
                    ) {
                        (Ok(r), Ok(g), Ok(b), Ok(a)) => {
                            let r = r as f32 / 255.0;
                            let g = g as f32 / 255.0;
                            let b = b as f32 / 255.0;
                            let a = a as f32 / 255.0;
                            let comment = gen_comment(&format!("Original hex: {}", hex));
                            let initializer = quote! {
                                Color::srgba(#r, #g, #b, #a)
                            };

                            (initializer, comment)
                        }
                        (r, g, b, a) => {
                            //todo
                            ident = quote! {()};
                            let comment = gen_comment(&format!(
                                "Error parsing hex {}\nr: {:?}\ng: {:?}\nb: {:?}\na: {:?}",
                                hex, r, g, b, a
                            ));
                            (err_initializer.clone(), comment)
                        }
                    }
                }
                _ => {
                    ident = quote! {()};
                    let comment =
                        gen_comment(&format!("Invalid hex length for this color: {}", hex));
                    (err_initializer.clone(), comment)
                }
            };
            let token_color =
                TokenColor::new(color_name, lighting.as_str(), ident, comment, initializer);
            colors.push(token_color)
        }
        colors
    }
}

fn gen_comment(doc: &str) -> Attribute {
    Attribute {
        pound_token: Default::default(),
        style: syn::AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: Meta::NameValue(MetaNameValue {
            path: Path::from(Ident::new("doc", Span::call_site())),
            eq_token: Default::default(),
            value: Expr::Lit(ExprLit {
                attrs: vec![],
                lit: Lit::Str(LitStr::new(doc, Span::call_site())),
            }),
        }),
    }
}
