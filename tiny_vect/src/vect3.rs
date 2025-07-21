use std::convert::{From, TryFrom};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vect3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vect3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn length_squared(&self) -> f32 {
        let result = self.x * self.x + self.y * self.y + self.z * self.z;
        debug_assert!(
            result.is_finite(),
            "Vect3::length_squared produced NaN or infinity"
        );
        result
    }

    pub fn length(&self) -> f32 {
        let result = self.length_squared().sqrt();
        debug_assert!(result.is_finite(), "Vect3::length produced NaN or infinity");
        result
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        debug_assert!(len >= 0.0, "Vect3::normalize: length negative (impossible)");
        if len == 0.0 {
            *self
        } else {
            let result = *self / len;
            debug_assert!(
                result.x.is_finite() && result.y.is_finite() && result.z.is_finite(),
                "Vect3::normalize produced non-finite result"
            );
            result
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        let result = self.x * other.x + self.y * other.y + self.z * other.z;
        debug_assert!(result.is_finite(), "Vect3::dot produced NaN or infinity");
        result
    }

    pub fn cross(&self, other: &Self) -> Self {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        debug_assert!(
            x.is_finite() && y.is_finite() && z.is_finite(),
            "Vect3::cross produced non-finite result"
        );
        Self { x, y, z }
    }

    pub fn distance(&self, other: &Self) -> f32 {
        let result = (*self - *other).length();
        debug_assert!(
            result.is_finite(),
            "Vect3::distance produced NaN or infinity"
        );
        result
    }

    pub fn distance_squared(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        let result = dx * dx + dy * dy + dz * dz;
        debug_assert!(
            result.is_finite(),
            "Vect3::distance_squared produced NaN or infinity"
        );
        result
    }

    pub fn angle_between(&self, other: &Self) -> f32 {
        // Return zero for identical or zero-length vectors
        if self == other {
            return 0.0;
        }
        let denom = self.length() * other.length();
        if denom == 0.0 {
            return 0.0;
        }
        let cos = (self.dot(other) / denom).clamp(-1.0, 1.0);
        // Mitigate floating-point drift near 1.0
        if (cos - 1.0).abs() < f32::EPSILON {
            return 0.0;
        }
        let result = cos.acos();
        debug_assert!(
            result.is_finite(),
            "Vect3::angle_between produced NaN or infinity"
        );
        result
    }

    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let x = self.x + (other.x - self.x) * t;
        let y = self.y + (other.y - self.y) * t;
        let z = self.z + (other.z - self.z) * t;
        debug_assert!(
            x.is_finite() && y.is_finite() && z.is_finite(),
            "Vect3::lerp produced non-finite result"
        );
        Self { x, y, z }
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        let n = normal.normalize();
        let dot = self.dot(&n);
        let result = *self - n * (2.0 * dot);
        debug_assert!(
            result.x.is_finite() && result.y.is_finite() && result.z.is_finite(),
            "Vect3::reflect produced non-finite result"
        );
        result
    }

    pub fn project(&self, other: &Self) -> Self {
        let len_sq = other.length_squared();
        if len_sq == 0.0 {
            Vect3::default()
        } else {
            let scalar = self.dot(other) / len_sq;
            let result = *other * scalar;
            debug_assert!(
                result.x.is_finite() && result.y.is_finite() && result.z.is_finite(),
                "Vect3::project produced non-finite result"
            );
            result
        }
    }

    // Checked operations in debug
    pub fn debug_checked_add(self, other: Self) -> Self {
        let result = self + other;
        debug_assert!(
            result.x.is_finite() && result.y.is_finite() && result.z.is_finite(),
            "Vect3 overflow in add"
        );
        result
    }

    pub fn debug_checked_sub(self, other: Self) -> Self {
        let result = self - other;
        debug_assert!(
            result.x.is_finite() && result.y.is_finite() && result.z.is_finite(),
            "Vect3 overflow in sub"
        );
        result
    }

    pub fn debug_checked_mul(self, scalar: f32) -> Self {
        let result = self * scalar;
        debug_assert!(
            result.x.is_finite() && result.y.is_finite() && result.z.is_finite(),
            "Vect3 overflow in mul"
        );
        result
    }

    pub fn debug_checked_div(self, scalar: f32) -> Self {
        let result = self / scalar;
        debug_assert!(scalar != 0.0, "Vect3 division by zero");
        debug_assert!(
            result.x.is_finite() && result.y.is_finite() && result.z.is_finite(),
            "Vect3 overflow in div"
        );
        result
    }

    // Utility methods
    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }

    pub fn is_normalized(&self) -> bool {
        (self.length_squared() - 1.0).abs() < f32::EPSILON
    }

    pub fn is_parallel(&self, other: &Self) -> bool {
        self.cross(other).length_squared().abs() < f32::EPSILON
    }
}

// Arithmetic operations
impl Add for Vect3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Sub for Vect3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Mul<f32> for Vect3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Div<f32> for Vect3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl Neg for Vect3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl AddAssign for Vect3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl SubAssign for Vect3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl MulAssign<f32> for Vect3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl DivAssign<f32> for Vect3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

// Indexing
impl Index<usize> for Vect3 {
    type Output = f32;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vect3"),
        }
    }
}
impl IndexMut<usize> for Vect3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for Vect3"),
        }
    }
}

// From conversions
impl From<[f32; 3]> for Vect3 {
    fn from(arr: [f32; 3]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        }
    }
}
impl From<(f32, f32, f32)> for Vect3 {
    fn from(t: (f32, f32, f32)) -> Self {
        Self {
            x: t.0,
            y: t.1,
            z: t.2,
        }
    }
}
impl From<[i32; 3]> for Vect3 {
    fn from(arr: [i32; 3]) -> Self {
        Self {
            x: arr[0] as f32,
            y: arr[1] as f32,
            z: arr[2] as f32,
        }
    }
}
impl From<(i32, i32, i32)> for Vect3 {
    fn from(t: (i32, i32, i32)) -> Self {
        Self {
            x: t.0 as f32,
            y: t.1 as f32,
            z: t.2 as f32,
        }
    }
}
impl From<Vect3> for [f32; 3] {
    fn from(v: Vect3) -> Self {
        [v.x, v.y, v.z]
    }
}

// TryFrom slices
impl TryFrom<&[f32]> for Vect3 {
    type Error = &'static str;
    fn try_from(slice: &[f32]) -> Result<Self, Self::Error> {
        if slice.len() == 3 {
            Ok(Self {
                x: slice[0],
                y: slice[1],
                z: slice[2],
            })
        } else {
            Err("Expected slice of length 3 for Vect3<f32>")
        }
    }
}
impl TryFrom<&[i32]> for Vect3 {
    type Error = &'static str;
    fn try_from(slice: &[i32]) -> Result<Self, Self::Error> {
        if slice.len() == 3 {
            Ok(Self {
                x: slice[0] as f32,
                y: slice[1] as f32,
                z: slice[2] as f32,
            })
        } else {
            Err("Expected slice of length 3 for Vect3<i32>")
        }
    }
}

// Display
impl Display for Vect3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// Hash
impl Hash for Vect3 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.x.to_bits());
        state.write_u32(self.y.to_bits());
        state.write_u32(self.z.to_bits());
    }
}
