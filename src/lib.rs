use std::f64::consts::PI;

/// Polygon trait for structs representing 2-dimensional shapes.
pub trait Polygon {
	fn area(&self)    -> f64;
	fn peri(&self) -> f64;
}

/// Struct that represents a rectangle.
#[derive(Debug)]
pub struct Rect {
	pub width:  f64,
	pub height: f64,
}

impl Rect {
	/// Returns a new Rect from given width *w* and height *h*.
	/// # Examples
	/// ```
	/// let rect = polys::Rect::new(12.0, 6.0);
	/// ```
	pub fn new(w: f64, h: f64) -> Rect {
		Rect {width: w, height: h}
	}

	/// Returns a new Rect where both width and height are set to given value *w*.
	/// # Examples
	/// ```
	/// let square = polys::Rect::square(12.0);
	/// ```
	pub fn square(w: f64) -> Rect {
		Rect {width: w, height: w}
	}
}

impl Polygon for Rect {
	/// Gets the area of the Rect according to its width and height.
	/// # Examples
	/// ```
	/// let rect = polys::Rect::new(10.0, 5.0);
	/// let area = rect.area();
	/// assert_eq!(area, 50.0);
	/// ```
	fn area(&self) -> f64 {
		(&self.width * &self.height) as f64
	}

	/// Gets the perimeter of the Rect from its width and height,
	/// # Examples
	/// ```
	/// let rect = polys::Rect::new(10.0, 5.0);
	/// let peri = rect.peri();
	/// assert_eq!(peri, 30f64);
	/// ```
	fn peri(&self) -> f64 {
		2f64 * &self.width + 2f64 * &self.height
	}
}

/// Struct that represents a circle.
#[derive(Debug)]
pub struct Circle {
	pub radius: f64,
}

impl Circle {
	/// Returns a new Circle from given radius *r*.
	/// ```
	/// let circle = polys::Circle::new(5.0);
	/// ```
	pub fn new(r: f64) -> Circle { Circle {radius: r} }
}

impl Polygon for Circle {
	/// Gets the area of the Circle from its radius.
	/// ```
	/// let circle = polys::Circle::new(5.0);
	/// let area = circle.area();
	/// assert_eq!(area, (std::f64::consts::PI * 25) as f64);
	/// ```
	fn area(&self) -> f64 {
		(PI * &self.radius * &self.radius) as f64
	}

	/// Gets the circumferance of the Circle from its radius.
	/// ```
	/// let circle = polys::Circle::new(5.0);
	/// let peri = circle.peri();
	/// assert_eq(peri, 4f64*PI);
	/// ```
	fn peri(&self) -> f64 {
		(2f64 * PI * &self.radius) as f64
	}
}

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