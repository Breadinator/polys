use crate::Polygon;

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
		Rect { width: w, height: h }
	}

	/// Returns a new Rect where both width and height are set to given value *w*.
	/// # Examples
	/// ```
	/// let square = polys::Rect::square(12.0);
	/// ```
	pub fn square(w: f64) -> Rect {
		Rect { width: w, height: w }
	}
}

impl Polygon for Rect {
	/// Gets the area of the Rect according to its width and height.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	///
	/// let rect = polys::Rect::new(10.0, 5.0);
	/// let area = rect.area();
	/// assert_eq!(area.expect("Is none"), 50.0);
	/// ```
	fn area(&self) -> Option<f64> {
		Some(&self.width * &self.height)
	}

	/// Gets the perimeter of the Rect from its width and height.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	///
	/// let rect = polys::Rect::new(10.0, 5.0);
	/// let peri = rect.peri();
	/// assert_eq!(peri.expect("Is none"), 30.0);
	/// ```
	fn peri(&self) -> Option<f64> {
		Some(2.0 * &self.width + 2.0 * &self.height)
	}

	/// Returns interior angles (all 90) in degrees.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	///
	/// let rect = polys::Rect::new(10.0, 5.0);
	/// let angles = rect.angles();
	/// assert_eq!((angles.expect("Is none"))[0], 90.0);
	/// ```
	fn angles(&self) -> Option<Vec<f64>> {
		Some(vec![90.0; 4])
	}
}