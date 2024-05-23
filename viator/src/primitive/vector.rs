use std::{
    fmt,
    hash::{Hash, Hasher},
};

use super::alias::{TransformFloat, TransformInt};

#[derive(Debug, Clone, Copy)]
pub struct Vec3F {
    pub x: TransformFloat,
    pub y: TransformFloat,
    pub z: TransformFloat,
}

impl Vec3F {
    pub fn translate(&mut self, other: &Vec3F) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl PartialEq for Vec3F {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl fmt::Display for Vec3F {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec3F(x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3I {
    pub x: TransformInt,
    pub y: TransformInt,
    pub z: TransformInt,
}

impl Vec3I {
    pub fn translate(&mut self, other: &Vec3I) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z
    }
}

impl PartialEq for Vec3I {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Vec3I {}

impl Hash for Vec3I {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl fmt::Display for Vec3I {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec3I(x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}
