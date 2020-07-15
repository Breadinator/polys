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
	/// use crate::polys::Polygon;
	///
	/// let tri = polys::Tri::new(24.0, 30.0, 18.0);
	/// let peri = tri.peri();
	/// assert_eq!(peri, 72f64);
	/// ```
	fn peri(&self) -> f64 {
		(&self.side1 + &self.side2 + &self.side3) as f64
	}
	
	/// Returns the a vector of three angles in radians such that `angles[0]` is the angle opposite `side1`, `angles[1]` is the angle opposite `side2`, and `angles[2]` is the angle opposite `side3`.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// use std::f64::consts::PI;
	///
	/// let tri = polys::Tri::new(24.0, 30.0, 24.0);
	/// let ang: Vec<f64> = tri.angles();
	/// assert_eq!(ang[0]+ang[1]+ang[2], PI);
	/// assert_eq!(ang[0], ang[2]); //iso tri
	///
	/// let tri = polys::Tri::new(12.0, 19.0, 8.0);
	/// let ang = tri.angles();
	/// assert_eq!(ang[0]+ang[1]+ang[2], PI);
	/// 
	/// let tri = polys::Tri::new(12.0, 12.0, 12.0);
	/// let ang = tri.angles();
	/// assert_eq!((ang[0]+ang[1]+ang[2]) as f32, PI as f32);
	/// ```
	fn angles(&self) -> Vec<f64> {
		let mut angles = Vec::new();
		let [a,  b,  c ] = [&self.side1, &self.side2, &self.side3];
		let [a2, b2, c2] = [a*a,         b*b,         c*c        ];

		let acos = (b2+c2-a2)/(2.0*b*c);
		let bcos = (a2+c2-b2)/(2.0*a*c);
		let ccos = (a2+b2-c2)/(2.0*a*b);

		let a = acos.acos();
		let b = bcos.acos();
		let c = ccos.acos();

		angles.push(a);
		angles.push(b);
		angles.push(c);

		angles
	}
}