use crate::vector::Vector;

#[derive(Copy, Clone)]
pub struct Basis {
    pub u: Vector,
    pub v: Vector,
    pub w: Vector
}

impl Basis {
    pub fn from_wv(w: Vector, v: Vector) -> Option<Basis> {
        if w.is_zero_approx() || v.is_zero_approx() {
            return None;
        }

        let w = w.normalized_or_zero();
        let u = v.cross(w).normalized_or_zero();
        let v = w.cross(u);

        Some(Basis {
            u, v, w
        })
    }
}