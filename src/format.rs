#[derive(Debug)]
pub enum AlphaMode {
    Opaque,
    Mask,
    Blend,
}

pub struct SkinningData {

}

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
