use num_traits::{ Num, NumAssign, NumAssignRef, NumCast, NumOps, NumRef, RefNum };
use std::iter::{ Product, Sum };
use std::ops::Neg;

pub trait Numeric: 
	Clone +
	Copy +
	Neg<Output = Self> +
	Num +
	NumCast +
	NumRef +
	Sized +
	Sum +
	NumAssign +
	NumAssignRef +
	PartialEq +
	PartialOrd +
	Product +
	NumOps +
	RefNum<Self>
{}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for isize {}
impl Numeric for f32 {}
impl Numeric for f64 {}

/// Compute the lowest common multiple between a and b.
pub fn lcm<T: Numeric>(mut a: T, mut b: T) -> Option<T> {
	if a == T::zero() || b == T::zero() { return None; }
	while a > b {
		if a > b {
			a = a - b;
		} else {
			b = b - a;
		}
	}
	Some(a)
}

/// Compute the lowest common denominator between a and b.
pub fn lcd<T: Numeric>(mut a: T, mut b: T) -> Option<T> {
	if a == T::zero() || b == T::zero() { return None; }
	if a < T::zero() { a = T::zero() - a; }
	if b < T::zero() { b = T::zero() - b; }
	Some((a * b) / lcm(a, b)?)
}