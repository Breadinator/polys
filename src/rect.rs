use std::f64::consts::PI;

use crate::Polygon;
use crate::tri::Tri;

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
	pub fn new(w: f64, h: f64) -> Option<Rect> {
		if w==0.0 || h==0.0 {
			None
		} else {
			Some(Rect { width: w, height: h })
		}
	}

	/// Returns a new Rect where both width and height are set to given value *w*.
	/// # Examples
	/// ```
	/// let square = polys::Rect::square(12.0);
	/// ```
	pub fn square(w: f64) -> Option<Rect> {
		if w == 0.0 {
			None
		} else {
			Some(Rect { width: w, height: w })
		}
	}

	/// Returns the triangle created by cutting the Rect through two opposite corners.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	///
	/// let rect = polys::Rect::new(3.0, 4.0)
	/// 	.expect("Cound not make Rect")
	/// 	.split()
	/// 	.expect("Could not convert to Tri");
	/// let tri = polys::Tri::new(3.0, 4.0, 5.0)
	/// 	.expect("Could not make Tri");
	/// 
	/// assert_eq!(rect.side3, tri.side3);
	/// ```
	pub fn split(&self) -> Option<Tri> {
		Tri::sas(self.width, self.height, PI/2.0)
	}
}

impl Polygon for Rect {
	/// Gets the area of the Rect according to its width and height.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	///
	/// let rect = polys::Rect::new(10.0, 5.0)
	/// 	.expect("Could not make Rect");
	/// let area = rect.area().expect("Is none");
	/// assert_eq!(area, 50.0);
	/// ```
	fn area(&self) -> Option<f64> {
		Some(&self.width * &self.height)
	}

	/// Gets the perimeter of the Rect from its width and height.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	///
	/// let rect = polys::Rect::new(10.0, 5.0)
	/// 	.expect("Could not make Rect");
	/// let peri = rect.peri().expect("Is none");
	/// assert_eq!(peri, 30.0);
	/// ```
	fn peri(&self) -> Option<f64> {
		Some(2.0 * &self.width + 2.0 * &self.height)
	}

	/// Returns interior angles (all 90) in degrees.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	///
	/// let rect = polys::Rect::new(10.0, 5.0)
	/// 	.expect("Could not make Rect");
	/// let angles = rect.angles().expect("Is none");
	/// assert_eq!((angles)[0], 90.0);
	/// ```
	fn angles(&self) -> Option<Vec<f64>> {
		Some(vec![90.0; 4])
	}
}