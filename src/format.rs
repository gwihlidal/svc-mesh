#![allow(dead_code)]

use crate::Vector2;
use crate::Vector3;
use crate::Vector4;

#[derive(Debug)]
pub enum AlphaMode {
    Opaque,
    Mask,
    Blend,
}

pub const MAX_BONE_INFLUENCES: usize = 16;

#[derive(Debug, Default)]
pub struct SkinningData {
    pub bone_count: u32,
    pub weights: [f32; MAX_BONE_INFLUENCES],
    pub bone_ids: [u32; MAX_BONE_INFLUENCES],
}

#[derive(Debug, Default)]
pub struct MeshData {
    pub skinning_data: Vec<SkinningData>,
    pub positions: Vec<Vector3>,
    pub tex_coords: Vec<Vector2>,
    pub normals: Vec<Vector3>,
    pub tangents: Vec<Vector3>,
    pub bitangents: Vec<Vector3>,
    pub colors: Vec<Vector4>,
    pub indices: Vec<u32>,
}

#[derive(Debug, Default)]
pub struct MeshAsset {}
