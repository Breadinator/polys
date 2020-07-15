use core::f64::consts::PI;

use crate::Polygon;

/// Struct that represents a circle.
#[derive(Debug)]
pub struct Circle {
	pub radius: f64,
}

impl Circle {
	/// Returns a new Circle from given radius *r*.
	/// # Examples
	/// ```
	/// let circle = polys::Circle::new(5.0);
	/// ```
	pub fn new(r: f64) -> Circle { Circle {radius: r} }
}

impl Polygon for Circle {
	/// Gets the area of the Circle from its radius.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let circle = polys::Circle::new(5.0);
	/// let area = circle.area();
	/// assert_eq!(area, (std::f64::consts::PI * 25f64) as f64);
	/// ```
	fn area(&self) -> f64 {
		(PI * &self.radius * &self.radius) as f64
	}

	/// Gets the circumferance of the Circle from its radius.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// use core::f64::consts::PI;
	///
	/// let circle = polys::Circle::new(5.0);
	/// let peri = circle.peri();
	/// assert_eq!(peri, 10f64*PI);
	/// ```
	fn peri(&self) -> f64 {
		(2f64 * PI * &self.radius) as f64
	}

	/// Returns an empty vector.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let circle = polys::Circle::new(5.0);
	/// let angles = circle.angles();
	/// assert!(angles.is_empty());
	/// ```
	fn angles(&self) -> Vec<f64> { vec![] }
}