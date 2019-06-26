#![allow(dead_code)]

use crate::Matrix4;
use crate::Vector2;
use crate::Vector3;
use crate::Vector4;

#[derive(Debug, Clone)]
pub enum AlphaMode {
    Opaque,
    Mask,
    Blend,
}

#[derive(Debug, Clone)]
pub enum AnimationType {
    None,
    Skinned,
    Rigid,
    Mesh,
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
    pub positions: Vec<[f32; 3]>,
    pub tex_coords: Vec<Vector2>,
    pub normals: Vec<Vector3>,
    pub tangents: Vec<Vector3>,
    pub bitangents: Vec<Vector3>,
    pub colors: Vec<Vector4>,
    pub indices: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct MeshAssetPart {
    pub index_start: u32,
    pub index_count: u32,
    pub material_index: Option<u32>,
    pub node_index: Option<u32>, // index into the Animation BoneNodes array
    pub base_transform: Matrix4,
    pub name: Option<String>, // todo: need this to pair with animation channel;
    pub animation_type: AnimationType,
}

#[derive(Debug, Default)]
pub struct MeshAsset {}
