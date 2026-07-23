use serde::{Serialize, Deserialize};
use sorted_vec::SortedVec;
use std::fmt::Debug;
use std::ops::*;
use utils::sortedvec;
use ux::u4;
use ethnum::{uint, u256};

use private;
mod private {
	use crate::{Signed, SWAR, I32x4, I64x4, I128x8, I256x8};
	use std::fmt::Debug;
	use std::ops::*;
	use ux::u4;
	use ethnum::{uint, u256};

	pub trait Block<U: SWAR>:
		Clone +
		Copy +
		Debug +
		Ord +
		Sized +
		Add<Output=Self> +
		Not<Output=Self> +
		Sub<Output=Self> +
		Shl<u32, Output=Self> +
		Shr<u32, Output=Self> +
		BitAnd<Output=Self> +
		BitAndAssign +
		BitOrAssign
	{
		const BITS: u32;
		const MAX: Self;
		/// The bits in which all independend quantities are set to zeros.
		/// 
		/// For example, a `I32x4`` stores 32 bits of 4-bits independent i4 quantities.
		/// 
		/// # # Warning
		/// Unconventionnal signed representation.
		/// Instead of bit-inversion, it uses leveraging.
		/// To get the 'true' value in X-bits, substract it by 2^(X-1).
		const ZERO: Self;
		fn as_chunk(&self) -> U::Chunk;
	}
	
	impl Block<I32x4> for u32 {
		const BITS: u32 = 32;
		const MAX: Self = u32::MAX;
		const ZERO: Self = 0x_8888_8888;
		fn as_chunk(&self) -> <I32x4 as SWAR>::Chunk {
			u4::new(*self as u8)
		}
	}
	impl Block<I64x4> for u64 {
		const BITS: u32 = 64;
		const MAX: Self = u64::MAX;
		const ZERO: Self = 0x_8888_8888_8888_8888;
		fn as_chunk(&self) -> <I64x4 as SWAR>::Chunk {
			u4::new(*self as u8)
		}
	}
	impl Block<I128x8> for u128 {
		const BITS: u32 = 128;
		const MAX: Self = u128::MAX;
		const ZERO: Self = 0x_8080_8080_8080_8080_8080_8080_8080_8080;
		fn as_chunk(&self) -> <I128x8 as SWAR>::Chunk {
			*self as u8
		}
	}
	impl Block<I256x8> for u256 {
		const BITS: u32 = 256;
		const MAX: Self = u256::MAX;
		const ZERO: Self = uint!("0x_8080_8080_8080_8080_8080_8080_8080_8080_8080_8080_8080_8080_8080_8080_8080_8080");
		fn as_chunk(&self) -> <I256x8 as SWAR>::Chunk {
			self.as_u8()
		}
	}

	pub trait Chunk<U: SWAR>:
		Clone +
		Copy +
		Debug +
		Ord +
		Sized +
		Add<Output=Self> +
		Not<Output=Self> +
		Sub<Output=Self> +
		Shl<u32, Output=Self> +
		Shr<u32, Output=Self> +
		BitAnd<Output=Self> +
		BitAndAssign +
		BitOrAssign
	{
		const BITS: u32;
		const MAX: Self;
		const ZERO: Self;
		fn as_block(&self) -> U::Block;
		fn signed(&self) -> Signed<Self>;
	}

	impl Chunk<I32x4> for u4 {
		const BITS: u32 = 4;
		const MAX: Self = u4::new(0b1111);
		const ZERO: Self = u4::new(0b1000);
		fn as_block(&self) -> <I32x4 as SWAR>::Block {
			u32::from(*self)
		}
		fn signed(&self) -> Signed<Self> {
			if *self < <Self as Chunk<I32x4>>::ZERO {
				Signed::Neg(
					<Self as Chunk<I32x4>>::ZERO - *self
				)
			} else {
				Signed::Pos(
					*self - <Self as Chunk<I32x4>>::ZERO
				)
			}
		}
	}
	impl Chunk<I64x4> for u4 {
		const BITS: u32 = 4;
		const MAX: Self = u4::new(0b1111);
		const ZERO: Self = u4::new(0b1000);
		fn as_block(&self) -> <I64x4 as SWAR>::Block {
			u64::from(*self)
		}
	}
	impl Chunk<I128x8> for u8 {
		const BITS: u32 = 8;
		const MAX: Self = 0b1111_1111;
		const ZERO: Self = 0b1000_0000;
		fn as_block(&self) -> <I128x8 as SWAR>::Block {
			u128::from(*self)
		}
	}
	impl Chunk<I256x8> for u8 {
		const BITS: u32 = 8;
		const MAX: Self = 0b1111_1111;
		const ZERO: Self = 0b1000_0000;
		fn as_block(&self) -> <I256x8 as SWAR>::Block {
			u256::from(*self)
		}
	}
}

/// SIMD: 'Single Instruction, Multiple Data'
/// 
/// SWAR: 'SIMD Within A Register'
/// 
/// # # Description
/// A 'way' of storing multiple data in a single register in order to perform SIMD.
/// 
/// For example, a `I32x4`` is a 32-bit value where each 4-bits store a independent i4 quantity.
/// 
/// In other words, it is used to represent multiple data in one single instance.
/// 
/// # # Warning
/// Unconventionnal signed representation.
/// Instead of bit-inversion, it uses leveraging.
pub trait SWAR: Copy + Debug + Default + Ord {
	/// The unsigned type holding the bits of all independent variables.
	type Block: private::Block<Self>;
	/// One single independant variables
	type Chunk: private::Chunk<Self>;

	fn get(block: Self::Block, index: u32) -> Self::Chunk {
		use private::{Block, Chunk};
		Self::Chunk::MAX & (block >> (index * Self::Chunk::BITS)).as_chunk()
	}
	fn set(block: &mut Self::Block, index: u32, value: Self::Chunk) {
		use private::{Block, Chunk};
		let bitshift = index * Self::Chunk::BITS;
		let bitmask = Self::Chunk::MAX.as_block() << bitshift;
		*block &= !bitmask;
		*block |= value.as_block() << bitshift;
	}
}

/// 8 signed 4-bits on 32 bits.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd,)]
pub struct I32x4 {}

/// 16 signed 4-bits on 64 bits.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd,)]
pub struct I64x4 {}

/// 16 signed 8-bits on 128 bits.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd,)]
pub struct I128x8 {}

/// 32 signed 8-bits on 256 bits.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd,)]
pub struct I256x8 {}

impl SWAR for I32x4 {
	type Block = u32;
	type Chunk = u4;
}
impl SWAR for I64x4 {
	type Block = u64;
	type Chunk = u4;
}
impl SWAR for I128x8 {
	type Block = u128;
	type Chunk = u8;
}
impl SWAR for I256x8 {
	type Block = u256;
	type Chunk = u8;
}


/// A product of unknown values represented within a register.
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
pub struct Register<U: SWAR>(U::Block);

/// Transform a SWAR-Register to a vector containing its chunks.
impl<U: SWAR> From<Register<U>> for Vec<i32> {
	fn from(value: Register<U>) -> Self {
		use private::{Block, Chunk};
		let mut result = vec![];
		for idx in 0..U::Block::BITS / U::Chunk::BITS {
			result.push(
				{
					let inner = U::get(value)
				}
				U::get(value.0, idx) as i32
				-
				U::Chunk::ZERO
			);
		}
		result
	}
}

impl<U: SWAR> Mul for Register<U> {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		if self.0 >= U::LEVERAGE {
			Register((self.0 - U::LEVERAGE) + rhs.0)
		} else if rhs.0 >= U::LEVERAGE {
			Register((rhs.0 - U::LEVERAGE) + self.0)
		} else {
			Register((self.0 + rhs.0) - U::LEVERAGE)
		}
	}
}

impl<U: SWAR> Div for Register<U> {
	type Output = Self;

	fn div(self, rhs: Self) -> Self {
		if self.0 < U::LEVERAGE {
			Register((self.0 + U::LEVERAGE) - rhs.0)
		} else if rhs.0 < U::LEVERAGE {
			Register((rhs.0 + U::LEVERAGE) - self.0)
		} else if self.0 > rhs.0 {
			Register((self.0 - rhs.0) + U::LEVERAGE)
		} else {
			Register((rhs.0 - self.0) + U::LEVERAGE)
		}
	}
}

impl<U: SWAR> Debug for Register<U> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", Vec::from(*self))
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

impl<U: SWAR> Mul for Signed<Register<U>> {
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

impl<U: SWAR> Div for Signed<Register<U>> {
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
pub struct Expended<U: SWAR>(SortedVec<Signed<Register<U>>>);

impl<U: SWAR> Add for Expended<U> {
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

impl<U: SWAR> Sub for Expended<U> {
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

impl<U: SWAR> Mul for Expended<U> {
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

impl<U: SWAR> Div for Expended<U> {
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


/// Similar to a token tree, but providing only mathematical expressions.
/// 
/// It is used to perform `TokenStream` -> [Expended] conversions.
#[derive(Debug, Serialize, Deserialize)]
pub enum MathTree {
	BinOp(BinOp),
	Index(usize),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BinOp {
	Add { l: Box<MathTree>, r: Box<MathTree> },
	Sub { l: Box<MathTree>, r: Box<MathTree> },
	Mul { l: Box<MathTree>, r: Box<MathTree> },
	Div { l: Box<MathTree>, r: Box<MathTree> },
}

impl<U: SWAR> From<MathTree> for Expended<U> {
	fn from(value: MathTree) -> Self {
		match value {
			MathTree::Index(idx) => Expended(
				sortedvec![
					Signed::Pos(
						Register(
							{
								let mut register = U::LEVERAGE;
								
								register
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