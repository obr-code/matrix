use num_traits::Unsigned;
use serde::{Serialize, Deserialize};
use sorted_vec::SortedVec;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{ Add, BitAnd, Div, Mul, Neg, Shl, Shr, Sub };
use utils::sortedvec;


/// # # Description
/// The `SerialFormat` is a 'way' of storing data in bytes.
/// 
/// For example, a `I32x4`` is a 32-bit value where each 4-bits store a independent i4 quantity.
/// 
/// In other words, it is used to represent multiple data in one single instance.
/// 
/// # # Warning
/// Unconventionnal signed representation.
/// Instead of bit-inversion, it uses leveraging.
pub trait SerialFormat: Copy + Debug + Default + Ord {
	type Support: Copy + Ord + Unsigned + TryFrom<u128> + Into<u128>;

	/// Return the SF.
	/// 
	/// # # Example
	/// ```
	/// let sf = I32x4::sf();
	/// assert_eq!((32, 4), sf);
	/// ```
	fn sf() -> (u8, u8);

	/// Return the bits in which all independend quantities are set to zeros.
	/// 
	/// For example, a `I32x4`` stores 32 bits of 4-bits independent i4 quantities.
	/// 
	/// # # Warning
	/// Unconventionnal signed representation.
	/// Instead of bit-inversion, it uses leveraging.
	/// To get the 'true' value in X-bits, substract it by 2^(X-1).
	fn zeros() -> Self::Support;
}


/// 8 signed 4-bits on 32 bits.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd,)]
pub struct I32x4 {}

/// 16 signed 4-bits on 64 bits.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd,)]
pub struct I64x4 {}

/// 32 signed 4-bits on 128 bits.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd,)]
pub struct I128x4 {}

/// 16 signed 8-bits on 128 bits.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd,)]
pub struct I128x8 {}


impl SerialFormat for I32x4 {
	type Support = u32;
	fn sf() -> (u8, u8) { (32,4) }
	fn zeros() -> Self::Support { 0x_8888_8888 }
}
impl SerialFormat for I64x4 {
	type Support = u64;
	fn sf() -> (u8, u8) { (64,4) }
	fn zeros() -> Self::Support { 0x_8888_8888_8888_8888 }
}
impl SerialFormat for I128x4 {
	type Support = u128;
	fn sf() -> (u8, u8) { (128,4) }
	fn zeros() -> Self::Support { 0x_8888_8888_8888_8888_8888_8888_8888_8888 }
}
impl SerialFormat for I128x8 {
	type Support = u128;
	fn sf() -> (u8, u8) { (128,8) }
	fn zeros() -> Self::Support { 0x_8080_8080_8080_8080_8080_8080_8080_8080 }
}


/// A product of unknown values represented with a `SerialFormat`.
/// Each independent value in the 'SF's support' represents a power.
/// 
/// For example, given the abstract 'pows' I32x4, let's represent it { pows = \[-1, 3, 5, ..., -6\], vars = \[a, b, c, ..., h\] }
/// 
/// The value of the I32x4 is the product:
/// ```
/// a.pow(-1) * b.pow(3) * c.pow(5) ... * h.pow(-6)
/// ```
/// 
/// To represent a I32x4 equals to (a * b.pow(2) / c):
/// ```
/// a.pow(1) * b.pow(2) * c.pow(-1) ... * h.pow(0)
/// ```
/// We can represent in a abstracted I32x4 format:
/// \[1, 2, -1, 0, 0, 0, 0, 0\]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct MulTable<SF: SerialFormat>(SF::Support);

impl<SF: SerialFormat> Default for MulTable<SF> {
	fn default() -> Self {
		MulTable(SF::zeros())
	}
}

impl<SF: SerialFormat> Mul for MulTable<SF> {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		if self.0 >= SF::zeros() {
			MulTable((self.0 - SF::zeros()) + rhs.0)
		} else if rhs.0 >= SF::zeros() {
			MulTable((rhs.0 - SF::zeros()) + self.0)
		} else {
			MulTable((self.0 + rhs.0) - SF::zeros())
		}
	}
}

impl<SF: SerialFormat> Div for MulTable<SF> {
	type Output = Self;

	fn div(self, rhs: Self) -> Self {
		if self.0 < SF::zeros() {
			MulTable((self.0 + SF::zeros()) - rhs.0)
		} else if rhs.0 < SF::zeros() {
			MulTable((rhs.0 + SF::zeros()) - self.0)
		} else if self.0 > rhs.0 {
			MulTable((self.0 - rhs.0) + SF::zeros())
		} else {
			MulTable((rhs.0 - self.0) + SF::zeros())
		}
	}
}

impl<SF: SerialFormat> Debug for MulTable<SF> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let (len, step) = SF::sf();
		let bitmask = u8::MAX as u32;
		let inner = self.0.into();
		
		write!(f, "[")?;
		for shift in (0..len).step_by(step as usize) {
			write!(f, "{:?}", ((inner >> shift) as u32) & bitmask)?;
			if shift != (len - step) {
				write!(f, ", ")?;
			}
		}
		write!(f, "]")
	}
}

/// I think it's pretty obvious what it represents.
#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub enum Sign { Pos, Neg }

/// Wraps a value and represents its sign (+/-).
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Signed<T: Copy> {
	Pos(T),
	Neg(T),
}
impl<T: Copy> Signed<T> {
	/// Return the `Sign`
	pub fn sign(&self) -> Sign {
		match self {
			Signed::Pos(_) => Sign::Pos,
			Signed::Neg(_) => Sign::Neg,
		}
	}
	/// Return the inner value
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

impl<SF: SerialFormat> Mul for Signed<MulTable<SF>> {
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

impl<SF: SerialFormat> Div for Signed<MulTable<SF>> {
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
/// 
/// # # Examples
/// todo!()
#[allow(non_camel_case_types)]
#[derive(Debug, Default)]
pub struct Expended<SF: SerialFormat>(SortedVec<Signed<MulTable<SF>>>);

impl<SF: SerialFormat> Add for Expended<SF> {
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

impl<SF: SerialFormat> Sub for Expended<SF> {
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

impl<SF: SerialFormat> Mul for Expended<SF> {
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

impl<SF: SerialFormat> Div for Expended<SF> {
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
pub enum MathTree {
	BinOp(BinOp),
	Index(usize),
}

#[derive(Serialize, Deserialize)]
pub enum BinOp {
	Add { l: Box<MathTree>, r: Box<MathTree> },
	Sub { l: Box<MathTree>, r: Box<MathTree> },
	Mul { l: Box<MathTree>, r: Box<MathTree> },
	Div { l: Box<MathTree>, r: Box<MathTree> },
}

impl<SF: SerialFormat> From<MathTree> for Expended<SF> {
	fn from(value: MathTree) -> Self {
		match value {
			MathTree::Index(idx) => Expended(
				sortedvec![
					Signed::Pos(
						MulTable(
							if let Ok(inner) = SF::Support::try_from(1u128 << (idx << 2)) {
								SF::zeros() + inner
							} else {
								panic!()
							}
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