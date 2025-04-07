use fnv::FnvHashMap;
use proc_macro2::Span;
use quote::quote;
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
}

/// e.g.
///
/// DEFAULT: "#483802"
/// 100: "102383"
#[derive(Deserialize)]
pub struct TailwindColor(FnvHashMap<String, String>);
// {
//     #[serde(rename = "DEFAULT")]
//     default: String,
//     #[serde(rename = "100")]
//     l100: String,
//     #[serde(rename = "200")]
//     l200: String,
//     #[serde(rename = "300")]
//     l300: String,
//     #[serde(rename = "400")]
//     l400: String,
//     #[serde(rename = "500")]
//     l500: String,
//     #[serde(rename = "600")]
//     l600: String,
//     #[serde(rename = "700")]
//     l700: String,
//     #[serde(rename = "800")]
//     l800: String,
//     #[serde(rename = "900")]
//     l900: String,
// }

impl TailwindColor {
    pub fn to_token_colors<'a>(&'a self, color_name: &'a str) -> Vec<TokenColor<'a>> {
        let mut colors = Vec::with_capacity(self.0.len());
        for (lighting, hex) in self.0.iter() {
            let mut ident = Ident::new("Srgba", Span::call_site());
            let err_initializer = quote! { (); };
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
                            let comment = comment(&format!("Original hex: {}", hex));

                            let initializer = quote! {
                                Srgba::rgb(#r, #g, #b)
                            };

                            (initializer, comment)
                        }
                        (r, g, b) => {
                            ident = Ident::new("()", Span::call_site());
                            let comment = comment(&format!(
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
                            let comment = comment(&format!("Original hex: {}", hex));
                            let initializer = quote! {
                                Srgba::rgba(#r, #g, #b, #a)
                            };

                            (initializer, comment)
                        }
                        (r, g, b, a) => {
                            //todo
                            ident = Ident::new("()", Span::call_site());
                            let comment = comment(&format!(
                                "Error parsing hex {}\nr: {:?}\ng: {:?}\nb: {:?}\na: {:?}",
                                hex, r, g, b, a
                            ));
                            (err_initializer.clone(), comment)
                        }
                    }
                }
                _ => {
                    let comment = comment(&format!("Invalid hex length for this color: {}", hex));
                    let initializer = quote! { (); };
                    (initializer, comment)
                }
            };
            let token_color =
                TokenColor::new(color_name, lighting.as_str(), ident, comment, initializer);
            colors.push(token_color)
        }
        colors
    }
}

fn comment(doc: &str) -> Attribute {
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
