use std::collections::HashMap;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Expr, ExprAssign, parse_macro_input, parse_quote};

#[proc_macro]
pub fn compute(input: TokenStream) -> TokenStream {
	let expr = parse_macro_input!(input as Expr);

	fn wrap_ref(expr: Expr) -> Expr {
		match expr {
			Expr::Path(path) => {
				parse_quote!(&#path)
			},
			Expr::Binary(mut bin) => {
				bin.left = Box::new(wrap_ref(*bin.left));
				bin.right = Box::new(wrap_ref(*bin.right));
				Expr::Binary(bin)
			},
			Expr::Unary(mut unary) => {
				unary.expr = Box::new(wrap_ref(*unary.expr));
				Expr::Unary(unary)
			},
			Expr::Paren(mut paren) => {
				paren.expr = Box::new(wrap_ref(*paren.expr));
				Expr::Paren(paren)
			},
			other => other,
		}
	}

	let expr = wrap_ref(expr);
	quote!(#expr).into()
}