use num_traits::{Num, NumCast};
use std::fmt;
use std::ops::Range;
use std::iter::Map;
use std::ops::{ Add, Sub, Mul, Div };

#[derive(PartialEq, Eq)]
pub struct Matrix<T: Num + NumCast  + Clone + Copy> {
	grid: Vec<T>,
	m: usize, n: usize,
}

impl<T> Matrix<T> where T: Num + NumCast + Clone + Copy {
	// Init
	pub fn new<F>(m: usize, n: usize, f: F) -> Self // O(M*N); O(M*N)
	where F: Fn(T, T) -> T
	{
		let mut grid = vec![T::zero(); m*n];
		for i in 0..m {
			for j in 0..n {
				grid[j + i * n] = f(T::from(i+1).unwrap(), T::from(j+1).unwrap());
			}
		}
		Self { grid, m, n }
	}
	pub fn from(vec: Vec<Vec<T>>) -> Self { // O(M*N); O(1)
		let (m, n) = (vec.len(), vec[0].len());
		assert!(vec.iter().all(|row| row.len() == n));
		Self {
			grid: vec.into_iter().flatten().collect(),
			m, n
		}
	}
	// Get
	pub fn get(&self, i: usize, j: usize) -> Option<T> { // O(1); O(1)
		if i < self.m && j < self.n {
			Some(self.grid[j + i * self.n])
		}
		else { None }
	}
	pub fn get_row(&self, i: usize) -> Option<Map<Range<usize>, impl FnMut(usize) -> T>> { // O(N); O(1)
		if i < self.m {
			Some((0..self.n).map(move |j| self.grid[j + i * self.n]))
		}
		else { None }
	}
	pub fn get_col(&self, j: usize) -> Option<Map<Range<usize>, impl FnMut(usize) -> T>> { // O(M); O(1)
		if j < self.n {
			Some((0..self.m).map(move |i| self.grid[j + i * self.n]))
		}
		else { None }
	}
	// Pivot
	pub fn pivot(&self, i: usize) -> Option<T> { // O(N); O(1)
		if let Some(mut row) = self.get_row(i) {
			row.find(|x| *x != T::zero())
		}
		else { None }
	}
	pub fn column_pivot(&self, i: usize) -> Option<usize> { // O(N); O(1)
		if let Some(mut row) = self.get_row(i) {
			row.position(|x| x != T::zero())
		}
		else { None }
	}
	// Matrix Type
	pub fn is_null(&self) -> bool { // O(M*N); O(1)
		self.grid.iter().all(|cell| cell == &T::zero())
	}
	pub fn is_scaled(&self) -> bool { // O(M*N); O(1)
		(1..self.m).all(|i|
			self.column_pivot(i) > self.column_pivot(i-1)
		)
	}
	pub fn is_reduced(&self) -> bool { // O(M*N); O(1)
		(1..self.m).all(|i|
			self.column_pivot(i) > self.column_pivot(i-1)
			&& self.pivot(i)   == Some(T::one())
			&& self.pivot(i-1) == Some(T::one())
		)
	}
	pub fn is_upper_triangular(&self) -> bool { // O(M*N); O(1)
		self.is_square()
		&& (1..self.m).all(|i|
			(0..i).all(|j| 
				self.get(i, j) == Some(T::zero())
			)
		)
	}
	pub fn is_lower_triangular(&self) -> bool { // O(M*N); O(1)
		self.is_square()
		&& (0..self.m).all(|i|
			(i+1..self.n).all(|j|
				self.get(i, j) == Some(T::zero())
			)
		)
	}
	pub fn is_diagonal(&self) -> bool { // O(M*N); O(1)
		self.is_upper_triangular() && self.is_lower_triangular()
	}
	pub fn is_scalar(&self) -> bool { // O(M*N); O(1)
		if self.is_diagonal() {
			self.diagonal().unwrap().all(|cell| cell == self.get(0, 0).unwrap())
		}
		else { false }
	}
	pub fn is_identity(&self) -> bool { // O(M*N); O(1)
		self.is_scalar() && self.get(0, 0).unwrap_or(T::one()) == T::one()
	}
	pub fn is_symetric(&self) -> bool { // O(M*N); O(1)
		self.is_square()
		&& (0..self.m).all(|i|
			(i+1..self.n).all(|j|
				self.get(i, j) == self.get(j, i)
			)
		)
	}
	pub fn is_antisymetric(&self) -> bool { // O(M*N); O(1)
		self.is_square()
		&& (0..self.m).all(|i|
			(i..self.n).all(|j|
				self.get(i, j).unwrap() + self.get(j, i).unwrap() == T::zero()
			)
		)
	}
	// Matrix Formats
	pub fn is_line(&self) -> bool { // O(1); O(1)
		self.m == 1
	}
	pub fn is_column(&self) -> bool { // O(1); O(1)
		self.n == 1
	}
	pub fn is_square(&self) -> bool { // O(1); O(1)
		self.m == self.n
	}
	// Diagonal
	pub fn diagonal(&self) -> Option<Map<Range<usize>, impl FnMut(usize) -> T>> { // O(M); O(1)
		if self.is_square() {
			Some((0..self.m).map(|i| self.get(i, i).unwrap()))
		}
		else { None }
	}
	pub fn trace(&self) -> Option<T> { // O(M); O(1)
		if let Some(diagonal) = self.diagonal() {
			Some(diagonal.fold(T::zero(), |acc, cell| acc + cell))
		} else {
			None
		}
	}
}

#[macro_export]
macro_rules! matrix {
	( $([ $( $elem:expr ),* ]),* $(,)? ) => {
		{
			let mut grid: Vec<Vec<_>> = vec![
				$( vec![ $( $elem ),* ] ),*
			];
			let (m, n) = (grid.len(), grid[0].len());
			assert!(grid.iter().all(|row| row.len() == n));
			Matrix::from(grid)
		}
	}
}
pub(crate) use matrix;

impl<T> fmt::Debug for Matrix<T> where T: Num + NumCast + Clone + Copy + fmt::Debug {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for i in 0..self.m {
			f.write_str(&format!("{:?}\n", self.get_row(i).unwrap().collect::<Vec<T>>()))?;
		}
		f.write_str("")
	}
}

// impl<T> Add for Matrix<T> where  T: Num + Clone + Copy {
// 	type Output = Self;

// 	fn add(self, other: Self) -> Self::Output {
// 		assert_eq!(self.m(), other.m());
// 		assert_eq!(self.n(), other.n());
// 		Self {

// 		}
// 	}
// }