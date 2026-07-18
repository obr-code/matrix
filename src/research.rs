use crate::collections::Matrix;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::_pdep_u32;


pub struct Permutator3x3(u128);
impl Permutator3x3 {
	fn recursive_permutation(&mut self, depth: u8, iterator: impl Iterator<Item = u32>) -> impl Iterator<Item = u32> {
		pub const AREA: u32 = 9;
		if depth == 0 { return iterator }
		
		iterator.map(|x|
			Self::optimal_computation()
		)
	}

	pub unsafe fn optimal_computation(&mut self) {
		let mut permutations = Self::permutations();

		for a in permutations {
			
		}
	}

	#[target_feature(enable = "bmi2")]
	unsafe fn permutations() -> impl Iterator<Item = u32> {
		(1..1 << 9).flat_map(|a|
			(a..1 << 9).map(move |b|
				b * _pdep_u32(a, u32::MAX / ((1 << 9) - 1))
			)
		)
	}
}
