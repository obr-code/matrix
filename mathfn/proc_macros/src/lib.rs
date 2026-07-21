use std::collections::HashMap;
use syn::{
	BinOp,
	Expr,
	ExprLit,
	ExprPath,
	Lit,
	parse_macro_input,
};

struct Parser {
	map: HashMap<String, i32>,
	cnt: usize,
}
impl Parser {
	pub fn parse(&mut self, expr: Expr) -> mathfn::Item {
		match expr {
			Expr::Binary(bin) => {
				match bin.op {
					BinOp::Add(_) => {
						let mut vec = vec![];
						vec.append(&mut self.parse(*bin.left));
						vec.append(&mut self.parse(*bin.right));
						vec
					},
					BinOp::Sub(_) => {
						let mut vec = vec![];
						vec.append(&mut self.parse(*bin.left));
						vec.append(&mut self.parse(*bin.right).into_iter().map(|x| -x).collect());
						vec
					}
					
					BinOp::Mul(_) | BinOp::Div(_) => {
						let mut set = hashset![];
						let mut vec1: Vec<Vec<Expr>> = proc(*bin.left).into_iter().collect();
						let mut vec2: Vec<Vec<Expr>> = proc(*bin.right).into_iter().collect();
						if let BinOp::Div(_) = bin.op {
							for i in 0..vec2.len() {
								for j in 0..vec2[i].len() {
									vec2[i][j] = -vec2[i][j];
								}
							}
						}
						for a in vec1.iter() {
							for b in vec2.iter() {
								set.insert(utils::merge_sorted((*a).iter(), (*b).iter()));
							}
						}
						set
					},
					_ => todo!(),
				}
			},
			Expr::Lit(lit) => mathfn::Item::Int(Self::int(lit)),
			Expr::Path(path) => mathfn::Item::Hash(self.hash(path)),
		}
	}

	fn hash(&mut self, path: ExprPath) -> i32 { // O(N); O(1)
		let s = path.path.segments.into_iter().fold(String::new(), |s, segment|
			s + &segment.ident.to_string()
		);

		if let Some(id) = self.map.get(&s) {
			*id
		} else {
			self.map.insert(s, self.cnt as i32);
			self.cnt += 1;
			self.cnt as i32 - 1
		}
	}

	fn int(lit: ExprLit) -> f64 { // O(1); O(1)
		if let Lit::Int(int) = lit.lit {
			int.base10_parse::<f64>().unwrap()
		} else { panic!() }
	}
}

#[proc_macro]
pub fn mathfn(input: TokenStream) -> TokenStream {
	let Expr::Closure(closure) = parse_macro_input!(input as Expr);
	let body = closure.body;
	
	fn proc(expr: Expr) -> Vec<Var> {
		match expr {
			Expr::Path(path) => vec![Var::Pat(
				path.path.segments.into_iter().fold(String::new(), |s, segment|
					s + &segment.ident.to_string()
				)
			)],
			Expr::Lit(lit) => vec![Var::Int(
				if let Lit::Int(int) = lit.lit {
					int.base10_parse::<f64>().unwrap()
				} else { panic!() }
			)],
			Expr::Binary(bin) => {
				
			},
			_ => todo!(),
		}
	}

	println!("{:?}", proc(*body));
}

#[proc_macro]
pub fn vartable(input: TokenStream) -> TokenStream