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
    pub fn new(r: f64) -> Option<Circle> {
        if r == 0.0 {
            None
        } else {
            Some(Circle { radius: r })
        }
    }
}

impl Polygon for Circle {
    /// Gets the area of the Circle from its radius.
    /// # Examples
    /// ```
    /// use crate::polys::Polygon;
    /// use std::f64::consts::PI;
    ///
    /// let circle = polys::Circle::new(5.0)
    ///     .expect("Could not make Circle");
    /// let area = circle.area().expect("Is none");
    /// assert_eq!(area, PI * 25.0);
    /// ```
    fn area(&self) -> Option<f64> {
        Some(PI * self.radius * self.radius)
    }

    /// Gets the circumferance of the Circle from its radius.
    /// # Examples
    /// ```
    /// use crate::polys::Polygon;
    /// use core::f64::consts::PI;
    ///
    /// let circle = polys::Circle::new(5.0)
    ///     .expect("Could not make Circle");
    /// let peri = circle.peri().expect("Is none");
    /// assert_eq!(peri, 10.0*PI);
    /// ```
    fn peri(&self) -> Option<f64> {
        Some(2.0 * PI * self.radius)
    }

    /// Returns None.
    /// # Examples
    /// ```
    /// use crate::polys::Polygon;
    /// let circle = polys::Circle::new(5.0)
    ///     .expect("Could not make Circle");
    /// let angles = circle.angles();
    /// assert!(angles.is_none());
    /// ```
    fn angles(&self) -> Option<Vec<f64>> {
        None
    }
}
