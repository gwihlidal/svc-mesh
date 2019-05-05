use std::f32;

pub use nalgebra::Unit;

pub type Vector2 = nalgebra::Vector2<f32>;
pub type Vector3 = nalgebra::Vector3<f32>;
pub type Vector4 = nalgebra::Vector4<f32>;

//pub type Point3 = nalgebra::Point3<f32>;

pub type Matrix4 = nalgebra::Matrix4<f32>;
pub type Quaternion = nalgebra::Quaternion<f32>;
pub type UnitQuaternion = nalgebra::UnitQuaternion<f32>;

#[derive(Debug)]
pub struct Dimensions {
    pub min: Vector3,
    pub max: Vector3,
    pub size: Vector3,
    pub center: Vector3,
    pub radius: f32,
}

impl Default for Dimensions {
    fn default() -> Dimensions {
        Dimensions {
            min: Vector3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX),
            max: Vector3::new(std::f32::MIN, std::f32::MIN, std::f32::MIN),
            size: Vector3::new(0.0, 0.0, 0.0),
            center: Vector3::new(0.0, 0.0, 0.0),
            radius: 0.0,
        }
    }
}

impl Dimensions {
    pub fn new(min: Vector3, max: Vector3) -> Dimensions {
        let distance = (max - min).norm();
        Dimensions {
            min,
            max,
            size: max - min,
            center: (min + max) / 2.0,
            radius: distance / 2.0,
        }
    }
}
