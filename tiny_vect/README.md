# Tiny Vect

[![Crates.io](https://img.shields.io/crates/v/tiny_vect.svg)](https://crates.io/crates/tiny_vect)
[![Docs.rs](https://docs.rs/tiny_vect/badge.svg)](https://docs.rs/tiny_vect)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

A minimal vector math library for 2D and 3D operations in Rust.

## Features

- **2D Vectors** (`Vect2`) with comprehensive mathematical operations
- **3D Vectors** (`Vect3`) with cross product and 3D-specific operations
- **No dependencies** - pure Rust implementation
- **Debug assertions** for catching numerical errors during development

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tiny_vect = "0.1.0"
```

## Quick Start

```rust
use tiny_vect::{Vect2, Vect3};

fn main() {
    // 2D vectors
    let a = Vect2::new(3.0, 4.0);
    let b = Vect2::new(1.0, 2.0);
    
    println!("Vector a: {}", a);
    println!("Length of a: {}", a.length());
    println!("a + b: {}", a + b);
    println!("Dot product: {}", a.dot(&b));
    
    // 3D vectors
    let u = Vect3::new(1.0, 0.0, 0.0);
    let v = Vect3::new(0.0, 1.0, 0.0);
    
    println!("Cross product: {}", u.cross(&v));
    println!("Distance: {}", u.distance(&v));
}
```

## Core Operations

### 2D Vectors (`Vect2`)

- **Basic operations**: `+`, `-`, `*`, `/`, `+=`, `-=`, `*=`, `/=`
- **Mathematical functions**: `length()`, `normalize()`, `dot()`, `cross()`
- **Geometric operations**: `distance()`, `angle()`, `rotate()`, `lerp()`
- **Utility functions**: `is_zero()`, `is_normalized()`, `is_parallel()`

### 3D Vectors (`Vect3`)

- **All 2D operations** plus 3D-specific functionality
- **Cross product**: Returns a 3D vector perpendicular to both inputs
- **3D distance calculations**
- **3D angle calculations**

## Examples

### Vector Arithmetic

```rust
let a = Vect2::new(1.0, 2.0);
let b = Vect2::new(3.0, 4.0);

let sum = a + b;           // Vect2 { x: 4.0, y: 6.0 }
let diff = a - b;          // Vect2 { x: -2.0, y: -2.0 }
let scaled = a * 2.0;      // Vect2 { x: 2.0, y: 4.0 }
```

### Geometric Operations

```rust
let v = Vect2::new(3.0, 4.0);
let length = v.length();           // 5.0
let normalized = v.normalize();    // Vect2 { x: 0.6, y: 0.8 }

let a = Vect2::new(1.0, 0.0);
let b = Vect2::new(0.0, 1.0);
let angle = a.angle_between(&b);   // π/2 radians
```

### 3D Cross Product

```rust
let a = Vect3::new(1.0, 0.0, 0.0);
let b = Vect3::new(0.0, 1.0, 0.0);
let cross = a.cross(&b);           // Vect3 { x: 0.0, y: 0.0, z: 1.0 }
```

## License

Licensed under:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

