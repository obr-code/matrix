use sorted_vec::SortedVec;
use std::collections::HashMap;
use utils::btreemap;


/// A product of unknown values represented by 64 bits.
/// 
/// The first leading 32-bits is the coefficient of the entire table as f32.
/// 
/// The others are 4-bits exponants of 24 variables across the overall product as i8.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct MulTable_24x4(u128);
impl Default for MulTable_16x4 {
	fn default() -> Self {
		MulTable_28x4(0x_8888_8888_8888_8888_8888_8888 + (1f32 as u32 as u128))
	}
}

/// Merge 2 MulTable together (performing a multiplication).
impl std::ops::Add for MulTable_16x4 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		let (a, b) = (self.0 as f16, rhs.0 as f16);

		let Some(coef) = a.checked_mul(b)
		else {
			panic!("!!! coefficient-overflow !!!");
		};

		let (a, b) = (self.0 & !0xFF, rhs.0 & !0xFF);

		if a >= 0x888888_00 {
			MulTable_16x4((a - 0x888888_00) + b + (coef as u8 as u64))
		} else if b >= 0x888888_00 {
			MulTable_16x4((b - 0x888888_00) + a + (coef as u8 as u64))
		} else {
			MulTable_16x4((a + b) - 0x888888_00 + (coef as u8 as u64))
		}
	}
}

/// Merge 2 MulTable together (performing a division).
impl std::ops::Sub for MulTable_16x4 {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self {
		let (a, b) = (self.0 as i8, rhs.0 as i8);

		let Some(coef) = a.checked_mul(b)
		else {
			panic!("!!! coefficient-overflow !!!");
		};

		if self.0 < 0x888888_00 {
			MulTable_16x4((self.0 + 0x888888_00) - rhs.0)
		} else if rhs.0 < ADD {
			MulTable_16x4((rhs.0 + 0x888888_00) - self.0)
		} else if self.0 > rhs.0 {
			MulTable_16x4((self.0 - rhs.0) + 0x888888_00)
		} else {
			MulTable_16x4((rhs.0 - self.0) + 0x888888_00)
		}
	}
}


/// Represent a sum of multiplicative expressions and coefficients in the most expended fashion.
#[allow(non_camel_case_types)]
pub struct Expended_16x4(SortedVec<(MulTable_16x4, i8)>);

impl std::ops::Neg for Expended_16x4 {
	type Output = Self;

	fn neg(self) -> Self::Output {
		
	}
}

impl std::ops::Add for Expended_16x4 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Expended_16x4(
			unsafe {
				SortedVec::from_sorted(
					{
						let mut iter1 = self.0.into_iter();
						let mut iter2 = rhs.0.into_iter();
						let mut opt1 = iter1.next();
						let mut opt2 = iter2.next();

						let mut vec = vec![];
						while let (Some(val1), Some(val2)) = (opt1, opt2) {
							if val1 < val2 {
								vec.push(val1);
								opt1 = iter1.next();
							} else {
								vec.push(val2);
								opt2 = iter2.next();
							}
							while val1.0 == vec.last().unwrap().0 {
								vec.last_mut().unwrap().1 += val1.1;
								opt1 = iter1.next();
							}
							while val2.0 == vec.last().unwrap().0 {
								vec.last_mut().unwrap().1 += val2.1;
								opt2 = iter2.next();
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
impl std::ops::Sub for Expended_16x4 {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Expended_16x4(
			unsafe {
				SortedVec::from_sorted(
					{
						let mut iter1 = self.0.into_iter();
						let mut iter2 = rhs.0.into_iter().map(|t| (t.0, -t.1));
						let mut opt1 = iter1.next();
						let mut opt2 = iter2.next();

						let mut vec = vec![];
						while let (Some(val1), Some(val2)) = (opt1, opt2) {
							if val1 < val2 {
								vec.push(val1);
								opt1 = iter1.next();
							} else {
								vec.push((val2.0, val2.1));
								opt2 = iter2.next();
							}
							while val1.0 == vec.last().unwrap().0 {
								vec.last_mut().unwrap().1 += val1.1;
								opt1 = iter1.next();
							}
							while val2.0 == vec.last().unwrap().0 {
								vec.last_mut().unwrap().1 += val2.1;
								opt2 = iter2.next();
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
impl std::ops::Mul for Expended_16x4 {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
			
	}
}