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
	NumAssign +
	NumAssignRef +
	NumOps +
	PartialEq +
	PartialOrd +
	Product +
	RefNum<Self> +
	Sized +
	Sum +
	ToOwned
{}
impl Numeric for i8 {}
impl Numeric for i16 {}
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

pub fn merge_sorted<I, T: PartialOrd>(mut iter1: I, mut iter2: I) -> Vec<T>
where I: Iterator<Item = T>
{
	let mut vec = vec![];
	let mut v1 = iter1.next().unwrap();
	let mut v2 = iter2.next().unwrap();

	loop {
		if v1 < v2 { vec.push(v1);
			if let Some(x) = iter1.next() { v1 = x; }
			else {
				vec.extend(iter2);
				return vec;
			}
		}
		else { vec.push(v2);
			if let Some(x) = iter2.next() { v2 = x; }
			else {
				vec.extend(iter1);
				return vec;
			}
		}
	}
}

/// Initialize a hashmap.
/// 
/// # Examples
/// 
/// ```
/// let map = hashmap![("key0", "val0"), ("key1", "val1")];
/// 
/// assert_eq!(Some("val0"), map.get("key0"));
/// assert_eq!(Some("val1"), map.get("key1"));
/// ```
#[macro_export]
macro_rules! hashmap {
	( $([ $key:expr => $val:expr ]),* $(,)? ) => {
		{
			use std::collections::HashMap;
			let mut map = HashMap::new();
			$( map.insert($key, $val); ),*
			map
		}
	}
}

#[macro_export]
macro_rules! btreemap {
	( $([ $key:expr => $val:expr ]),* $(,)? ) => {
		{
			use std::collections::BTreeMap;
			let mut map = BTreeMap::new();
			$( map.insert($key, $val); ),*
			map
		}
	}
}

#[macro_export]
macro_rules! hashset {
	( $( $val:expr ),* $(,)? ) => {
		{
			use std::collections::HashSet;
			let mut set = HashSet::new();
			$( set.insert($val); ),*
			set
		}
	}
}

#[macro_export]
macro_rules! btreeset {
	( $( $val:expr ),* $(,)? ) => {
		{
			use std::collections::BTreeSet;
			let mut set = BTreeSet::new();
			$( set.insert($val); ),*
			set
		}
	}
}

#[macro_export]
macro_rules! sortedvec {
	( $( $val:expr ),* $(,)? ) => {
		{
			use sorted_vec::SortedVec;
			let mut vec = SortedVec::new();
			$( vec.push($val); ),*
			vec
		}
	}
}
