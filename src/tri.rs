use crate::Polygon;

/// Struct that represents a triangle.
#[derive(Debug)]
pub struct Tri {
	pub side1: f64,
	pub side2: f64,
	pub side3: f64,
}

impl Tri {
	/// Returns a new Tri from given sides *s1*, *s2*, *s3*.
	/// # Examples
	/// ```
	/// let tri = polys::Tri::new(24.0, 30.0, 18.0);
	/// ```
	pub fn new(s1: f64, s2: f64, s3: f64) -> Tri {
		Tri {side1: s1, side2: s2, side3: s3}
	}
}

impl Polygon for Tri {
	/// Gets the area of the Tri from its sides.
	/// # Examples
	/// ```
	/// let tri = polys::Tri::new(24.0, 30.0, 18.0);
	/// let area = tri.area();
	/// assert_eq!(area, 216 as f64);
	/// ```
	fn area(&self) -> f64 {
		let [a, b, c] = [&self.side1, &self.side2, &self.side3];
		let p: f64 = ((a+b+c)/2.0) as f64;
		let squared: f64 = p*(p-a)*(p-b)*(p-c);
		squared.sqrt()
	}

	/// Gets the perimeter of the Tri from its sides.
	/// # Examples
	/// ```
	/// let tri = polys::Tri::new(24.0, 30.0, 18.0);
	/// let peri = tri.peri();
	/// assert_eq!(peri, 72f64);
	/// ```
	fn peri(&self) -> f64 {
		(&self.side1 + &self.side2 + &self.side3) as f64
	}
}