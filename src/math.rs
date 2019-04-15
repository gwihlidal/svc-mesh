use std::f32;

use cgmath;
pub use cgmath::prelude::*;
pub use cgmath::{vec3, vec4};

use collision;
//use num_traits::clamp;

pub type Vector2 = cgmath::Vector2<f32>;
pub type Vector3 = cgmath::Vector3<f32>;
pub type Vector4 = cgmath::Vector4<f32>;

pub type Point3 = cgmath::Point3<f32>;

pub type Matrix4 = cgmath::Matrix4<f32>;
pub type Quaternion = cgmath::Quaternion<f32>;

pub type Aabb3 = collision::Aabb3<f32>;
