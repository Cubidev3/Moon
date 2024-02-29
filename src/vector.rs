use std::fmt::{Display, Formatter, write};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::vector_base;

vector_base!(Vector<f64> {x,y,z});

impl Vector {
    pub(crate) const ZERO: Vector = Vector { x: 0f64, y: 0f64, z: 0f64 };
    pub fn from(x: f64, y: f64, z: f64) -> Vector {
        Vector{ x, y, z }
    }

    pub fn dot(self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vector) -> Vector {
        let x: f64 = self.y * other.z - self.z * other.y;
        let y: f64 = self.z * other.x - self.x * other.z;
        let z: f64 = self.x * other.y - self.y * other.x;
        Vector { x, y, z }
    }

    pub fn reflected(self, axis: Vector) -> Vector{
        self - 2f64 * self.dot(axis) * axis
    }

    pub fn rotated(self, angle: f64, axis: Vector) -> Vector {
        let axis = axis.normalized_or_zero();
        let sin_theta = angle.sin();
        let cos_theta = angle.cos();
        let rot_matrix = [
            [
                cos_theta + axis.x * axis.x * (1.0 - cos_theta),
                axis.x * axis.y * (1.0 - cos_theta) - axis.z * sin_theta,
                axis.x * axis.z * (1.0 - cos_theta) + axis.y * sin_theta,
            ],
            [
                axis.y * axis.x * (1.0 - cos_theta) + axis.z * sin_theta,
                cos_theta + axis.y * axis.y * (1.0 - cos_theta),
                axis.y * axis.z * (1.0 - cos_theta) - axis.x * sin_theta,
            ],
            [
                axis.z * axis.x * (1.0 - cos_theta) - axis.y * sin_theta,
                axis.z * axis.y * (1.0 - cos_theta) + axis.x * sin_theta,
                cos_theta + axis.z * axis.z * (1.0 - cos_theta),
            ],
        ];

        Vector {
            x: self.dot(Vector::from(rot_matrix[0][0], rot_matrix[0][1], rot_matrix[0][2])),
            y: self.dot(Vector::from(rot_matrix[1][0], rot_matrix[1][1], rot_matrix[1][2])),
            z: self.dot(Vector::from(rot_matrix[2][0], rot_matrix[2][1], rot_matrix[2][2])),
        }
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalized(self) -> Option<Vector> {
        let length: f64 = self.length();
        if length < f64::EPSILON {
            return None;
        }

        Some(self / length)
    }

    pub fn normalized_or_zero(self) -> Vector {
        self.normalized().unwrap_or(Vector::ZERO)
    }

    pub fn is_zero_approx(&self) -> bool {
        self.length_squared() <= f64::EPSILON
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}