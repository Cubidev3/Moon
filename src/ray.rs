use crate::vector::Vector;

#[derive(Copy, Clone)]
pub struct Ray {
    pub starting_point: Vector,
    pub direction: Vector
}

impl Ray {
    pub fn from(starting_point: Vector, direction: Vector) -> Option<Ray> {
        match direction.normalized() {
            Some(direction) => Some(Ray { starting_point, direction }),
            None => None
        }
    }

    pub fn from_points(starting_point: Vector, end_point: Vector) -> Option<Ray> {
        let direction = end_point - starting_point;
        Ray::from(starting_point, direction)
    }

    pub fn point_at(&self, t: f64) -> Vector {
        self.starting_point + self.direction * t
    }
}