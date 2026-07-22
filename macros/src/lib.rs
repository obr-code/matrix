use mathfn::MathTree;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::{HashMap};
use std::hash::Hash;
use syn::{BinOp, Expr, parse_macro_input};


#[derive(Eq, Hash, PartialEq)]
enum Variable {
	Lit(syn::Lit),
	Path(syn::Path),
}

#[derive(Default)]
struct Parser<T> {
	map: HashMap<Variable, T>,
	cnt: T,
}
impl Parser<u8> {
	fn parse(&mut self, expr: Expr) -> MathTree<u8> {
		match expr {
			Expr::Lit(lit) => {
				let key = Variable::Lit(lit.lit);
				if let Some(&id) = self.map.get(&key) {
					MathTree::Value(id)
				} else {
					self.map.insert(key, self.cnt);
					self.cnt += 1;
					MathTree::Value(self.cnt - 1)
				}
			},
			Expr::Path(path) => {
				let key = Variable::Path(path.path);
				if let Some(&id) = self.map.get(&key) {
					MathTree::Value(id)
				} else {
					self.map.insert(key, self.cnt);
					self.cnt += 1;
					MathTree::Value(self.cnt - 1)
				}
			},
			Expr::Binary(bin) => {
				MathTree::BinOp(
					match bin.op {
						BinOp::Add(_) => mathfn::BinOp::Add {
							l: Box::new(self.parse(*bin.left)), 
							r: Box::new(self.parse(*bin.right)),
						},
						BinOp::Sub(_) => mathfn::BinOp::Sub {
							l: Box::new(self.parse(*bin.left)), 
							r: Box::new(self.parse(*bin.right)),
						},
						BinOp::Mul(_) => mathfn::BinOp::Mul {
							l: Box::new(self.parse(*bin.left)), 
							r: Box::new(self.parse(*bin.right)),
						},
						BinOp::Div(_) => mathfn::BinOp::Div {
							l: Box::new(self.parse(*bin.left)), 
							r: Box::new(self.parse(*bin.right)),
						},
						_ => panic!(),
					}
				)
			}
			_ => panic!(),
		}
	}
}

#[proc_macro]
pub fn math_tree(input: TokenStream) -> TokenStream {
	let expr = parse_macro_input!(input);

	let mut parser = Parser::<u8>::default();
	let math_tree = parser.parse(expr);

	let bytes = bincode::serialize(&math_tree).unwrap();
	let byte_tokens = proc_macro2::Literal::byte_string(&bytes);

	quote! {
		{
			let bytes = #byte_tokens;
			bincode::deserialize::<MathTree<u8>>(bytes).unwrap()
		}
	}.into()
}