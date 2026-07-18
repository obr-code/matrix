#![feature(trait_alias)]

use crate::maths;
use std::iter::{FusedIterator, Peekable};
use std::ops::{ Add, Sub, Mul, Div, Range };

pub trait MatSized {
	/// Return the matrix's dimensions.
	fn dimensions(&self) -> (usize, usize);

	/// Return the number of row in the matrix.
	fn m(&self) -> usize;

	/// Return the number of columns in the matrix.
	fn n(&self) -> usize;
}

pub trait Mat<T> {
	/// Initialize a column matrix from a vector.
	fn column(vec: Vec<T>) -> Self;

	/// Initialize a new matrix filled with `val`.
	fn fill(m: usize, n: usize, val: T) -> Self;

	/// Initialize a new matrix from a grid of vectors.
	fn from(vec: Vec<Vec<T>>) -> Self;

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
	fn from_fn<F>(m: usize, n: usize, f: F) -> Self
	where 
		F: Fn(usize, usize) -> T;
		

	/// Initialize a new matrix from four equaly-dimensionned dials.
	fn from_dials(m11: &Matrix<T>, m12: &Matrix<T>, m21: &Matrix<T>, m22: &Matrix<T>) -> Self;

	/// Initialize a identity matrix.
	fn identity(m: usize, n: usize) -> Self;

	/// Initializie the inverse of a matrix
	/// 
	/// If it does not exist, create a copy of the matrix.
	fn inversed(matrix: &Matrix<T>) -> Self;

	/// Initialize a new matrix filled with zeros.
	fn null(m: usize, n: usize) -> Self;

	/// Create a new reduced matrix with Gauss' reduction method from another matrix.
	fn reduced(matrix: &Matrix<T>) -> Self;

	/// Initialize a new matrix by inversing the order of each element in rows.
	/// 
	/// # Examples
	/// 
	/// ```
	/// use matlib::matrix;
	/// use matlib::collections::Matrix;
	/// let a = matrix![[1, 2], [3, 4]];
	/// let b = matrix![[2, 1], [4, 3]];
	/// let c = Matrix::row_reflected(&b);
	/// assert_eq!(a, c);
	/// ```
	fn row_reflected(matrix: &Matrix<T>) -> Self;

	/// Initialize a new matrix from a given scalar.
	fn scalar(m: usize, n: usize, scalar: T) -> Self;

	/// Initialize a new matrix by transposing another one.
	fn transposed(matrix: &Matrix<T>) -> Self;

	// -- Sub -- //

	/// Return the inner matrix laying in `range_i` and `range_j`.
	fn sub_matrix(&self, range_i: Range<usize>, range_j: Range<usize>) -> Self;

	/// Return the four dials of the given matrix.
	fn dials(&self) -> [Matrix<T>; 4];

	// -- Mutation -- //

	/// Resize the matrix filling with zeros.
	fn resize(&mut self, m: usize, n: usize);

	/// Push a row with a matching length to the end of the matrix.
	fn push_row(&mut self, row: Vec<T>);

	/// Push a column with a matching length to the end of the matrix.
	fn push_col(&mut self, col: Vec<T>);

	/// Remove the row at index `i` and return it.
	fn remove_row(&mut self, i: usize) -> Vec<T>;

	/// Remove the column at index `j` and return it.
	fn remove_col(&mut self, j: usize) -> Vec<T>;

	// -- Get -- //

	/// Return a value at given coordonates. 
	/// 
	/// If coordonates are out of bound, return None.
	fn get(&self, i: usize, j: usize) -> Option<T>;

	/// Return an iterator over all cells in matrix.
	fn iter_all(&self) -> impl Iterator<Item = &T>;

	/// Return an iterator over row `i`.
	fn iter_row(&self, i: usize) -> Option<impl Iterator<Item = T>>;

	/// Return an iterator over column `j`.
	fn iter_col(&self, j: usize) -> Option<impl Iterator<Item = T>>;

	/// Maximize a linear optimization matrix.
	/// 
	/// Return variables value.
	fn maximize_linear(&self) -> Vec<T>;

	/// Return the determinant of the matrice using Gauss' reduction method.
	fn determinant(&self) -> Option<T>;

	/// Return the cofactor of the given coordonates.
	fn cofactor(&self, i: usize, j: usize) -> Option<T>;

	/// Return the minor of the given coordonates.
	/// 
	/// The minor of a matrix at coordonate `i, j` is the determinant 
	/// of the matrix without row `i` and column `j`.
	fn minor(&self, i: usize, j: usize) -> Option<T>;

	/// Find the value of the pivot for a given row `i`.
	fn pivot(&self, i: usize) -> Option<T>;

	/// Find the column of the pivot for a given row `i`.
	fn column_pivot(&self, i: usize) -> Option<usize>;

	/// Return the diagonal of the matrix.
	fn diagonal(&self) -> Option<impl Iterator<Item = T>>;

	/// Return the trace of the matrix.
	fn trace(&self) -> Option<T>;

	// -- Check -- //

	/// Check if the matrix is null.
	fn is_null(&self) -> bool;

	/// Check if the matrix is scaled.
	fn is_scaled(&self) -> bool;

	/// Check if the matrix is reduced.
	fn is_reduced(&self) -> bool;

	/// Check if the matrix is upper-triangular.
	fn is_upper_triangular(&self) -> bool;

	/// Check if the matrix is lower-triangular.
	fn is_lower_triangular(&self) -> bool;

	/// Check if the matrix is diagonal.
	fn is_diagonal(&self) -> bool;

	/// Check if the matrix is invertible.
	fn is_invertible(&self) -> bool;

	/// Check if the matrix is scalar.
	fn is_scalar(&self) -> bool;

	/// Check if the matrix is identity.
	fn is_identity(&self) -> bool;

	/// Check if the matrix is symetric.
	fn is_symetric(&self) -> bool;

	/// Check if the matrix is antisymetric.
	fn is_antisymetric(&self) -> bool;

	/// Check if the matrix is a transition matrix.
	fn is_transition(&self) -> bool;

	/// Check if the matrix is a level state matrix.
	fn is_level_state(&self) -> bool;

	/// Check if the matrix is regular.
	fn is_regular(&self) -> bool;

	// -- Matrix Formats -- //

	/// Check if matrix is line.
	fn is_line(&self) -> bool;

	/// Check if matrix is column.
	fn is_column(&self) -> bool;

	/// Check if matrix is square.
	fn is_square(&self) -> bool;

}

pub struct Matrix<f64>(Vec<Vec<f64>>);

impl MatSized for Matrix<f64> {
	fn dimensions(&self) -> (usize, usize) {
		(self.m(), self.n())
	}
	fn m(&self) -> usize {
		self.0.len()
	}
	fn n(&self) -> usize {
		if self.m() == 0 { 0 }
		else { self.0[0].len() }
	}
}
impl Mat<f64> for Matrix<f64> {
	fn column(vec: Vec<f64>) -> Self {
		Matrix::from_fn(vec.len(), 1, |i, _|
			vec[i]
		)
	}

	fn fill(m: usize, n: usize, val: f64) -> Self { // O(M*N); O(M*N)
		Matrix(
			vec![vec![val; n]; m]
		)
	}

	fn from(vec: Vec<Vec<f64>>) -> Self { // O(M*N); O(1)
		Matrix(
			vec.into_iter().map(|row|
				row.into_iter().map(|val|
					val
				).collect()
			).collect()
		)
	}

	fn from_dials(m11: &Matrix<f64>, m12: &Matrix<f64>, m21: &Matrix<f64>, m22: &Matrix<f64>) -> Self {
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

	fn from_fn<F>(m: usize, n: usize, f: F) -> Self // O(M*N); O(M*N)
	where 
		F: Fn(usize, usize) -> f64
	{
		Matrix(
			(0..m).map(|i|
				(0..n).map(|j|
					f(i, j)
				).collect()
			).collect()
		)
	}

	fn identity(m: usize, n: usize) -> Self { // O(M*N); O(1)
		Matrix::scalar(m, n, 1f64)
	}

	fn inversed(matrix: &Matrix<f64>) -> Self { // O(M^3); O(1)
		if let Some(det) = matrix.determinant() {
			&Self::transposed(matrix) * (1f64 / det)
		} else {
			matrix.clone()
		}
	}

	fn null(m: usize, n: usize) -> Self { // O(M*N); O(M*N)
		Matrix(
			vec![vec![0f64; n]; m]
		)
	}

	fn reduced(matrix: &Matrix<f64>) -> Self { // O(MN*(M+N)); O(M*N)
		let mut matrix = matrix.clone();
		let (m, n) = matrix.dimensions();

		(0..m.min(n)).for_each(|p| // pivot
			(p+1..m).for_each(|i| {
				if let Some(lcd) = maths::lcd(matrix[i][p], matrix[p][p]) {
					let k1 = lcd / matrix[i][p];
					let k2 = lcd / matrix[p][p];

					(p..n).for_each(|j| {
						matrix[i][j] *= k1;
						let prod = matrix[p][j] * k2;
						matrix[i][j] -= prod;
					});
				}
			})
		);

		matrix
	}

	fn row_reflected(matrix: &Matrix<f64>) -> Self {
		Matrix::from_fn(matrix.m(), matrix.n(), |i, j|
			matrix.get(i, matrix.n()-1 - j).unwrap()
		)
	}

	fn scalar(m: usize, n: usize, scalar: f64) -> Self { // O(M*N); O(1)	
		Matrix(
			(0..m).map(|i|
				(0..n).map(|j|
					if i == j { scalar } else { 0f64 }
				).collect()
			).collect()
		)
	}

	fn transposed(matrix: &Matrix<f64>) -> Self { // O(M*N); O(1)
		Matrix::from_fn(matrix.n(), matrix.m(), |i, j|
			matrix.get(j, i).unwrap()
		)
	}

	fn sub_matrix(&self, range_i: Range<usize>, range_j: Range<usize>) -> Self { // O(M*N); O(M*N)
		Matrix(
			self.0[range_i].iter().map(|row|
				row[range_j.clone()].to_vec()
			).collect()
		)
	}

	fn dials(&self) -> [Matrix<f64>; 4] {
		assert!(self.m() & 1 == 0);
		assert!(self.n() & 1 == 0);

		let m11 = self.sub_matrix(0..self.m()>>1, 0..self.n()>>1);
		let m12 = self.sub_matrix(0..self.m()>>1, self.n()>>1..self.n());
		let m21 = self.sub_matrix(self.m()>>1..self.m(), 0..self.n()>>1);
		let m22 = self.sub_matrix(self.m()>>1..self.m(), self.n()>>1..self.n());

		[m11, m12, m21, m22]
	}

	// -- Mutation -- //

	fn resize(&mut self, m: usize, n: usize) { // O(M*N); O(M*N)
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

	fn push_row(&mut self, row: Vec<f64>) { // O(N); O(N)
		assert_eq!(self.n(), row.len());
		self.0.push(row);
	}

	fn push_col(&mut self, col: Vec<f64>) { // O(N); O(N)
		assert_eq!(self.m(), col.len());
		(0..self.m()).for_each(|i|
			self.0[i].push(col[i])
		);
	}

	fn remove_row(&mut self, i: usize) -> Vec<f64> {
		self.0.remove(i)
	}

	fn remove_col(&mut self, j: usize) -> Vec<f64> {
		let mut out = vec![];
		for row in self.0.iter_mut() {
			out.push(row.remove(j));
		}
		out
	}

	// -- Get -- //

	fn get(&self, i: usize, j: usize) -> Option<f64> { // O(1); O(1)
		if i < self.m() && j < self.n() {
			Some(self[i][j])
		}
		else { None }
	}

	/// Return an iterator over all cells in matrix.
	fn iter_all(&self) -> impl Iterator<Item = &f64> { // O(1); O(1)
		self.0.iter().flatten()
	}

	/// Return an iterator over row `i`.
	fn iter_row(&self, i: usize) -> Option<impl Iterator<Item = f64>> { // O(1); O(1)
		if i < self.m() {
			Some(
				(0..self.n()).map(move |j|
					self[i][j]
				)
			)
		}
		else { None }
	}

	/// Return an iterator over column `j`.
	fn iter_col(&self, j: usize) -> Option<impl Iterator<Item = f64>> { // O(1); O(1)
		if j < self.n() {
			Some(
				(0..self.m()).map(move |i| 
					self[i][j]
				)
			)
		}
		else { None }
	}

	/// Maximize a linear optimization matrix.
	/// 
	/// Return variables value.
	fn maximize_linear(&self) -> Vec<f64> { // O(M^3); O(M*N)
		let (m, n) = self.dimensions();
		let mut vec = vec![0; m-1];
		let mut matrix = self.clone();

		for j1 in 0..m-1 {
			let iterator = (0..m-1).filter(|i| matrix[*i][j1] != 0f64);
			let i1 = iterator.clone().min_by(|i1, i2|
				(matrix[*i1][n-1] / matrix[*i1][j1]).partial_cmp(&(matrix[*i2][n-1] / matrix[*i2][j1])).unwrap()
			).unwrap();
			vec[j1] = i1;
			
			for i2 in (0..m-1).filter(|i2| *i2 != i1) {
				let lcd = lcd(matrix[i1][j1], matrix[i2][j1]).unwrap();
				let (k1, k2) = (lcd / matrix[i1][j1], lcd / matrix[i2][j1]);

				for j2 in 0..n {
					matrix[i2][j2] *= k2;
					let tmp = matrix[i1][j2] * k1;
					matrix[i2][j2] -= tmp;
				}
			}
		}

		vec.into_iter().enumerate().map(|(j, i)|
			matrix[i][n-1] / matrix[i][j]
		).collect()
	}

	/// Return the determinant of the matrice using Gauss' reduction method.
	pub fn determinant(&self) -> Option<f64> { // O(M^3); O(M*N)
		let mut matrix = self.clone();

		matrix.push_col(vec![T::one(); self.m()]);

		let matrix = Matrix::reduced(&matrix);
		println!("{:?}", &matrix);
		
		Some(
			matrix.diagonal()?.zip(matrix.iter_col(self.n()-1)?).map(|(a, b)|
				a / b
			).product()
		)
	}

	/// Return the cofactor of the given coordonates.
	fn cofactor(&self, i: usize, j: usize) -> Option<f64> { // O(M^3); O(M*N)
		let det = self.minor(i, j)?;
		if (i ^ j) & 1 == 1 {
			Some(-det)
		} else {
			Some(det)
		}
	}

	fn minor(&self, i: usize, j: usize) -> Option<f64> { // O(N*M)
		let mut matrix = self.clone();
		matrix.remove_row(i);
		matrix.remove_col(j);
		matrix.determinant()
	}

	fn pivot(&self, i: usize) -> Option<f64> { // O(N); O(1)
		if let Some(mut row) = self.iter_row(i) {
			row.find(|x| *x != 0f64)
		}
		else { None }
	}

	fn column_pivot(&self, i: usize) -> Option<f64> { // O(N); O(1)
		if let Some(mut row) = self.iter_row(i) {
			row.position(|x| x != 0f64)
		}
		else { None }
	}

	fn diagonal(&self) -> Option<impl Iterator<Item = f64>> { // O(M); O(1)
		if self.is_square() {
			Some((0..self.m()).map(|i| self.get(i, i).unwrap()))
		}
		else { None }
	}

	fn trace(&self) -> Option<f64> { // O(M); O(1)
		if let Some(diagonal) = self.diagonal() {
			Some(diagonal.fold(0f64, |acc, cell| acc + cell))
		} else {
			None
		}
	}

	// -- Check -- //

	fn is_null(&self) -> bool { // O(M*N); O(1)
		self.0.iter().all(|row|
			row.iter().all(|val|
				*val == 0f64
			)
		)
	}

	fn is_scaled(&self) -> bool { // O(M*N); O(1)
		(0..self.m()-1).all(|i| {
			let j1 = self.column_pivot(i);
			let j2 = self.column_pivot(i+1);
			(j1 == None && j2 == None)
			|| self.column_pivot(i) < self.column_pivot(i+1)
		})
	}

	fn is_reduced(&self) -> bool { // O(M*N); O(1)
		let mut cols = (0..self.m()-1).filter_map(|i| 
			self.column_pivot(i)
		);
		cols.clone().is_sorted_by(|a, b| a < b)
		&& cols.all(|j|
			self.iter_col(j).unwrap().filter(|cell|
				*cell != 0f64
			).count() == 1
		)
	}

	fn is_upper_triangular(&self) -> bool { // O(M*N); O(1)
		self.is_square()
		&& (1..self.m()).all(|i|
			(0..i).all(|j| 
				self.get(i, j) == Some(0f64)
			)
		)
	}

	/// Check if the matrix is lower-triangular.
	fn is_lower_triangular(&self) -> bool { // O(M*N); O(1)
		self.is_square()
		&& (0..self.m()).all(|i|
			(i+1..self.n()).all(|j|
				self.get(i, j) == Some(0f64)
			)
		)
	}

	/// Check if the matrix is diagonal.
	fn is_diagonal(&self) -> bool { // O(M*N); O(1)
		self.is_upper_triangular() && self.is_lower_triangular()
	}

	/// Check if the matrix is invertible.
	fn is_invertible(&self) -> bool {
		let det = self.determinant();
		det.is_some() && det != Some(0f64)
	}

	/// Check if the matrix is scalar.
	fn is_scalar(&self) -> bool { // O(M*N); O(1)
		self.is_diagonal()
		&& self.diagonal().unwrap().all(|cell|
			cell == self.get(0, 0).unwrap()
		)
	}

	/// Check if the matrix is identity.
	fn is_identity(&self) -> bool { // O(M*N); O(1)
		self.get(0, 0) == Some(1f64) && self.is_scalar()
	}

	/// Check if the matrix is symetric.
	fn is_symetric(&self) -> bool { // O(M*N); O(1)
		self.is_square()
		&& (0..self.m()).all(|i|
			(i+1..self.n()).all(|j|
				self.get(i, j) == self.get(j, i)
			)
		)
	}

	/// Check if the matrix is antisymetric.
	fn is_antisymetric(&self) -> bool { // O(M*N); O(1)
		self.is_square()
		&& (0..self.m()).all(|i|
			(i..self.n()).all(|j|
				self.get(i, j).unwrap() + self.get(j, i).unwrap() == 0f64
			)
		)
	}

	/// Check if the matrix is a transition matrix.
	fn is_transition(&self) -> bool { // O(M*N); O(1)
		self.is_square()
		&& (0..self.n()).all(|j|
			(0..self.m()).fold(0f64, |acc, i|
				acc + self.get(i, j).unwrap()
			) == 1f64
		)
	}

	/// Check if the matrix is a level state matrix.
	fn is_level_state(&self) -> bool { // O(M); O(1)
		self.is_column()
		&& (0..self.m()).fold(0f64, |acc, i|
			acc + self.get(i, 0).unwrap()
		) == 1f64
	}

	/// Check if the matrix is regular.
	fn is_regular(&self) -> bool {
		self.determinant() != Some(0f64)
	}

	// -- Matrix Formats -- //

	/// Check if matrix is line.
	fn is_line(&self) -> bool { // O(1); O(1)
		self.m() == 1
	}

	/// Check if matrix is column.
	fn is_column(&self) -> bool { // O(1); O(1)
		self.n() == 1
	}

	/// Check if matrix is square.
	fn is_square(&self) -> bool { // O(1); O(1)
		self.m() == self.n()
	}
}

pub struct MatrixFn(Box<dyn FnMut(usize, usize) -> f64>);

impl Mat<f64> for MatrixFn {
	fn column(vec: Vec<f64>) -> Self { // O(1); O(1)
		MatrixFn(|i, j|
			if j != 0 {
				None
			} else {
				vec.get(i)
			}
		)
	}
	fn fill(m: usize, n: usize, val: f64) -> Self { // O(1); O(1)
		MatrixFn(|i, j|
			if i < m && j < n {
				Some(val)
			} else {
				None
			}
		)
	}
	
	fn from(vec: Vec<Vec<f64>>) -> Self { // O(1); O(1)
		MatrixFn(|i, j|
			vec.get(i)?.get(j)
		)
	}
	
	fn from_fn<F>(m: usize, n: usize, f: F) -> Self // O(F); O(F)
	where 
		F: Fn(usize, usize) -> f64
	{
		MatrixFn(f)
	}
	
	fn from_dials(m11: &Matrix<f64>, m12: &Matrix<f64>, m21: &Matrix<f64>, m22: &Matrix<f64>) -> Self {
		MatrixFn(|i, j|
			if m11.m() < i {
				if m11.n() < j {

				} else {

				}
			} else {
				if m1
			}
		)
	}
	
	fn identity(m: usize, n: usize) -> Self { // O(1); O(1)
		MatrixFn(|i, j|
			if m == n {
				1f64
			} else {
				0f64
			}
		)
	}
	
	fn inversed(matrix: &Matrix<f64>) -> Self {
					todo!()
			}
	
	fn null(m: usize, n: usize) -> Self {
					todo!()
			}
	
	fn reduced(matrix: &Matrix<f64>) -> Self {
					todo!()
			}
	
	fn row_reflected(matrix: &Matrix<f64>) -> Self {
					todo!()
			}
	
	fn scalar(m: usize, n: usize, scalar: f64) -> Self {
					todo!()
			}
	
	fn transposed(matrix: &Matrix<f64>) -> Self {
					todo!()
			}
	
	fn sub_matrix(&self, range_i: Range<usize>, range_j: Range<usize>) -> Self {
					todo!()
			}
	
	fn dials(&self) -> [Matrix<f64>; 4] {
					todo!()
			}
	
	fn resize(&mut self, m: usize, n: usize) {
					todo!()
			}
	
	fn push_row(&mut self, row: Vec<f64>) {
					todo!()
			}
	
	fn push_col(&mut self, col: Vec<f64>) {
					todo!()
			}
	
	fn remove_row(&mut self, i: usize) -> Vec<f64> {
					todo!()
			}
	
	fn remove_col(&mut self, j: usize) -> Vec<f64> {
					todo!()
			}
	
	fn get(&self, i: usize, j: usize) -> Option<f64> {
					todo!()
			}
	
	fn iter_all(&self) -> impl Iterator<Item = &f64> {
					todo!()
			}
	
	fn iter_row(&self, i: usize) -> Option<impl Iterator<Item = f64>> {
					todo!()
			}
	
	fn iter_col(&self, j: usize) -> Option<impl Iterator<Item = f64>> {
					todo!()
			}
	
	fn maximize_linear(&self) -> Vec<f64> {
					todo!()
			}
	
	fn determinant(&self) -> Option<f64> {
					todo!()
			}
	
	fn cofactor(&self, i: usize, j: usize) -> Option<f64> {
					todo!()
			}
	
	fn minor(&self, i: usize, j: usize) -> Option<f64> {
					todo!()
			}
	
	fn pivot(&self, i: usize) -> Option<f64> {
					todo!()
			}
	
	fn column_pivot(&self, i: usize) -> Option<usize> {
					todo!()
			}
	
	fn diagonal(&self) -> Option<impl Iterator<Item = f64>> {
					todo!()
			}
	
	fn trace(&self) -> Option<f64> {
					todo!()
			}
	
	fn is_null(&self) -> bool {
					todo!()
			}
	
	fn is_scaled(&self) -> bool {
					todo!()
			}
	
	fn is_reduced(&self) -> bool {
					todo!()
			}
	
	fn is_upper_triangular(&self) -> bool {
					todo!()
			}
	
	fn is_lower_triangular(&self) -> bool {
					todo!()
			}
	
	fn is_diagonal(&self) -> bool {
					todo!()
			}
	
	fn is_invertible(&self) -> bool {
					todo!()
			}
	
	fn is_scalar(&self) -> bool {
					todo!()
			}
	
	fn is_identity(&self) -> bool {
					todo!()
			}
	
	fn is_symetric(&self) -> bool {
					todo!()
			}
	
	fn is_antisymetric(&self) -> bool {
					todo!()
			}
	
	fn is_transition(&self) -> bool {
					todo!()
			}
	
	fn is_level_state(&self) -> bool {
					todo!()
			}
	
	fn is_regular(&self) -> bool {
					todo!()
			}
	
	fn is_line(&self) -> bool {
					todo!()
			}
	
	fn is_column(&self) -> bool {
					todo!()
			}
	
	fn is_square(&self) -> bool {
					todo!()
			}
}
pub struct MatrixSuper<T: Numeric>(Matrix<T>, Matrix<T>);