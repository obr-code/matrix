use std::sync::LazyLock;
use std::fmt;
use std::iter::{ Map, RepeatN, repeat_n, Product, Sum };
use std::ops::{ Add, Div, Index, IndexMut, Mul, Range, RangeFrom, RangeTo, Sub, Neg };
use std::cmp::{ Ord, Ordering };

use num_traits::NumCast;

use utils::Numeric;

// -- Utils -- //


/// A superposed matrix composed of 2 matrix.
pub struct MatrixSuper<T: Numeric>(Matrix<T>, Matrix<T>);

#[derive(PartialEq, Eq, Clone)]
pub struct Matrix<T: Numeric> (Vec<Vec<T>>);

impl<T> Matrix<T> where T: Numeric {
	// -- SUMMARY -- //


	// -- Utils -- //

	/// Return the matrix's dimensions.
	pub fn dimensions(&self) -> (usize, usize) {
		(self.m(), self.n())
	}

	/// Return the number of row in the matrix.
	pub fn m(&self) -> usize {
		self.0.len()
	}

	/// Return the number of columns in the matrix.
	pub fn n(&self) -> usize {
		self.0[0].len()
	}

	// -- Init -- //

	/// Solve a SLE with unknown constants but known coefficients by using Broy's method (me).
	/// 
	/// This enables O(M^2) time complexity for solving multiple SLE with matching coefficients.
	/// 
	/// Return Some(matrix) where the sum of row `i` 0..n-1 multiplicated by constant[i]
	/// is equal to the value of variable `i` multiplicated by matrix[i][n-1] row.
	/// 
	/// Return None if solution is not unique.
	pub fn broy(matrix: &Matrix<T>) -> Option<Self> { // O(M^3); O(M^2)
		let (m, n) = matrix.dimensions();
		let mut matrix = matrix.clone();
		let mut consts = Matrix::identity(m, n);

		for j1 in 0..n {
			if let Some(i1) = (0..m).find(|i1|
				matrix.column_pivot(*i1) == Some(j1)
			) {
				for i2 in (0..j1).chain(j1+1..m) {
					if let Some(lcd) = utils::lcd(matrix[i1][j1], matrix[i2][j1]) {
						let k1 = lcd / matrix[i1][j1];
						let k2 = lcd / matrix[i2][j1];

						for j2 in 0..n {
							matrix[i2][j2] *= k2;
							consts[i2][j2] *= k2;
							matrix[i2][j2] = matrix[i2][j2] - matrix[i1][j2] * k1;
							consts[i2][j2] = consts[i2][j2] - consts[i1][j2] * k1;
						}
					}
				}
			}
		}
		consts.push_col(matrix.diagonal().unwrap().collect());

		Some(consts)
	}

	/// Initialize a column matrix from a vector.
	pub fn column(vec: Vec<T>) -> Self {
		Matrix::from_fn(vec.len(), 1, |i, _|
			vec[<usize as NumCast>::from(i).unwrap()]
		)
	}

	/// Initialize a new matrix filled with `val`.
	pub fn fill(m: usize, n: usize, val: T) -> Self { // O(M*N); O(M*N)
		Matrix(
			vec![vec![val; n]; m]
		)
	}

	/// Initialize a new matrix from a grid of vectors.
	pub fn from(vec: Vec<Vec<T>>) -> Self { // O(M*N); O(1)
		Matrix(
			vec.into_iter().map(|row|
				row.into_iter().map(|val|
					val
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

	/// Initialize a identity matrix.
	pub fn identity(m: usize, n: usize) -> Self { // O(M*N); O(1)
		Matrix::scalar(m, n, T::one())
	}

	/// Initializie the inverse of a matrix
	/// 
	/// If it does not exist, create a copy of the matrix.
	pub fn inversed(matrix: &Matrix<T>) -> Self { // O(M^3); O(1)
		if let Some(det) = matrix.determinant() {
			&Self::transposed(matrix) * (T::one() / det)
		} else {
			matrix.clone()
		}
	}

	/// Initialize a new matrix filled with zeros.
	pub fn null(m: usize, n: usize) -> Self { // O(M*N); O(M*N)
		Matrix(
			vec![vec![T::zero(); n]; m]
		)
	}

	/// Create a new reduced matrix with Gauss' reduction method from another matrix.
	pub fn reduced(matrix: &Matrix<T>) -> Self { // O(MN*(M+N)); O(M*N)
		let mut matrix = matrix.clone();
		let (m, n) = matrix.dimensions();

		(0..m.min(n)).for_each(|p| // pivot
			(p+1..m).for_each(|i| {
				if let Some(lcd) = utils::lcd(matrix[i][p], matrix[p][p]) {
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
	pub fn row_reflected(matrix: &Matrix<T>) -> Self {
		Matrix::from_fn(matrix.m(), matrix.n(), |i, j|
			matrix.get(
				<usize as NumCast>::from(i).unwrap(),
				matrix.n()-1 - <usize as NumCast>::from(j).unwrap(),
			).unwrap()
		)
	}

	/// Initialize a new matrix from a given scalar.
	pub fn scalar(m: usize, n: usize, scalar: T) -> Self { // O(M*N); O(1)	
		Matrix(
			(0..m).map(|i|
				(0..n).map(|j|
					if i == j { scalar } else { T::zero() }
				).collect()
			).collect()
		)
	}

	/// Initialize a new matrix by transposing another one.
	pub fn transposed(matrix: &Matrix<T>) -> Self { // O(M*N); O(1)
		Matrix::from_fn(matrix.n(), matrix.m(), |i, j|
			matrix.get(
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
	pub fn dials(&self) -> [Matrix<T>; 4] {
		assert!(self.m() & 1 == 0);
		assert!(self.n() & 1 == 0);

		let m11 = self.sub_matrix(0..self.m()>>1, 0..self.n()>>1);
		let m12 = self.sub_matrix(0..self.m()>>1, self.n()>>1..self.n());
		let m21 = self.sub_matrix(self.m()>>1..self.m(), 0..self.n()>>1);
		let m22 = self.sub_matrix(self.m()>>1..self.m(), self.n()>>1..self.n());

		[m11, m12, m21, m22]
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

	/// Push a column with a matching length to the end of the matrix.
	pub fn push_col(&mut self, col: Vec<T>) { // O(N); O(N)
		assert_eq!(self.m(), col.len());
		(0..self.m()).for_each(|i|
			self.0[i].push(col[i])
		);
	}

	/// Remove the row at index `i` and return it.
	pub fn remove_row(&mut self, i: usize) -> Vec<T> {
		self.0.remove(i)
	}

	/// Remove the column at index `j` and return it.
	pub fn remove_col(&mut self, j: usize) -> Vec<T> {
		let mut out = vec![];
		for row in self.0.iter_mut() {
			out.push(row.remove(j));
		}
		out
	}

	// -- Get -- //

	/// Return a value at given coordonates. 
	/// 
	/// If coordonates are out of bound, return None.
	pub fn get(&self, i: usize, j: usize) -> Option<T> { // O(1); O(1)
		if i < self.m() && j < self.n() {
			Some(self[i][j])
		}
		else { None }
	}

	/// Return an iterator over all cells in matrix.
	pub fn iter_all(&self) -> impl Iterator<Item = &T> { // O(1); O(1)
		self.0.iter().flatten()
	}

	/// Return an iterator over row `i`.
	pub fn iter_row(&self, i: usize) -> Option<impl Iterator<Item = T>> { // O(1); O(1)
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
	pub fn iter_col(&self, j: usize) -> Option<impl Iterator<Item = T>> { // O(1); O(1)
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
	pub fn maximize_linear(&self) -> Vec<T> { // O(M^3); O(M*N)
		let (m, n) = self.dimensions();
		let mut vec = vec![0; m-1];
		let mut matrix = self.clone();

		for j1 in 0..m-1 {
			let iterator = (0..m-1).filter(|i| matrix[*i][j1] != T::zero());
			let i1 = iterator.clone().min_by(|i1, i2|
				(matrix[*i1][n-1] / matrix[*i1][j1]).partial_cmp(&(matrix[*i2][n-1] / matrix[*i2][j1])).unwrap()
			).unwrap();
			vec[j1] = i1;
			
			for i2 in (0..m-1).filter(|i2| *i2 != i1) {
				let lcd = utils::lcd(matrix[i1][j1], matrix[i2][j1]).unwrap();
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
	pub fn determinant(&self) -> Option<T> { // O(M^3); O(M*N)
		let mut matrix = self.clone();

		for i in 0..self.m() {
			let mut vec = vec![T::zero(); self.n()];
			vec[i] = T::one();
			matrix.0[i].append(&mut vec);
		}

		let matrix = Matrix::reduced(&matrix);
		
		Some(
			(0..self.m().min(self.n())).map(|i|
				matrix[i][i] / matrix[i][self.n() + i]
			).product()
		)
	}

	/// Return the cofactor of the given coordonates.
	pub fn cofactor(&self, i: usize, j: usize) -> Option<T> { // O(M^3); O(M*N)
		let det = self.minor(i, j)?;
		if (i ^ j) & 1 == 1 {
			Some(-det)
		} else {
			Some(det)
		}
	}

	/// Return the minor of the given coordonates.
	/// 
	/// The minor of a matrix at coordonate `i, j` is the determinant 
	/// of the matrix without row `i` and column `j`.
	pub fn minor(&self, i: usize, j: usize) -> Option<T> { // O(N*M)
		let mut matrix = self.clone();
		matrix.remove_row(i);
		matrix.remove_col(j);
		matrix.determinant()
	}

	/// Find the value of the pivot for a given row `i`.
	pub fn pivot(&self, i: usize) -> Option<T> { // O(N); O(1)
		if let Some(mut row) = self.iter_row(i) {
			row.find(|x| *x != T::zero())
		}
		else { None }
	}

	/// Find the column of the pivot for a given row `i`.
	pub fn column_pivot(&self, i: usize) -> Option<usize> { // O(N); O(1)
		if let Some(mut row) = self.iter_row(i) {
			row.position(|x| x != T::zero())
		}
		else { None }
	}

	/// Return the diagonal of the matrix.
	pub fn diagonal(&self) -> Option<impl Iterator<Item = T>> { // O(M); O(1)
		if self.is_square() {
			Some((0..self.m()).map(|i| self.get(i, i).unwrap()))
		}
		else { None }
	}

	/// Return the trace of the matrix.
	pub fn trace(&self) -> Option<T> { // O(M); O(1)
		if let Some(diagonal) = self.diagonal() {
			Some(diagonal.fold(T::zero(), |acc, cell| acc + cell))
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
				*cell != T::zero()
			).count() == 1
		)
	}

	/// Check if the matrix is upper-triangular.
	pub fn is_upper_triangular(&self) -> bool { // O(M*N); O(1)
		self.is_square()
		&& (1..self.m()).all(|i|
			(0..i).all(|j| 
				self.get(i, j) == Some(T::zero())
			)
		)
	}

	/// Check if the matrix is lower-triangular.
	pub fn is_lower_triangular(&self) -> bool { // O(M*N); O(1)
		self.is_square()
		&& (0..self.m()).all(|i|
			(i+1..self.n()).all(|j|
				self.get(i, j) == Some(T::zero())
			)
		)
	}

	/// Check if the matrix is diagonal.
	pub fn is_diagonal(&self) -> bool { // O(M*N); O(1)
		self.is_upper_triangular() && self.is_lower_triangular()
	}

	/// Check if the matrix is invertible.
	pub fn is_invertible(&self) -> bool {
		let det = self.determinant();
		det.is_some() && det != Some(T::zero())
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
		self.is_scalar() && self.get(0, 0) == Some(T::one())
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

	/// Check if the matrix is antisymetric.
	pub fn is_antisymetric(&self) -> bool { // O(M*N); O(1)
		self.is_square()
		&& (0..self.m()).all(|i|
			(i..self.n()).all(|j|
				self.get(i, j).unwrap() + self.get(j, i).unwrap() == T::zero()
			)
		)
	}

	/// Check if the matrix is a transition matrix.
	pub fn is_transition(&self) -> bool { // O(M*N); O(1)
		self.is_square()
		&& (0..self.n()).all(|j|
			(0..self.m()).fold(T::zero(), |acc, i|
				acc + self.get(i, j).unwrap()
			) == T::one()
		)
	}

	/// Check if the matrix is a level state matrix.
	pub fn is_level_state(&self) -> bool { // O(M); O(1)
		self.is_column()
		&& (0..self.m()).fold(T::zero(), |acc, i|
			acc + self.get(i, 0).unwrap()
		) == T::one()
	}

	/// Check if the matrix is regular.
	pub fn is_regular(&self) -> bool {
		self.determinant() != Some(T::zero())
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

impl<T> fmt::Debug for Matrix<T> where T: Numeric {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[");
		for i in 0..self.m() {
			write!(f, "\n[");
			for j in 0..self.n() {
				write!(f, "{},", <i64 as NumCast>::from(self[i][j]).unwrap());
			}
			write!(f, "]");
		}
		write!(f, "\n]")
	}
}

// -- Indexing -- //

/// Implement basic indexing for matrix.
impl<T> Index<usize> for Matrix<T> where T: Numeric {
	type Output = [T];
	fn index(&self, index: usize) -> &Self::Output { // O(1); O(1)
		&self.0[index]
	}
}

/// Implement mutable indexing for matrix.
impl<T> IndexMut<usize> for Matrix<T> where T: Numeric {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output { // O(1); O(1)
		&mut self.0[index]
	}
}

// -- Operations -- //

/// Implement the addition opperation between matrix.
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
impl<T> Add<Matrix<T>> for &Matrix<T> where T: Numeric {
	type Output = Matrix<T>;
	fn add(self, mut other: Matrix<T>) -> Self::Output { // O(M*N); O(1)
		assert_eq!(self.m(), other.m());
		assert_eq!(self.n(), other.n());
		(0..self.m()).for_each(|i|
			(0..self.n()).for_each(|j|
				other[i][j] += self[i][j]
			)
		);
		other
	}
}
impl<T> Add<&Matrix<T>> for Matrix<T> where T: Numeric {
	type Output = Matrix<T>;
	fn add(mut self, other: &Matrix<T>) -> Self::Output { // O(M*N); O(1)
		assert_eq!(self.m(), other.m());
		assert_eq!(self.n(), other.n());
		(0..self.m()).for_each(|i|
			(0..self.n()).for_each(|j|
				self[i][j] += other[i][j]
			)
		);
		self
	}
}
impl<T> Add for Matrix<T> where T: Numeric {
	type Output = Matrix<T>;
	fn add(mut self, other: Matrix<T>) -> Self::Output { // O(M*N); O(1)
		assert_eq!(self.m(), other.m());
		assert_eq!(self.n(), other.n());
		(0..self.m()).for_each(|i|
			(0..self.n()).for_each(|j|
				self[i][j] += other[i][j]
			)
		);
		self
	}
}

/// Implement the substraction opperation between matrix.
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
impl<T> Sub<&Matrix<T>> for Matrix<T> where T: Numeric {
	type Output = Matrix<T>;
	fn sub(mut self, other: &Matrix<T>) -> Self::Output { // O(M*N); O(1)
		assert_eq!(self.m(), other.m());
		assert_eq!(self.n(), other.n());
		(0..self.m()).for_each(|i|
			(0..self.n()).for_each(|j|
				self[i][j] -= other[i][j]
			)
		);
		self
	}
}
impl<T> Sub<Matrix<T>> for &Matrix<T> where T: Numeric {
	type Output = Matrix<T>;
	fn sub(self, mut other: Matrix<T>) -> Self::Output { // O(M*N); O(1)
		assert_eq!(self.m(), other.m());
		assert_eq!(self.n(), other.n());
		(0..self.m()).for_each(|i|
			(0..self.n()).for_each(|j|
				other[i][j] -= self[i][j]
			)
		);
		other
	}
}
impl<T> Sub for Matrix<T> where T: Numeric {
	type Output = Matrix<T>;
	fn sub(mut self, other: Matrix<T>) -> Self::Output { // O(M*N); O(1)
		assert_eq!(self.m(), other.m());
		assert_eq!(self.n(), other.n());
		(0..self.m()).for_each(|i|
			(0..self.n()).for_each(|j|
				self[i][j] -= other[i][j]
			)
		);
		self
	}
}

/// Implement the multiplication opperation between a scalar and a matrix.
impl<T> Mul<T> for &Matrix<T> where T: Numeric {
	type Output = Matrix<T>;
	fn mul(self, scalar: T) -> Self::Output { // O(M*N); O(1)
		Matrix(
			self.0.iter().map(|row|
				row.iter().map(|cell|
					*cell * scalar
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
impl<T> Mul for &Matrix<T> where  T: Numeric {
	type Output = Matrix<T>;

	fn mul(self, other: Self) -> Self::Output { // O(7^log M); O(M^2)

		fn naive<T: Numeric>(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T> {
			Matrix::from_fn(a.m(), b.n(), |i, j|
				a.iter_row(<usize as NumCast>::from(i).unwrap()).unwrap().zip(b.iter_col(<usize as NumCast>::from(j).unwrap()).unwrap()).fold(T::zero(), |acc, (a, b)|
					acc + a * b
				)
			)
		}

		fn strassen<T: Numeric>(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T> {
			fn process<T: Numeric>(a: Matrix<T>, b: Matrix<T>) -> Matrix<T> {
				let [a11, a12, a21, a22] = a.dials();
				let [b11, b12, b21, b22] = b.dials();

				let m1 = (&a11 + &a22) * (&b11 + &b22);
				let m2 = (&a21 + &a22) * &b11;
				let m3 = &a11 * (&b12 - &b22);
				let m4 = &a22 * (&b21 - &b11);
				let m5 = (&a11 + &a12) * &b22;
				let m6 = (&a21 - &a11) * (&b11 + &b12);
				let m7 = (&a12 - &a22) * (&b21 + &b22);

				let c11 = (&m1 + &m4) - (&m5 - &m7);
				let c12 = &m3 + &m5;
				let c21 = &m2 + &m4;
				let c22 = (&m1 - &m2) + (&m3 + &m6);
				
				Matrix::from_dials(&c11, &c12, &c21, &c22)
			}
			// `len` is the smallest even length to resize `a` and `b` into square matrices with same dimensions.
			let len = (a.m().max(a.n()).max(b.n()) + 1) & (!1);
			let mut a_copy: Matrix<T> = a.clone();
			let mut b_copy: Matrix<T> = b.clone();
			a_copy.resize(len, len);
			b_copy.resize(len, len);

			process(a_copy, b_copy)
		}

		if self.n() * self.m() < 100_000 || (
			self.m() * self.n() * (self.m() + self.n())
			<
			7_usize.pow(self.m().max(self.n()).max(other.n()).ilog2())
		) {
			naive(&self, &other)
		} else {
			strassen(&self, &other)
		}
	}

}

impl<T> Mul<&Matrix<T>> for Matrix<T> where T: Numeric {
	type Output = Matrix<T>;
	fn mul(self, other: &Matrix<T>) -> Self::Output { // O(M*N); O(1)
		&self * other
	}
}
impl<T> Mul<Matrix<T>> for &Matrix<T> where T: Numeric {
	type Output = Matrix<T>;
	fn mul(self, other: Matrix<T>) -> Self::Output { // O(M*N); O(1)
		self * &other
	}
}
impl<T> Mul for Matrix<T> where T: Numeric {
	type Output = Matrix<T>;
	fn mul(self, other: Matrix<T>) -> Self::Output { // O(M*N); O(1)
		&self * &other
	}
}

/// Initialize a matrix.
/// 
/// # Examples
/// 
/// ```
/// let mat = matrix![
/// 	[0, 1],
/// 	[2, 3],
/// ];
/// ```
#[macro_export]
macro_rules! matrix { // O(M*N); O(M*N)
	( $([ $( $elem:expr ),* ]),* $(,)? ) => {
		{
			let grid: Vec<Vec<_>> = vec![
				$( vec![ $( $elem ),* ] ),*
			];
			let n = grid[0].len();
			assert!(grid.iter().all(|row| row.len() == n));
			Matrix::from(grid)
		}
	};
	( $lit:literal; $m:expr; $n:expr ) => {
		Matrix::fill($lit, $m, $n)
	}
}