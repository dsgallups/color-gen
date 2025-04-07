use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Attribute, Ident};

pub struct TokenColor<'a> {
    color_name: &'a str,
    ///e.g. 100, 200, etc.
    color_lighting: &'a str,
    /// Srgba
    color_type: TokenStream,
    comment: Attribute,
    initializer: TokenStream,
}
impl<'a> TokenColor<'a> {
    pub fn new(
        color_name: &'a str,
        color_lighting: &'a str,
        color_type: TokenStream,
        comment: Attribute,
        initializer: TokenStream,
    ) -> Self {
        Self {
            color_name,
            color_lighting,
            color_type,
            comment,
            initializer,
        }
    }
}

impl ToTokens for TokenColor<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let comment = &self.comment;
        let variable_name = Ident::new(
            &format!(
                "{}_{}",
                self.color_name.to_uppercase(),
                self.color_lighting.to_uppercase()
            ),
            Span::call_site(),
        );
        let ident = &self.color_type;
        let initializer = &self.initializer;
        let output = quote! {
            #comment
            pub const #variable_name: #ident = #initializer;
        };
        tokens.extend(output);
    }
}
