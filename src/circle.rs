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
	/// use std::f64::consts::PI;
	///
	/// let circle = polys::Circle::new(5.0);
	/// let area = circle.area();
	/// assert_eq!(area.expect("Is none"), PI * 25.0);
	/// ```
	fn area(&self) -> Option<f64> {
		Some(PI * &self.radius * &self.radius)
	}

	/// Gets the circumferance of the Circle from its radius.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// use core::f64::consts::PI;
	///
	/// let circle = polys::Circle::new(5.0);
	/// let peri = circle.peri();
	/// assert_eq!(peri.expect("Is none"), 10.0*PI);
	/// ```
	fn peri(&self) -> Option<f64> {
		Some(2.0 * PI * &self.radius)
	}

	/// Returns an empty vector.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let circle = polys::Circle::new(5.0);
	/// let angles = circle.angles();
	/// assert!(angles.is_none());
	/// ```
	fn angles(&self) -> Option<Vec<f64>> { None }
}