use num_traits::{Num, NumCast};
use std::fmt;
use std::iter::Map;
use std::ops::{ Add, Sub, Mul, Div, Range, Index, IndexMut };

trait Numeric: Clone + Copy + Num + NumCast {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for u32 {}
impl Numeric for u64 {}
impl Numeric for usize {}
impl Numeric for isize {}
impl Numeric for f32 {}
impl Numeric for f64 {}

#[derive(PartialEq, Eq)]
pub struct Matrix<T: Numeric> {
	grid: Vec<T>,
	m: usize, n: usize,
}

impl<T> Matrix<T> where T: Numeric {
	// Init
	pub fn null(m: usize, n: usize) -> Self { // O(M*N); O(M*N)
		Matrix {
			grid: vec![T::zero(); m*n],
			m, n
		}
	}
	pub fn fill(val: T, m: usize, n: usize) -> Self { // O(M*N); O(M*N)
		Matrix {
			grid: vec![val; m*n],
			m, n
		}
	}
	pub fn from_fn<F>(m: usize, n: usize, f: F) -> Self // O(M*N); O(M*N)
	where F: Fn(T, T) -> T
	{
		let mut matrix = Self::null(m, n);
		for i in 0..m {
			for j in 0..n {
				matrix[i][j] = f(T::from(i+1).unwrap(), T::from(j+1).unwrap());
			}
		}
		matrix
	}
	pub fn from_vec(vec: Vec<Vec<T>>) -> Self { // O(M*N); O(1)
		let (m, n) = (vec.len(), vec[0].len());
		assert!(vec.iter().all(|row| row.len() == n));
		Self {
			grid: vec.into_iter().flatten().collect(),
			m, n
		}
	}
	pub fn from_scalar(m: usize, n: usize, scalar: T) -> Self { // O(M*N); O(1)
		Matrix::from_fn(m, n, |i, j|
			if i == j {
				scalar
			} else {
				T::zero()
			}
		)
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
	// Transposition
	pub fn transposed(&self) -> Self { // O(M*N); O(1)
		Matrix::from_fn(self.n, self.m, |i, j|
			self.get(
				<usize as NumCast>::from(j).unwrap(),
				<usize as NumCast>::from(i).unwrap(),
			).unwrap()
		)
	}
}

impl<T> Index<usize> for Matrix<T> where T: Numeric {
	type Output = [T];

	fn index(&self, index: usize) -> &Self::Output { // O(1); O(1)
		let start = index * self.n;
		let end = start + self.n;
		&self.grid[start..end]
	}
}
impl<T> IndexMut<usize> for Matrix<T> where T: Numeric {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output { // O(1); O(1)
		let start = index * self.n;
		let end = start + self.n;
		&mut self.grid[start..end]
	}
}

impl<T> fmt::Debug for Matrix<T> where T: Numeric + fmt::Debug {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for i in 0..self.m {
			f.write_str(&format!("{:?}\n", self.get_row(i).unwrap().collect::<Vec<T>>()))?;
		}
		f.write_str("")
	}
}

impl<T> Add for Matrix<T> where  T: Numeric {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output { // O(M*N); O(1)
		assert_eq!(self.m, other.m);
		assert_eq!(self.n, other.n);
		Matrix { 
			grid: self.grid.into_iter().zip(other.grid.into_iter()).map(|(a, b)| a + b).collect(),
			m: self.m, n: self.n
		}
	}
}
impl<T> Sub for Matrix<T> where  T: Numeric {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output { // O(M*N); O(1)
		assert_eq!(self.m, other.m);
		assert_eq!(self.n, other.n);
		Matrix { 
			grid: self.grid.into_iter().zip(other.grid.into_iter()).map(|(a, b)| a - b).collect(),
			m: self.m, n: self.n
		}
	}
}
impl<T> Mul<T> for Matrix<T> where T: Numeric {
	type Output = Self;

	fn mul(self, scalar: T) -> Self::Output { // O(M*N); O(1)
		Matrix { 
			grid: self.grid.into_iter().map(|cell| cell * scalar).collect(),
			m: self.m, n: self.n
		}
	}
}
impl<T> Mul for Matrix<T> where  T: Numeric {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output { // O((M*N)^1.5); O(M*N)
		assert_eq!(self.n, other.m);

		fn naive<T: Numeric>(a: Matrix<T>, b: Matrix<T>) -> Matrix<T>
		where T: Num + NumCast + Clone + Copy {
			let (m, n) = (a.m, b.n);
			
			Matrix::from_fn(m, n, |i, j| {
				let a = a.get_row(<usize as NumCast>::from(i).unwrap()).unwrap();
				let b = b.get_row(<usize as NumCast>::from(j).unwrap()).unwrap();
				let c = a.zip(b);
				c.fold(T::zero(), |acc, (a, b)| acc + a * b)
			})
		}

		naive(self, other)
	}
}



#[macro_export]
macro_rules! matrix { // O(M*N); O(M*N)
	( $([ $( $elem:expr ),* ]),* $(,)? ) => {
		{
			let mut grid: Vec<Vec<_>> = vec![
				$( vec![ $( $elem ),* ] ),*
			];
			let (m, n) = (grid.len(), grid[0].len());
			assert!(grid.iter().all(|row| row.len() == n));
			Matrix::from_vec(grid)
		}
	};
	( $lit:literal; $m:expr; $n:expr ) => {
		Matrix::fill($lit, $m, $n)
	}
}
pub(crate) use matrix;