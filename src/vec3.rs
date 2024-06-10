use std::ops::{Neg, Index, IndexMut};

#[derive(Debug)]
pub struct Vec3 {
    pub e: [f64; 3]
}

impl Vec3 {
    pub fn new() -> Self {
        Vec3 {
            e: [0.0, 0.0, 0.0]
        }
    }

    pub fn create(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 {
            e: [e0, e1, e2]
        }
    }

    pub fn x(self) -> f64 {
        self.e[0]
    }

    pub fn y(self) -> f64 {
        self.e[1]
    }

    pub fn z(self) -> f64 {
        self.e[2]
    }

    pub fn length_squared(self) -> f64 {
        (self.e[0] * self.e[0]) + (self.e[1] * self.e[1]) + (self.e[2] * self.e[2])
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::create(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index<'a>(&'a self, i: usize) -> &'a f64 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f64 {
        &mut self.e[i]
    }
}

type Point3 = Vec3;