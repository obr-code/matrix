use mathfn::{Expended, MathTree, V64x4};
use macros::math_tree;

pub fn main() {
	let tree: MathTree<u8> = math_tree!(a + b - c * d / e);
	let exp: Expended<V64x4> = Expended::from(tree);

	println!("{:?}", &exp);
}