use std::ops;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vec3 {
    e: [f32;3],
}

impl Vec3 {

    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2]
        }
    }

    pub fn length_squared(self) -> f32 {
        self.e[0].powf(2.0) + self.e[1].powf(2.0)
        + self.e[2].powf(2.0)
    }

    pub fn length(self) -> f32 {
        Vec3::length_squared(self).sqrt()
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v / v.length()
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, _rhs: f32) -> Self::Output {

        let t: f32 = 1.0/_rhs;

        Vec3 {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t]
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    // where _rhs is the right hand Vector3 to add to self
    fn add(self, _rhs: Vec3) -> Self::Output {

        Vec3 { e: [self.e[0] + _rhs.e[0],
                   self.e[1] + _rhs.e[1],
                   self.e[2] + _rhs.e[2]],
             }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_add() {
        assert_eq!(Vec3::new(2.0,4.0,6.0) + Vec3::new(1.0, 1.0, 2.0), Vec3::new(3.0, 5.0, 8.0))
    }

    #[test]
    fn test_vec_divide() {
        assert_eq!(Vec3::new(8.0,4.0,2.0)/2.0, Vec3::new(4.0,2.0,1.0))
    }
    
    #[test]
    fn test_ray_point_at_parameter() {

    }
}
