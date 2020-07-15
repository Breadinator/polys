pub mod rect;
pub use crate::rect::Rect;
pub mod circle;
pub use crate::circle::Circle;
pub mod tri;
pub use crate::tri::Tri;
pub mod reg;
pub use crate::reg::Reg;

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