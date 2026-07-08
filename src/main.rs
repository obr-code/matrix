use matlib::matrix;
use matlib::collections::Matrix;

fn main() {
	println!("Chapitre 1");
	{
		println!("\nQuestion 1");
		{
			let mat = matrix![
				[15, 18],
				[23, 28],
				[27, 24],
			];
			println!("a:\n{:?}", mat);
			println!("d: {:?}", mat.get(2, 1).unwrap())
		}
		println!("\nQuestion 4");
		{
			let mat: Matrix<f64> = Matrix::from_fn(3, 3, |i: f64, j: f64|
				(-1f64).powf(i) * j
			);
			println!("a:\n{:?}", mat);

			let mat: Matrix<f64> = Matrix::from_fn(2, 3, |i: f64, j: f64|
				i + j.powf(2f64)
			);
			println!("b:\n{:?}", mat);

			let mat: Matrix<f64> = Matrix::from_fn(4, 1, |i: f64, j: f64|
				(-2f64).powf(i)
			);
			println!("c:\n{:?}", mat);

			let mat: Matrix<f64> = Matrix::from_fn(4, 3, |i: f64, j: f64|
				if i != j {
					i + j
				} else {
					0f64
				}
			);
			println!("d:\n{:?}", mat);

			let mat: Matrix<f64> = Matrix::from_fn(4, 3, |i: f64, j: f64|
				if i <= j {
					2f64 * j
				} else {
					-1f64.powf(i)
				}
			);
			println!("e:\n{:?}", mat);
		}
	}

	println!("Chapitre 2");
	{
		let a: Matrix<i64> = Matrix::from_fn(500, 500, |i, j|
			i + j
		);
		let b = &a * &a;
	}
}
