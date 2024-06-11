use crate::vec3::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3
}

impl Ray {
    pub fn new() -> Self {
        Ray {
            orig: Vec3::new(),
            dir: Vec3::new()
        }
    }

    pub fn create(origin: Point3, direction: Vec3) -> Self {
        Ray {
            orig: origin,
            dir: direction
        }
    }

    pub fn origin(self) -> Point3 {
        self.orig
    }

    pub fn direction(self) -> Vec3 {
        self.dir
    }

    pub fn at(self, s: f64) -> Point3 {
        self.orig + mul_scalar(&self.dir, &s)
    }
}