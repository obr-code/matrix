use matlib::matrix;
use matlib::collections::Matrix;
use macros::{ matrix, mathfn };

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
		mathfn!(a + b * c);
	}

	println!("Chapitre 3");
	{
		println!("{:?}",
			matrix![
				[1, 2, 3],
				[5, 6, 5],
				[7, 8, 9],
			].determinant()
		);
	}

	println!("Chapitre 5");
	{
		let mat = matrix![
			[ 1, 4, 1, 0, 0, 12],
			[ 2, 1, 0, 1, 0, 10],
			[-3,-3, 0, 0, 1,  0],
		];
		let vars = mat.maximize_linear();
		println!("{{ x = {}, y = {} }}",
			vars[0],
			vars[1],
		);

		println!()
	}
}
