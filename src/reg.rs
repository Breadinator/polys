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
	/// let reg = polys::Reg::new(3f64, 5f64);
	/// ```
	pub fn new(l: f64, s: f64) -> Reg {
		Reg { length: l, sides: s }
	}

	/// Returns a new 3-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let reg = polys::Reg::tri(3f64);
	/// ```
	pub fn tri(l: f64) -> Reg {
		Reg { length: l, sides: 3f64 }
	}

	/// Returns a new 4-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let reg = polys::Reg::quad(3f64);
	/// ```
	pub fn quad(l: f64) -> Reg {
		Reg { length: l, sides: 4f64 }
	}

	/// Returns a new 5-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let reg = polys::Reg::pent(3f64);
	/// ```
	pub fn pent(l: f64) -> Reg {
		Reg { length: l, sides: 5f64 }
	}

	/// Returns a new 6-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let reg = polys::Reg::hex(3f64);
	/// ```
	pub fn hex(l: f64) -> Reg {
		Reg { length: l, sides: 6f64 }
	}

	/// Returns a new 7-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let reg = polys::Reg::sept(3f64);
	/// ```
	pub fn sept(l: f64) -> Reg {
		Reg { length: l, sides: 7f64 }
	}

	/// Returns a new 8-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let reg = polys::Reg::oct(3f64);
	/// ```
	pub fn oct(l: f64) -> Reg {
		Reg { length: l, sides: 8f64 }
	}

	/// Returns a new 9-sided Reg with sides length *l*.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let reg = polys::Reg::non(3f64);
	/// ```
	pub fn non(l: f64) -> Reg {
		Reg { length: l, sides: 9f64 }
	}
}

impl Polygon for Reg {
	/// Gets the area of the Reg.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let reg  = polys::Reg::new(3f64, 5f64);
	/// let area = reg.area();
	/// assert_eq!(area, 15.484296605300704);
	/// ```
	///
	/// This can also be seen by comparing the Rect square and Reg implementations.
	/// ```
	/// use crate::polys::Polygon;
	/// let square = polys::Rect::square(7f64).area();
	/// let reg    = polys::Reg::new(7f64, 4f64) .area();
	/// assert_eq!(square as i32, reg as i32); // casted to i32 because floats suck :)
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
	/// use crate::polys::Polygon;
	/// let reg  = polys::Reg::new(3f64, 5f64);
	/// let peri = reg.peri();
	/// assert_eq!(peri, 15f64);
	/// ```
	///
	/// This can also be seen by comparing the Rect square and Reg implementations.
	/// ```
	/// use crate::polys::Polygon;
	/// let square = polys::Rect::square(7f64).peri();
	/// let reg    = polys::Reg::new(7f64, 4f64) .peri();
	/// ```
	fn peri(&self) -> f64 { 
		&self.length * &self.sides
	}

	/// Returns the size of the interior angles in degrees.
	/// # Examples
	/// ```
	/// use crate::polys::Polygon;
	/// let poly = polys::Reg::pent(12f64);
	/// 
	/// ```
	fn angles(&self) -> Vec<f64> {
		let total = 180f64*(&self.sides-2f64);
		vec![total/&self.sides; *&self.sides as usize]
	}
}