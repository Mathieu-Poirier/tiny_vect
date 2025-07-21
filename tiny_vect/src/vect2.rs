use std::convert::{From, TryFrom};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vect2 {
    pub x: f32,
    pub y: f32,
}

impl Vect2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn length_squared(&self) -> f32 {
        let result = self.x * self.x + self.y * self.y;
        debug_assert!(
            result.is_finite(),
            "Vect2::length_squared produced NaN or infinity"
        );
        result
    }

    pub fn length(&self) -> f32 {
        let result = (self.x * self.x + self.y * self.y).sqrt();
        debug_assert!(result.is_finite(), "Vect2::length produced NaN or infinity");
        result
    }

    pub fn normalize(&self) -> Self {
        // Compute squared length without any early debug_assert
        let sq = self.x * self.x + self.y * self.y;
        // If that overflowed to infinity or is NaN, error out here
        debug_assert!(
            sq.is_finite(),
            "Vect2::normalize produced non-finite result"
        );

        // Safe to sqrt now
        let len = sq.sqrt();
        // Zero‑length stays zero‑vector
        if len == 0.0 {
            *self
        } else {
            let result = *self / len;
            // Final sanity check (should never fire if sq was finite)
            debug_assert!(
                result.x.is_finite() && result.y.is_finite(),
                "Vect2::normalize produced non-finite result"
            );
            result
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        let result = self.x * other.x + self.y * other.y;
        debug_assert!(result.is_finite(), "Vect2::dot produced NaN or infinity");
        result
    }

    pub fn cross(&self, other: &Self) -> f32 {
        let result = self.x * other.y - self.y * other.x;
        debug_assert!(result.is_finite(), "Vect2::cross produced NaN or infinity");
        result
    }

    pub fn rotate(&self, angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        let x = self.x * cos - self.y * sin;
        let y = self.x * sin + self.y * cos;
        debug_assert!(
            x.is_finite() && y.is_finite(),
            "Vect2::rotate produced non-finite result"
        );
        Self { x, y }
    }

    pub fn distance(&self, other: &Self) -> f32 {
        let result = (*self - *other).length();
        debug_assert!(
            result.is_finite(),
            "Vect2::distance produced NaN or infinity"
        );
        result
    }

    pub fn distance_squared(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let result = dx * dx + dy * dy;
        debug_assert!(
            result.is_finite(),
            "Vect2::distance_squared produced NaN or infinity"
        );
        result
    }

    pub fn angle(&self, other: &Self) -> f32 {
        let dot = self.dot(other);
        let cross = self.cross(other);
        let result = cross.atan2(dot);
        debug_assert!(result.is_finite(), "Vect2::angle produced NaN or infinity");
        result
    }

    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let x = self.x + (other.x - self.x) * t;
        let y = self.y + (other.y - self.y) * t;
        debug_assert!(
            x.is_finite() && y.is_finite(),
            "Vect2::lerp produced non-finite result"
        );
        Self { x, y }
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        let normal = normal.normalize();
        let result = *self - normal * 2.0 * self.dot(&normal);
        debug_assert!(
            result.x.is_finite() && result.y.is_finite(),
            "Vect2::reflect produced non-finite result"
        );
        result
    }

    pub fn project(&self, other: &Self) -> Self {
        let len_sq = other.length_squared();
        if len_sq == 0.0 {
            Vect2::default()
        } else {
            let scalar = self.dot(other) / len_sq;
            let result = *other * scalar;
            debug_assert!(
                result.x.is_finite() && result.y.is_finite(),
                "Vect2::project produced non-finite result"
            );
            result
        }
    }

    pub fn angle_between(&self, other: &Self) -> f32 {
        // Identical vectors → zero
        if self == other {
            return 0.0;
        }
        // Guard against zero‑length
        let denom = self.length() * other.length();
        if denom == 0.0 {
            return 0.0;
        }
        // Compute cosine, clamped to [-1,1]
        let cos = (self.dot(other) / denom).clamp(-1.0, 1.0);
        // Mitigate tiny rounding drift near 1.0
        if (cos - 1.0).abs() < f32::EPSILON {
            return 0.0;
        }
        let result = cos.acos();
        debug_assert!(
            result.is_finite(),
            "Vect2::angle_between produced NaN or infinity"
        );
        result
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    pub fn is_normalized(&self) -> bool {
        (self.length_squared() - 1.0).abs() < f32::EPSILON
    }

    pub fn is_parallel(&self, other: &Self) -> bool {
        self.cross(other).abs() < f32::EPSILON
    }
}

// Checked operations
impl Vect2 {
    pub fn debug_checked_add(self, other: Self) -> Self {
        let result = self + other;
        debug_assert!(
            result.x.is_finite() && result.y.is_finite(),
            "Vect2 overflow in add"
        );
        result
    }

    pub fn debug_checked_sub(self, other: Self) -> Self {
        let result = self - other;
        debug_assert!(
            result.x.is_finite() && result.y.is_finite(),
            "Vect2 overflow in sub"
        );
        result
    }

    pub fn debug_checked_mul(self, scalar: f32) -> Self {
        let result = self * scalar;
        debug_assert!(
            result.x.is_finite() && result.y.is_finite(),
            "Vect2 overflow in mul"
        );
        result
    }

    pub fn debug_checked_div(self, scalar: f32) -> Self {
        let result = self / scalar;
        debug_assert!(scalar != 0.0, "Vect2 division by zero");
        debug_assert!(
            result.x.is_finite() && result.y.is_finite(),
            "Vect2 overflow in div"
        );
        result
    }
}

// Arithmetic operations
impl Add for Vect2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vect2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Vect2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Vect2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Neg for Vect2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl AddAssign for Vect2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Vect2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl MulAssign<f32> for Vect2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl DivAssign<f32> for Vect2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

// Indexing
impl Index<usize> for Vect2 {
    type Output = f32;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds for Vect2"),
        }
    }
}

impl IndexMut<usize> for Vect2 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds for Vect2"),
        }
    }
}

// From conversions
impl From<[f32; 2]> for Vect2 {
    fn from(arr: [f32; 2]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
        }
    }
}

impl From<(f32, f32)> for Vect2 {
    fn from(tuple: (f32, f32)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl From<[i32; 2]> for Vect2 {
    fn from(arr: [i32; 2]) -> Self {
        Self {
            x: arr[0] as f32,
            y: arr[1] as f32,
        }
    }
}

impl From<(i32, i32)> for Vect2 {
    fn from(tuple: (i32, i32)) -> Self {
        Self {
            x: tuple.0 as f32,
            y: tuple.1 as f32,
        }
    }
}

impl From<Vect2> for [f32; 2] {
    fn from(v: Vect2) -> Self {
        [v.x, v.y]
    }
}

impl TryFrom<&[f32]> for Vect2 {
    type Error = &'static str;

    fn try_from(slice: &[f32]) -> Result<Self, Self::Error> {
        if slice.len() == 2 {
            Ok(Self {
                x: slice[0],
                y: slice[1],
            })
        } else {
            Err("Expected slice of length 2 for Vect2<f32>")
        }
    }
}

impl TryFrom<&[i32]> for Vect2 {
    type Error = &'static str;

    fn try_from(slice: &[i32]) -> Result<Self, Self::Error> {
        if slice.len() == 2 {
            Ok(Self {
                x: slice[0] as f32,
                y: slice[1] as f32,
            })
        } else {
            Err("Expected slice of length 2 for Vect2<i32>")
        }
    }
}

// Display and parsing
impl Display for Vect2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Hashing
impl Hash for Vect2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.x.to_bits());
        state.write_u32(self.y.to_bits());
    }
}
