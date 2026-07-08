use num_traits::{Num, NumCast};
use std::fmt;
use std::cell::{RefCell, Cell};
use std::iter::{Map, RepeatN, repeat_n};
use std::ops::{ Add, Div, Index, IndexMut, Mul, Range, RangeFrom, RangeTo, Sub };

pub trait Numeric: Clone + Copy + Num + NumCast + Sized {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for u32 {}
impl Numeric for u64 {}
impl Numeric for usize {}
impl Numeric for isize {}
impl Numeric for f32 {}
impl Numeric for f64 {}


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Matrix<T: Numeric> (Vec<Vec<T>>);

impl<T> Matrix<T> where T: Numeric {
	// -- SUMMARY -- //


	// -- Utils -- //

	/// Return the number of row in the matrix.
	pub fn m(&self) -> usize {
		self.0.len()
	}

	/// Return the number of columns in the matrix.
	pub fn n(&self) -> usize {
		self.0[0].len()
	}

	/// Return the matrix's dimensions.
	pub fn dimensions(&self) -> (usize, usize) {
		(self.m(), self.n())
	}

	// -- Init -- //

	/// Initialize a new matrix filled with zeros.
	pub fn null(m: usize, n: usize) -> Self { // O(M*N); O(M*N)
		Matrix(
			vec![vec![T::zero(); n]; m]
		)
	}

	/// Initialize a new matrix filled with `val`.
	pub fn fill(m: usize, n: usize, val: T) -> Self { // O(M*N); O(M*N)
		Matrix(
			vec![vec![val; n]; m]
		)
	}

	/// Initialize a new matrix according to a function `f(i, j)`.
	/// 
	/// # Examples
	/// 
	/// ```
	/// use matlib::matrix;
	/// use matlib::collections::Matrix;
	/// let a = Matrix::from_fn(3, 3, |i, j| i + j);
	/// let b = matrix![
	/// 	[0, 1, 2],
	/// 	[1, 2, 3],
	/// 	[2, 3, 4],
	/// ];
	/// assert_eq!(a, b);
	/// ```
	pub fn from_fn<F>(m: usize, n: usize, f: F) -> Self // O(M*N); O(M*N)
	where 
		F: Fn(T, T) -> T
	{
		Matrix(
			(0..m).map(|i|
				(0..n).map(|j|
					f(T::from(i).unwrap(), T::from(j).unwrap())
				).collect()
			).collect()
		)
	}

	/// Initialize a new matrix from a grid of vectors.
	pub fn from_vec(vec: Vec<Vec<T>>) -> Self { // O(M*N); O(1)
		Matrix(
			vec.into_iter().map(|row|
				row.into_iter().map(|val|
					val
				).collect()
			).collect()
		)
	}

	/// Initialize a new matrix from a given scalar.
	pub fn from_scalar(m: usize, n: usize, scalar: T) -> Self { // O(M*N); O(1)	
		Matrix(
			(0..m).map(|i|
				(0..n).map(|j|
					if i == j { scalar } else { T::zero() }
				).collect()
			).collect()
		)
	}

	/// Initialize a new matrix from four equaly-dimensionned dials.
	pub fn from_dials(m11: &Matrix<T>, m12: &Matrix<T>, m21: &Matrix<T>, m22: &Matrix<T>) -> Self {
		let (m, n) = m11.dimensions();
		assert_eq!((m, n), m12.dimensions());
		assert_eq!((m, n), m21.dimensions());
		assert_eq!((m, n), m22.dimensions());

		let mut matrix = Matrix::null(m<<1, n<<1);
		(0..m).for_each(|i| (0..n).for_each(|j| matrix[i][j] = m11[i][j]));
		(0..m).for_each(|i| (0..n).for_each(|j| matrix[i][j+n] = m12[i][j]));
		(0..m).for_each(|i| (0..n).for_each(|j| matrix[i+m][j] = m21[i][j]));
		(0..m).for_each(|i| (0..n).for_each(|j| matrix[i+m][j+n] = m22[i][j]));

		matrix
	}

	/// Create a new matrix by transposing another one.
	pub fn transposed(matrix: &Matrix<T>) -> Self { // O(M*N); O(1)
		Matrix::from_fn(matrix.n(), matrix.m(), |i, j|
			*matrix.get(
				<usize as NumCast>::from(j).unwrap(),
				<usize as NumCast>::from(i).unwrap(),
			).unwrap()
		)
	}

	// -- Sub -- //

	/// Return the inner matrix laying in `range_i` and `range_j`.
	pub fn sub_matrix(&self, range_i: Range<usize>, range_j: Range<usize>) -> Self { // O(M*N); O(M*N)
		Matrix(
			self.0[range_i].iter().map(|row|
				row[range_j.clone()].to_vec()
			).collect()
		)
	}

	/// Return the four dials of the given matrix.
	pub fn dials(&self) -> (Matrix<T>, Matrix<T>, Matrix<T>, Matrix<T>) {
		assert!(self.m() & 1 == 0);
		assert!(self.n() & 1 == 0);

		let m11 = self.sub_matrix(0..self.m()>>1, 0..self.n()>>1);
		let m12 = self.sub_matrix(0..self.m()>>1, self.n()>>1..self.n());
		let m21 = self.sub_matrix(self.m()>>1..self.m(), 0..self.n()>>1);
		let m22 = self.sub_matrix(self.m()>>1..self.m(), self.n()>>1..self.n());

		(m11, m12, m21, m22)
	}

	// -- Mutation -- //

	/// Resize the matrix filling with zeros.
	pub fn resize(&mut self, m: usize, n: usize) { // O(M*N); O(M*N)
		if self.m() <= m {
			self.0.truncate(m);
		} else {
			self.0.append(
				&mut vec![vec![T::zero(); n]; m - self.m()]
			);
		}
		if self.n() < n {
			self.0.iter_mut().for_each(|row|
				row.truncate(n)
			);
		} else {
			let len = n - self.n();
			self.0.iter_mut().for_each(|row|
				row.append(
					&mut vec![T::zero(); len]
				)
			);
		}
	}

	/// Push a row with a matching length to the end of the matrix.
	pub fn push_row(&mut self, row: Vec<T>) { // O(N); O(N)
		assert_eq!(self.n(), row.len());
		self.0.push(row);
	}

	// -- Get -- //

	/// Return a value at given coordonates. 
	/// 
	/// If coordonates are out of bound, return None.
	pub fn get(&self, i: usize, j: usize) -> Option<&T> { // O(1); O(1)
		if i < self.m() && j < self.n() {
			Some(&self[i][j])
		}
		else { None }
	}

	/// Return an iterator over all cells in matrix.
	pub fn iter_all(&self) -> impl Iterator<Item = &T> { // O(1); O(1)
		self.0.iter().flatten()
	}

	/// Return an iterator over row `i`.
	pub fn iter_row(&self, i: usize) -> Option<impl Iterator<Item = &T>> { // O(1); O(1)
		if i < self.m() {
			Some(
				(0..self.n()).map(move |j|
					&self[i][j]
				)
			)
		}
		else { None }
	}

	/// Return an iterator over column `j`.
	pub fn iter_col(&self, j: usize) -> Option<impl Iterator<Item = &T>> { // O(1); O(1)
		if j < self.n() {
			Some(
				(0..self.m()).map(move |i| 
					&self[i][j]
				)
			)
		}
		else { None }
	}

	/// Find the value of the pivot for a given row `i`.
	pub fn pivot(&self, i: usize) -> Option<&T> { // O(N); O(1)
		if let Some(mut row) = self.iter_row(i) {
			row.find(|x| **x != T::zero())
		}
		else { None }
	}

	/// Find the column of the pivot for a given row `i`.
	pub fn column_pivot(&self, i: usize) -> Option<usize> { // O(N); O(1)
		if let Some(mut row) = self.iter_row(i) {
			row.position(|x| *x != T::zero())
		}
		else { None }
	}

	/// Return the diagonal of the matrix.
	pub fn diagonal(&self) -> Option<impl Iterator<Item = &T>> { // O(M); O(1)
		if self.is_square() {
			Some((0..self.m()).map(|i| self.get(i, i).unwrap()))
		}
		else { None }
	}

	/// Return the trace of the matrix.
	pub fn trace(&self) -> Option<T> { // O(M); O(1)
		if let Some(diagonal) = self.diagonal() {
			Some(diagonal.fold(T::zero(), |acc, cell| acc + *cell))
		} else {
			None
		}
	}

	// -- Check -- //

	/// Check if the matrix is null.
	pub fn is_null(&self) -> bool { // O(M*N); O(1)
		self.0.iter().all(|row|
			row.iter().all(|val|
				*val == T::zero()
			)
		)
	}

	/// Check if the matrix is scaled.
	pub fn is_scaled(&self) -> bool { // O(M*N); O(1)
		(0..self.m()-1).all(|i| {
			let j1 = self.column_pivot(i);
			let j2 = self.column_pivot(i+1);
			(j1 == None && j2 == None)
			|| self.column_pivot(i) < self.column_pivot(i+1)
		})
	}

	/// Check if the matrix is reduced.
	pub fn is_reduced(&self) -> bool { // O(M*N); O(1)
		let mut cols = (0..self.m()-1).filter_map(|i| 
			self.column_pivot(i)
		);
		cols.clone().is_sorted_by(|a, b| a < b)
		&& cols.all(|j|
			self.iter_col(j).unwrap().filter(|cell|
				**cell != T::zero()
			).count() == 1
		)
	}

	/// Check if the matrix is upper-triangular.
	pub fn is_upper_triangular(&self) -> bool { // O(M*N); O(1)
		self.is_square()
		&& (1..self.m()).all(|i|
			(0..i).all(|j| 
				self.get(i, j) == Some(&T::zero())
			)
		)
	}

	/// Check if the matrix is lower-triangular.
	pub fn is_lower_triangular(&self) -> bool { // O(M*N); O(1)
		self.is_square()
		&& (0..self.m()).all(|i|
			(i+1..self.n()).all(|j|
				self.get(i, j) == Some(&T::zero())
			)
		)
	}

	/// Check if the matrix is diagonal.
	pub fn is_diagonal(&self) -> bool { // O(M*N); O(1)
		self.is_upper_triangular() && self.is_lower_triangular()
	}

	/// Check if the matrix is scalar.
	pub fn is_scalar(&self) -> bool { // O(M*N); O(1)
		self.is_diagonal()
		&& self.diagonal().unwrap().all(|cell|
			cell == self.get(0, 0).unwrap()
		)
	}

	/// Check if the matrix is identity.
	pub fn is_identity(&self) -> bool { // O(M*N); O(1)
		self.is_scalar() && self.get(0, 0) == Some(&T::one())
	}

	/// Check if the matrix is symetric.
	pub fn is_symetric(&self) -> bool { // O(M*N); O(1)
		self.is_square()
		&& (0..self.m()).all(|i|
			(i+1..self.n()).all(|j|
				self.get(i, j) == self.get(j, i)
			)
		)
	}

	/// Check if the matrix antisymetric.
	pub fn is_antisymetric(&self) -> bool { // O(M*N); O(1)
		self.is_square()
		&& (0..self.m()).all(|i|
			(i..self.n()).all(|j|
				*self.get(i, j).unwrap() + *self.get(j, i).unwrap() == T::zero()
			)
		)
	}

	// -- Matrix Formats -- //

	/// Check if matrix is line.
	pub fn is_line(&self) -> bool { // O(1); O(1)
		self.m() == 1
	}

	/// Check if matrix is column.
	pub fn is_column(&self) -> bool { // O(1); O(1)
		self.n() == 1
	}

	/// Check if matrix is square.
	pub fn is_square(&self) -> bool { // O(1); O(1)
		self.m() == self.n()
	}
}

impl<T> Index<usize> for Matrix<T> where T: Numeric {
	type Output = [T];
	fn index(&self, index: usize) -> &Self::Output { // O(1); O(1)
		&self.0[index]
	}
}
impl<T> IndexMut<usize> for Matrix<T> where T: Numeric {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output { // O(1); O(1)
		&mut self.0[index]
	}
}

impl<T> Add for &Matrix<T> where  T: Numeric {
	type Output = Matrix<T>;
	fn add(self, other: Self) -> Self::Output { // O(M*N); O(M*N)
		assert_eq!(self.m(), other.m());
		assert_eq!(self.n(), other.n());
		Matrix(
			self.0.iter().zip(other.0.iter())
				.map(|(a, b)| 
					a.into_iter().zip(b.into_iter())
						.map(|(a, b)|
							*a + *b
						).collect()
				).collect()
		)
	}
}
impl<T> Sub for &Matrix<T> where  T: Numeric {
	type Output = Matrix<T>;
	fn sub(self, other: Self) -> Self::Output { // O(M*N); O(M*N)
		assert_eq!(self.m(), other.m());
		assert_eq!(self.n(), other.n());
		Matrix(
			self.0.iter().zip(other.0.iter()).map(|(a, b)| 
				a.into_iter().zip(b.into_iter()).map(|(a, b)|
					*a - *b
				).collect()
			).collect()
		)
	}
}

/// Perform matrix product using Strassen's method for larger matrices.
/// 
/// # Examples
/// 
/// ```
/// use matlib::collections::Matrix;
/// use matlib::matrix;
/// let a = matrix![
/// 	[1, 2],
/// 	[3, 4],
/// ];
/// let b = matrix![
/// 	[5, 6],
/// 	[7, 8],
/// ];
/// let c = matrix![
/// 	[19, 22],
/// 	[43, 50],
/// ];
/// let d = &a * &b;
/// assert_eq!(c, d);
/// ```
impl<T> Mul<T> for Matrix<T> where T: Numeric {
	type Output = Self;
	fn mul(self, scalar: T) -> Self::Output { // O(M*N); O(1)
		Matrix(
			self.0.into_iter().map(|row|
				row.into_iter().map(|cell|
					cell * scalar
				).collect()
			).collect()
		)
	}
}
impl<T> Mul for Matrix<T> where T: Numeric {
	type Output = Matrix<T>;
	fn mul(self, other: Self) -> Self::Output { // O(7^log M); O(M^2)
		&self * &other
	}
}
impl<T> Mul for &Matrix<T> where  T: Numeric {
	type Output = Matrix<T>;
	fn mul(self, other: Self) -> Self::Output { // O(7^log M); O(M^2)

		fn naive<T: Numeric>(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T> {
			Matrix::from_fn(a.m(), b.n(), |i, j|
				a.iter_row(<usize as NumCast>::from(i).unwrap()).unwrap().zip(b.iter_col(<usize as NumCast>::from(j).unwrap()).unwrap()).fold(T::zero(), |acc, (a, b)|
					acc + *a * *b
				)
			)
		}

		fn strassen<T: Numeric>(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T> {
			fn process<T: Numeric>(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T> {
				let (a11, a12, a21, a22) = a.dials();
				let (b11, b12, b21, b22) = b.dials();

				let m1 = (&a11 + &a22) * (&b11 + &b22);
				let m2 = &(&a21 + &a22) * &b11;
				let m3 = &a11 * &(&b12 - &b22);
				let m4 = &a22 * &(&b21 - &b11);
				let m5 = &(&a11 + &a12) * &b22;
				let m6 = (&a21 - &a11) * (&b11 + &b12);
				let m7 = (&a12 - &a22) * (&b21 + &b22);

				let c11 = &(&m1 + &m4) - &(&m5 - &m7);
				let c12 = &m3 + &m5;
				let c21 = &m2 + &m4;
				let c22 = &(&m1 - &m2) + &(&m3 + &m6);
				
				Matrix::from_dials(&c11, &c12, &c21, &c22)
			}
			// `len` is the smallest even length to resize `a` and `b` into square matrices with same dimensions.
			let len = (a.m().max(a.n()).max(b.n()) + 1) & (!1);
			let mut a_copy: Matrix<T> = a.clone();
			let mut b_copy: Matrix<T> = b.clone();
			a_copy.resize(len, len);
			b_copy.resize(len, len);

			process(&a_copy, &b_copy)
		}

		if self.n() * self.m() < 50_000 || (
			self.m() * self.n() * (self.m() + self.n())
			<
			7_usize.pow(self.m().max(self.n()).max(other.n()).ilog2())
		) {
			naive(self, other)
		} else {
			strassen(self, other)
		}
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