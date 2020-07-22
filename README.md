# polys
[![Crates.io](https://img.shields.io/crates/v/polys)](https://crates.io/crates/polys)
[![Docs](https://docs.rs/polys/badge.svg)](https://docs.rs/polys)

polys is a Rust crate implementing basic polygons as structs, all implementing a trait which gives the basic functions associated with polygons.

## Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
polys = "0.4.2"
```

## Usage
The following is the `main.rs` file of my test program. It shows the area and perimeter of a rectangle, a triangle, and a circle.
```rust
use polys::{Polygon, Rect, Tri, Circle, Reg};

fn main() {
    let poly = Rect::new(12.0, 6.0).expect("Could not make Rect");
    println!("{:?}\n    area: {}, peri: {}\n", &poly, &poly.area().expect("Is none"), &poly.peri().expect("Is none"));

    let poly = Tri::new(24.0, 30.0, 18.0).expect("Could not make Tri");
    println!("{:?}\n    area: {}, peri: {}\n", &poly, &poly.area().expect("Is none"), &poly.peri().expect("Is none"));

    let poly = Circle::new(5.0).expect("Could not make Circle");
    println!("{:?}\n    area: {}, peri: {}\n", &poly, &poly.area().expect("Is none"), &poly.peri().expect("Is none"));

    let poly = Reg::new(3.0, 5.0).expect("Could not make Reg");
    println!("{:?}\n    area: {}, peri: {}\n", &poly, &poly.area().expect("Is none"), &poly.peri().expect("Is none"));
}
```

The output of this program returns the following:
```
Rect { width: 12.0, height: 6.0 }
    area: 72, peri: 36

Tri { side1: 24.0, side2: 30.0, side3: 18.0 }
    area: 216, peri: 72

Circle { radius: 5.0 }
    area: 78.53981633974483, peri: 31.41592653589793

Reg { length: 3.0, sides: 5.0 }
    area: 15.484296605300704, peri: 15
```

## License
polys is distributed under the MIT license. See [LICENSE](LICENSE).