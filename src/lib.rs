//! Single_line_macro
//!
//! A procedural macro for writing one-line functions and methods
//! using `=> expr` syntax. Doc comments (`///`) are captured
//! and re-emitted so that `cargo doc` shows them.

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parenthesized;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Paren};
use syn::{parse_macro_input, Attribute, Expr, FnArg, Ident, Result, Token, Type, Visibility};

struct SingleLine {
    attrs: Vec<Attribute>,
    vis: Visibility,
    _fn_token: Option<Token![fn]>,
    name: Ident,
    args: Vec<FnArg>,
    ret_ty: Option<Type>,
    expr: Expr,
}

impl Parse for SingleLine {
    fn parse(input: ParseStream) -> Result<Self> {
        // 1. Capture doc comments and other outer attributes
        let attrs = input.call(Attribute::parse_outer)?;

        // 2. Visibility (`pub`, `pub(crate)`, or nothing)
        let vis: Visibility = input.parse()?;

        // 3. Optional `fn` keyword
        let fn_token = if input.peek(Token![fn]) {
            Some(input.parse()?)
        } else {
            None
        };

        // 4. Function name
        let name: Ident = input.parse()?;

        // 5. Argument list: parse `( ... )` if present
        let mut args = Vec::new();
        if input.peek(Paren) {
            let content;
            parenthesized!(content in input);
            let punct: Punctuated<FnArg, Comma> = content.parse_terminated(FnArg::parse, Comma)?;
            args.extend(punct);
        }

        // 6. Optional return type: parse `-> ...` if present
        let ret_ty = if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            Some(input.parse()?)
        } else {
            None
        };

        // 7. Body expression: parse `=> ...`
        input.parse::<Token![=>]>()?;
        let expr: Expr = input.parse()?;

        Ok(SingleLine {
            attrs,
            vis,
            _fn_token: fn_token,
            name,
            args,
            ret_ty,
            expr,
        })
    }
}

#[proc_macro]
pub fn single_line(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as SingleLine);

    let attrs = input.attrs;
    let vis = input.vis;
    let name = input.name;
    let expr = input.expr;
    let args = input.args;

    let ret_ty = input.ret_ty.unwrap_or_else(|| syn::parse_quote! { () });

    // Detect if this is a method (has &self or &mut self)
    let is_method = args.iter().any(|arg| matches!(arg, FnArg::Receiver(_)));

    // If method and a single bare identifier, rewrite `x` to `self.x`
    let body = if is_method {
        if let Expr::Path(ref path) = expr {
            if path.path.segments.len() == 1 {
                let field = &path.path.segments[0].ident;
                quote! { self.#field }
            } else {
                quote! { #expr }
            }
        } else {
            quote! { #expr }
        }
    } else {
        quote! { #expr }
    };

    // build the function signature
    let args_quote = if !args.is_empty() {
        quote! { ( #(#args),* ) }
    } else {
        quote! { () }
    };

    let expanded = quote! {
        #(#attrs)*
        #vis fn #name #args_quote -> #ret_ty {
            #body
        }
    };
    expanded.into()
}
