use crate::Polygon;

use std::f64::consts::PI;

/// Struct that represents a regular polygon.
#[derive(Debug)]
pub struct Reg {
	length: f64,
	sides:  f64,
}

impl Reg {
	/// Returns a new Reg based on a given length *l* and sides *s*
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let reg = polys::Reg::new(3.0, 5.0);
	/// ```
	pub fn new(l: f64, s: f64) -> Reg {
		Reg { length: l, sides: s }
	}

	/// Returns a new 3-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let reg = polys::Reg::tri(3.0);
	/// ```
	pub fn tri(l: f64) -> Reg {
		Reg { length: l, sides: 3.0 }
	}

	/// Returns a new 4-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let reg = polys::Reg::quad(3.0);
	/// ```
	pub fn quad(l: f64) -> Reg {
		Reg { length: l, sides: 4.0 }
	}

	/// Returns a new 5-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let reg = polys::Reg::pent(3.0);
	/// ```
	pub fn pent(l: f64) -> Reg {
		Reg { length: l, sides: 5.0 }
	}

	/// Returns a new 6-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let reg = polys::Reg::hex(3.0);
	/// ```
	pub fn hex(l: f64) -> Reg {
		Reg { length: l, sides: 6.0 }
	}

	/// Returns a new 7-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let reg = polys::Reg::sept(3.0);
	/// ```
	pub fn sept(l: f64) -> Reg {
		Reg { length: l, sides: 7.0 }
	}

	/// Returns a new 8-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let reg = polys::Reg::oct(3.0);
	/// ```
	pub fn oct(l: f64) -> Reg {
		Reg { length: l, sides: 8.0 }
	}

	/// Returns a new 9-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let reg = polys::Reg::non(3.0);
	/// ```
	pub fn non(l: f64) -> Reg {
		Reg { length: l, sides: 9.0 }
	}
}

impl Polygon for Reg {
	/// Gets the area of the Reg.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let reg  = polys::Reg::new(3.0, 5.0);
	/// let area = reg.area().expect("Is none");
	/// assert_eq!(area, 15.484296605300704);
	/// ```
	///
	/// This can also be seen by comparing the Rect square and Reg implementations.
	/// ```
	/// use crate::polys::Polygon;
	/// let square = polys::Rect::square(7.0).area().expect("Is none");
	/// let reg = polys::Reg::new(7.0, 4.0).area().expect("Is none");
	/// assert_eq!(square as i32, reg as i32); // casted to i32 because floats suck :)
	/// ```
	fn area(&self) -> Option<f64> { 
		let p  = &self.peri();

		match p {
			Some(perimeter) => {
				let angle = PI/&self.sides;
				let apoth = &self.length/(2.0*angle.tan());
				Some(perimeter * apoth * 0.5)
			},
			None => None,
		}
	}

	/// Gets the perimeter of the Reg.
	/// # Examples
	/// This works on a regular polygon where it has 5 of more sides.
	/// ```
	/// use crate::polys::Polygon;
	/// let reg  = polys::Reg::new(3.0, 5.0);
	/// let peri = reg.peri().expect("Is none");
	/// assert_eq!(peri, 15.0);
	/// ```
	///
	/// This can also be seen by comparing the Rect square and Reg implementations.
	/// ```
	/// use crate::polys::Polygon;
	/// let square = polys::Rect::square(7.0) .peri();
	/// let reg    = polys::Reg::new(7.0, 4.0).peri();
	/// ```
	fn peri(&self) -> Option<f64> { 
		Some(&self.length * &self.sides)
	}

	/// Returns the size of the interior angles in degrees.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let poly = polys::Reg::pent(12.0);
	/// let angles = poly.angles().expect("Is none");
	/// assert_eq!(angles[0], 108.0);
	/// ```
	fn angles(&self) -> Option<Vec<f64>> {
		let total = 180.0*(&self.sides-2.0);
		Some(vec![total/&self.sides; *&self.sides as usize])
	}
}