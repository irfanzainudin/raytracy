use std::{io::Write, ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub}};

#[derive(Debug, Clone, Copy)]
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

trait MulScalar {
    type Output;
    
    fn mul_scalar(v: &Vec3, s: f64) -> Self::Output;
}

trait MulAssignScalar {
    fn mul_assign_scalar(&mut self, s: f64);
}

trait DivAssignScalar {
    fn div_assign_scalar(&mut self, s: f64);
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::create(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]]
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl Sub for Vec3 {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2]]
        }
    }
}

// impl SubAssign for Vec3 {
//     fn sub_assign(&mut self, rhs: Self) {
//         self.e[0] -= rhs.e[0];
//         self.e[1] -= rhs.e[1];
//         self.e[2] -= rhs.e[2];
//     }
// }

impl Mul for Vec3 {
    type Output = Self;
    
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2]]
        }
    }
}

impl MulScalar for Vec3 {
    type Output = Self;

    fn mul_scalar(v: &Vec3, s: f64) -> Self::Output {
        Vec3 {
            e: [s * v.e[0], s * v.e[1], s * v.e[2]]
        }
    }
}

// impl MulAssign for Vec3 {
    // fn mul_assign(&mut self, rhs: Self) {
    //     self.e[0] *= rhs.e[0];
    //     self.e[1] *= rhs.e[1];
    //     self.e[2] *= rhs.e[2];
    // }
    // fn mul_assign(&mut self, s: f64) {
    //     self.e[0] *= s;
    //     self.e[1] *= s;
    //     self.e[2] *= s;
    // }
// }

impl MulAssignScalar for Vec3 {
    fn mul_assign_scalar(&mut self, s: f64) {
        self.e[0] *= s;
        self.e[1] *= s;
        self.e[2] *= s;
    }
}

impl Div for Vec3 {
    type Output = Self;
    
    fn div(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [self.e[0] / rhs.e[0], self.e[1] / rhs.e[1], self.e[2] / rhs.e[2]]
        }
    }
}

// impl DivAssign for Vec3 {
//     fn div_assign(&mut self, s: f64) {
//         self.e[0] /= s;
//         self.e[1] /= s;
//         self.e[2] /= s;
//     }
// }

impl DivAssignScalar for Vec3 {
    fn div_assign_scalar(&mut self, s: f64) {
        self.e[0] *= (1.0 / s);
        self.e[1] *= (1.0 / s);
        self.e[2] *= (1.0 / s);
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

pub type Point3 = Vec3;

fn div_scalar(v: &Vec3, s: &f64) -> Vec3 {
    Vec3 {
        e: [(1.0 / s) * v.e[0], (1.0 / s) * v.e[1], (1.0 / s) * v.e[2]]
    }
}

fn dot(u: &Vec3, v: &Vec3) -> f64 {
    (u.e[0] * v.e[0]) + (u.e[1] * v.e[1]) + (u.e[2] * v.e[2])
}

fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        e: [
            (u.e[1] * v.e[2]) - (u.e[2] * v.e[1]),
            (u.e[2] * v.e[0]) - (u.e[0] * v.e[2]),
            (u.e[0] * v.e[1]) - (u.e[1] * v.e[0])
        ]
    }
}

fn unit_vector(v: &Vec3, v_length: &f64) -> Vec3 {
    div_scalar(v, v_length)
}

pub type Colour = Vec3;

// TODO: Change the function signature to this
// fn write_colour(out: Iterator, pixel_colour: Colour) {
pub fn write_colour(data_file: &mut std::fs::File, pixel_colour: &Colour) {
// pub fn write_colour(x: &f64, y: &f64, z: &f64) {
    // let r = x;
    // let g = y;
    // let b = z;
    let r = pixel_colour.x();
    let g = pixel_colour.y();
    let b = pixel_colour.z();

    // Translate the [0,1] component values to the byte range [0,255]
    let rbyte = (255.999 * r) as i64;
    let gbyte = (255.999 * g) as i64;
    let bbyte = (255.999 * b) as i64;

    // Write out the pixel colour components
    let pixel_data = format!("{} {} {}\n", rbyte, gbyte, bbyte);
    data_file.write(pixel_data.as_bytes());
}