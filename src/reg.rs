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
	/// let reg = polys::Reg::new(3, 5);
	/// ```
	pub fn new(l: f64, s: f64) -> Reg {
		Reg { length: l, sides: s }
	}

	/// Returns a new 3-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// let reg = polys::Reg::tri(3);
	/// ```
	pub fn tri(l: f64) -> Reg {
		Reg { length: l, sides: 3f64 }
	}

	/// Returns a new 4-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// let reg = polys::Reg::quad(3);
	/// ```
	pub fn quad(l: f64) -> Reg {
		Reg { length: l, sides: 4f64 }
	}

	/// Returns a new 5-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// let reg = polys::Reg::pent(3);
	/// ```
	pub fn pent(l: f64) -> Reg {
		Reg { length: l, sides: 5f64 }
	}

	/// Returns a new 6-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// let reg = polys::Reg::hex(3);
	/// ```
	pub fn hex(l: f64) -> Reg {
		Reg { length: l, sides: 6f64 }
	}

	/// Returns a new 7-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// let reg = polys::Reg::sept(3);
	/// ```
	pub fn sept(l: f64) -> Reg {
		Reg { length: l, sides: 7f64 }
	}

	/// Returns a new 8-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// let reg = polys::Reg::oct(3);
	/// ```
	pub fn oct(l: f64) -> Reg {
		Reg { length: l, sides: 8f64 }
	}

	/// Returns a new 9-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// let reg = polys::Reg::non(3);
	/// ```
	pub fn non(l: f64) -> Reg {
		Reg { length: l, sides: 9f64 }
	}
}

impl Polygon for Reg {
	/// Gets the area of the Reg.
	/// # Examples
	/// ```
	/// let reg  = polys::Reg::new(3, 5);
	/// let area = reg.area();
	/// assert_eq!(area, 15.484296605300704);
	/// ```
	///
	/// This can also be seen by comparing the Rect square and Reg implementations.
	/// ```
	/// let square = polys::Rect::square(7).area();
	/// let reg    = polys::Reg::new(7, 4) .area();
	/// assert_eq!(square, reg);
	/// ```
	fn area(&self) -> f64 { 
		let peri  = &self.peri();
		let angle = (180f64/&self.sides)*(PI/180f64); //used because func uses radians
		let apoth = &self.length/(2f64*angle.tan());
		(peri * apoth)/2f64
	}

	/// Gets the perimeter of the Reg.
	/// # Examples
	/// This works on a regular polygon where it has 5 of more sides.
	/// ```
	/// let reg  = polys::Reg::new(3, 5);
	/// let peri = reg.peri();
	/// assert_eq!(peri, 15);
	/// ```
	///
	/// This can also be seen by comparing the Rect square and Reg implementations.
	/// ```
	/// let square = polys::Rect::square(7).peri();
	/// let reg    = polys::Reg::new(7, 4) .peri();
	/// assert_eq!(square, reg);
	/// ```
	fn peri(&self) -> f64 { 
		&self.length * &self.sides
	}
}