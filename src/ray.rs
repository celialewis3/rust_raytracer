use crate::vec3::Vec3;
use std::ops::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {

    // Takes in two vecs, and creates a Ray
    pub fn ray(origin: Vec3, direction: Vec3) -> Ray {
        Ray { orig: origin, dir: direction }
    }

    pub fn origin(self) -> Vec3 {
        self.orig
    }

    pub fn direction(self) -> Vec3 {
        self.dir
    }

    pub fn point_at(self, t: f32) -> Vec3 {
        self.orig + self.dir * t
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_origin() {

    }

    #[test]
    fn test_ray_direction() {

    }
    
    #[test]
    fn test_ray_point_at_parameter() {

    }
}
