use std::f64::consts::PI;

/// Polygon trait for structs representing 2-dimensional shapes.
pub trait Polygon {
	fn area(&self) -> f64;
	fn peri(&self) -> f64;
}

/// Enum of all the polygon types
pub enum Poly {
	Rect(Rect),
	Circle(Circle),
	Tri(Tri),
	Reg(Reg),
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