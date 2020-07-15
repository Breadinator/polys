use std::f64::consts::PI;

/// Polygon trait for structs representing 2-dimensional shapes.
pub trait Polygon {
	fn area(&self) -> f64;
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
	fn area(&self) -> f64 { (&self.width * &self.height) as f64 }
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
	///
	/// ```
	/// let circle = polys::Circle::new(5.0);
	/// let area = circle.area();
	/// assert_eq!(area, (std::f64::consts::PI * 25) as f64);
	/// ```
	fn area(&self) -> f64 { (PI * &self.radius * &self.radius) as f64 }
}