use serde::{Serialize, Deserialize};
use sorted_vec::SortedVec;
use std::hash::Hash;
use std::ops::{ Add, Div, Mul, Neg, Shl, Sub };
use utils::sortedvec;


/// Organizations of bits for multiple formats.
pub trait Arch: Copy + std::fmt::Debug + Default + Ord {
	type Int;
	fn zero() -> Self::Int;
}
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd,)]
pub struct V8x4 {}
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd,)]
pub struct V16x4 {}
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd,)]
pub struct V32x4 {}
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd,)]
pub struct V16x8 {}
impl Arch for V8x4 {
	type Int = u32;
	fn zero() -> Self::Int { 0x_8888_8888 }
}
impl Arch for V16x4 {
	type Int = u64;
	fn zero() -> Self::Int { 0x_8888_8888_8888_8888 }
}
impl Arch for V32x4 {
	type Int = u128;
	fn zero() -> Self::Int { 0x_8888_8888_8888_8888_8888_8888_8888_8888 }
}
impl Arch for V16x8 {
	type Int = u128;
	fn zero() -> Self::Int { 0x_8080_8080_8080_8080_8080_8080_8080_8080 }
}

/// A product of unknown values represented by 64 bits.
/// 
/// 16 4-bits exponants where each 4-bits is signed.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MulTable<A: Arch>(A::Int)
where
	<A as Arch>::Int:
		Ord +
		PartialOrd +
		Add<Output = <A as Arch>::Int> +
		Sub<Output = <A as Arch>::Int>;

impl<A: Arch> Default for MulTable<A>
where
	<A as Arch>::Int:
		Ord +
		PartialOrd +
		Add<Output = <A as Arch>::Int> +
		Sub<Output = <A as Arch>::Int>
{
	fn default() -> Self {
		MulTable(A::zero())
	}
}

/// Merges MulTables (performing a multiplication).
impl<A: Arch> Mul for MulTable<A>
where 
	<A as Arch>::Int:
		Copy +
		Ord +
		PartialOrd +
		Add<Output = <A as Arch>::Int> +
		Sub<Output = <A as Arch>::Int>
{
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		if self.0 >= A::zero() {
			MulTable((self.0 - A::zero()) + rhs.0)
		} else if rhs.0 >= A::zero() {
			MulTable((rhs.0 - A::zero()) + self.0)
		} else {
			MulTable((self.0 + rhs.0) - A::zero())
		}
	}
}

/// Merge 2 MulTables together (performing a division).
impl<A: Arch> Div for MulTable<A>
where 
	<A as Arch>::Int:
		Copy +
		Ord +
		PartialOrd +
		Add<Output = <A as Arch>::Int> +
		Sub<Output = <A as Arch>::Int>
{
	type Output = Self;

	fn div(self, rhs: Self) -> Self {
		if self.0 < A::zero() {
			MulTable((self.0 + A::zero()) - rhs.0)
		} else if rhs.0 < A::zero() {
			MulTable((rhs.0 + A::zero()) - self.0)
		} else if self.0 > rhs.0 {
			MulTable((self.0 - rhs.0) + A::zero())
		} else {
			MulTable((rhs.0 - self.0) + A::zero())
		}
	}
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub enum Sign { Pos, Neg }

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Signed<T: Copy> {
	Pos(T),
	Neg(T),
}
impl<T: Copy> Signed<T> {
	pub fn sign(&self) -> Sign {
		match self {
			Signed::Pos(_) => Sign::Pos,
			Signed::Neg(_) => Sign::Neg,
		}
	}
	pub fn get(&self) -> T {
		match *self {
			Signed::Pos(x) => x,
			Signed::Neg(x) => x,
		}
	}
}

impl<T: Copy> Neg for Signed<T> {
	type Output = Self;
	fn neg(self) -> Self::Output {
		match self {
			Signed::Pos(x) => Signed::Neg(x),
			Signed::Neg(x) => Signed::Pos(x),
		}
	}
}

impl<A: Arch> Mul for Signed<MulTable<A>>
where
	<A as Arch>::Int:
		Copy +
		Ord +
		PartialOrd +
		Add<Output = <A as Arch>::Int> +
		Sub<Output = <A as Arch>::Int>
{
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output {
		if self.sign() == rhs.sign() {
			if self.sign() == Sign::Pos {
				Signed::Pos(self.get() * rhs.get())
			} else {
				Signed::Neg(self.get() * rhs.get())
			}
		} else {
			if self.sign() == Sign::Pos {
				Signed::Neg(self.get() * rhs.get())
			} else {
				Signed::Pos(self.get() * rhs.get())
			}
		}
	}
}

impl<A: Arch> Div for Signed<MulTable<A>>
where
	<A as Arch>::Int:
		Copy +
		Ord +
		PartialOrd +
		Add<Output = <A as Arch>::Int> +
		Sub<Output = <A as Arch>::Int>
{
	type Output = Self;
	fn div(self, rhs: Self) -> Self::Output {
		if self.sign() == rhs.sign() {
			if self.sign() == Sign::Pos {
				Signed::Pos(self.get() / rhs.get())
			} else {
				Signed::Neg(self.get() / rhs.get())
			}
		} else {
			if self.sign() == Sign::Pos {
				Signed::Neg(self.get() / rhs.get())
			} else {
				Signed::Pos(self.get() / rhs.get())
			}
		}
	}
}

/// Represent a sum of multiplicative expressions and coefficients in the most expended fashion.
#[allow(non_camel_case_types)]
#[derive(Debug, Default)]
pub struct Expended<A: Arch>(SortedVec<Signed<MulTable<A>>>)
where
	Signed<MulTable<A>>: Ord,
	<A as Arch>::Int:
		Copy +
		Ord +
		PartialOrd +
		Add<Output = <A as Arch>::Int> +
		Sub<Output = <A as Arch>::Int> +
		std::fmt::Debug;

impl<A: Arch> Add for Expended<A>
where
	Signed<MulTable<A>>: Ord,
	<A as Arch>::Int:
		Copy +
		Ord +
		PartialOrd +
		Add<Output = <A as Arch>::Int> +
		Sub<Output = <A as Arch>::Int> +
		std::fmt::Debug
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Expended(
			unsafe {
				SortedVec::from_sorted(
					{
						let mut iter1 = self.0.into_iter().peekable();
						let mut iter2 = rhs.0.into_iter().peekable();

						let mut vec = vec![];
						while let (Some(a), Some(b)) = (iter1.peek(), iter2.peek()) {
							if a < b {
								vec.push(iter1.next().unwrap());
							} else {
								vec.push(iter2.next().unwrap());
							}
						}
						vec.extend(iter1);
						vec.extend(iter2);
						vec
					}
				)
			}
		)
	}
}

impl<A: Arch> Sub for Expended<A>
where
	Signed<MulTable<A>>: Ord,
	<A as Arch>::Int:
		Copy +
		Ord +
		PartialOrd +
		Add<Output = <A as Arch>::Int> +
		Sub<Output = <A as Arch>::Int> +
		std::fmt::Debug
{
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Expended(
			unsafe {
				SortedVec::from_sorted(
					{
						let mut iter1 = self.0.into_iter().peekable();
						let mut iter2 = rhs.0.into_iter().peekable();

						let mut vec = vec![];
						while let (Some(a), Some(b)) = (iter1.peek(), iter2.peek()) {
							if a < b {
								vec.push(iter1.next().unwrap());
							} else {
								vec.push(-iter2.next().unwrap());
							}
						}
						vec.extend(iter1);
						vec.extend(iter2);
						vec
					}
				)
			}
		)
	}
}

impl<A: Arch> Mul for Expended<A>
where
	Expended<A>: Default,
	Signed<MulTable<A>>: Ord,
	<A as Arch>::Int:
		Copy +
		Ord +
		PartialOrd +
		Add<Output = <A as Arch>::Int> +
		Sub<Output = <A as Arch>::Int> +
		std::fmt::Debug
{
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		let mut c = Self::default();
		for a in self.0.iter() {
			for b in rhs.0.iter() {
				c.0.push(*a * *b);
			}
		}
		c
	}
}

impl<A: Arch> Div for Expended<A>
where
	Expended<A>: Default,
	Signed<MulTable<A>>: Ord,
	<A as Arch>::Int:
		Copy +
		Ord +
		PartialOrd +
		Add<Output = <A as Arch>::Int> +
		Sub<Output = <A as Arch>::Int> +
		std::fmt::Debug
{
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		let mut c = Self::default();
		for a in self.0.iter() {
			for b in rhs.0.iter() {
				c.0.push(*a / *b);
			}
		}
		c
	}
}

#[derive(Serialize, Deserialize)]
pub enum MathTree<T: Hash> {
	BinOp(BinOp<T>),
	Value(T),
}

#[derive(Serialize, Deserialize)]
pub enum BinOp<T: Hash> {
	Add { l: Box<MathTree<T>>, r: Box<MathTree<T>> },
	Sub { l: Box<MathTree<T>>, r: Box<MathTree<T>> },
	Mul { l: Box<MathTree<T>>, r: Box<MathTree<T>> },
	Div { l: Box<MathTree<T>>, r: Box<MathTree<T>> },
}

impl<A: Arch, T: Hash + Shl<u8>> From<MathTree<T>> for Expended<A>
where
	Expended<A>: Default,
	Signed<MulTable<A>>: Ord,
	T: Into<u64>,
	<A as Arch>::Int:
		Copy +
		From<u64> +
		Ord +
		PartialOrd +
		Add<Output = <A as Arch>::Int> +
		Sub<Output = <A as Arch>::Int> +
		std::fmt::Debug
{
	fn from(value: MathTree<T>) -> Self {
		match value {
			MathTree::Value(val) => Expended(
				sortedvec![
					Signed::Pos(
						MulTable(
							A::zero() + <A as Arch>::Int::from(1u64 << (val.into() << 2))
						)
					)
				]
			),
			MathTree::BinOp(bin) => {
				match bin {
					BinOp::Add { l, r } => Expended::from(*l) + Expended::from(*r),
					BinOp::Sub { l, r } => Expended::from(*l) - Expended::from(*r),
					BinOp::Mul { l, r } => Expended::from(*l) * Expended::from(*r),
					BinOp::Div { l, r } => Expended::from(*l) / Expended::from(*r),
				}
			}
		}
	}
}