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
	/// use crate::polys::Polygon;
	///
	/// let tri = polys::Tri::new(24.0, 30.0, 18.0);
	/// let area = tri.area().expect("Is none");
	/// assert_eq!(area, 216.0);
	/// ```
	fn area(&self) -> Option<f64> {
		let [a, b, c] = [&self.side1, &self.side2, &self.side3];
		let p = (a+b+c)/2.0;
		let squared = p*(p-a)*(p-b)*(p-c);
		Some(squared.sqrt())
	}

	/// Gets the perimeter of the Tri from its sides.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	///
	/// let tri = polys::Tri::new(24.0, 30.0, 18.0);
	/// let peri = tri.peri().expect("Is none");
	/// assert_eq!(peri, 72.0);
	/// ```
	fn peri(&self) -> Option<f64> {
		Some(&self.side1 + &self.side2 + &self.side3)
	}
	
	/// Returns the a vector of three angles in radians such that `angles[0]` is the angle opposite `side1`, `angles[1]` is the angle opposite `side2`, and `angles[2]` is the angle opposite `side3`.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// use std::f64::consts::PI;
	///
	/// let tri = polys::Tri::new(24.0, 30.0, 24.0);
	/// let ang = tri.angles().expect("Is none");
	/// assert_eq!(ang[0]+ang[1]+ang[2], PI);
	/// assert_eq!(ang[0], ang[2]); //iso tri
	///
	/// let tri = polys::Tri::new(12.0, 19.0, 8.0);
	/// let ang = tri.angles().expect("Is none");
	/// assert_eq!(ang[0]+ang[1]+ang[2], PI);
	/// 
	/// let tri = polys::Tri::new(12.0, 12.0, 12.0);
	/// let ang = tri.angles().expect("Is none");
	/// assert_eq!((ang[0]+ang[1]+ang[2]) as f32, PI as f32);
	/// ```
	fn angles(&self) -> Option<Vec<f64>> {
		let mut angles = Vec::new();
		let sides = [&self.side1, &self.side2, &self.side3];

		let cosine_rule = |i: usize| -> f64 {
			let [a, b, c] = [
				sides[i],
				sides[(i+1)%3],
				sides[(i+2)%3]
			];
			(((b*b)+(c*c)-(a*a))/(2.0*b*c)).acos()
		};

		for i in 0..3 {
			let angle = cosine_rule(i as usize);
			if !angle.is_normal() { return None; }
			angles.push(angle);
		}

		Some(angles)
	}
}