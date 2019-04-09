use crate::GltfIndex;
use crate::Matrix4;

pub struct GltfJointNode
{
    pub node_index: GltfIndex,
    pub global_index: i32, // -1
}

pub struct GltfSkin
{
    pub name: String,
    pub skeleton_root: GltfIndex,
    pub inv_bind_matrices: Vec<Matrix4>,
    pub joints: Vec<GltfJointNode>,
}