//! Single_line_macro
//!
//! A procedural macro for writing one-line functions and methods
//! using `=> expr` syntax. Doc comments (`///`) are captured
//! and re-emitted so that `cargo doc` shows them.

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Paren};
use syn::{Attribute, Expr, Ident, Result, Token, Type, Visibility, parse_macro_input};
use syn::{FnArg, parenthesized};

/// Parses a single `single_line![…]` invocation.
struct SingleLine {
    attrs: Vec<Attribute>,
    vis: Visibility,
    #[allow(unused)]
    fn_token: Option<Token![fn]>,
    name: Ident,
    args: Vec<FnArg>,
    ret_ty: Type,
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

        // 6. Return arrow and type
        input.parse::<Token![->]>()?;
        let ret_ty: Type = input.parse()?;

        // 7. `=>` and the expression
        input.parse::<Token![=>]>()?;
        let expr: Expr = input.parse()?;

        Ok(SingleLine {
            attrs,
            vis,
            fn_token,
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
    let ret_ty = input.ret_ty;
    let expr = input.expr;
    let args = input.args;

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

    // Reconstruct argument list or default to ()
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
