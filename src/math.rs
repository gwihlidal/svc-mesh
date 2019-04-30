use std::f32;

use cgmath;
pub use cgmath::prelude::*;
pub use cgmath::{vec3, vec4};

use collision;
//use num_traits::clamp;

pub type Vector2 = nalgebra::Vector2<f32>;
pub type Vector3 = nalgebra::Vector3<f32>;
pub type Vector4 = nalgebra::Vector4<f32>;

pub type Point3 = nalgebra::Point3<f32>;

pub type Matrix4 = nalgebra::Matrix4<f32>;
pub type Quaternion = nalgebra::Quaternion<f32>;
pub type UnitQuaternion = nalgebra::UnitQuaternion<f32>;

//pub type Aabb3 = collision::Aabb3<f32>;
