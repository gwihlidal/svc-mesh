use crate::GltfBuffers;
use crate::GltfIndex;
use crate::Matrix4;
use crate::StdError;
use cgmath::SquareMatrix;
use std::collections::HashMap;

pub enum GltfInterpolationType {
    Linear,
    Step,
    CubicSpline,
}

pub struct GltfJointNode {
    pub node_index: GltfIndex,
    pub global_index: i32, // -1
}

pub struct GltfSkin {
    pub name: String,
    pub skeleton_root: GltfIndex,
    pub inv_bind_matrices: Vec<Matrix4>,
    pub joints: Vec<GltfJointNode>,
}

pub fn load_skin(
    skin: &gltf::Skin<'_>,
    buffers: &GltfBuffers,
    skin_entity: usize,
    node_map: &HashMap<usize, usize>,
    meshes: Vec<usize>,
    //    prefab: &mut Prefab<GltfPrefab>,
) -> Result<(), Box<StdError>> {
    let joints = skin
        .joints()
        .map(|j| {
            node_map.get(&j.index()).cloned().expect(
                "Unreachable: `node_map` is initialized with the indexes from the `Gltf` object",
            )
        })
        .collect::<Vec<_>>();

    let _reader = skin.reader(|buffer| buffers.buffer(&buffer));

    /*let inverse_bind_matrices = reader
    .read_inverse_bind_matrices()
    .map(|matrices| matrices.map(|m| m.into()).collect())
    .unwrap_or(vec![Matrix4::identity().into(); joints.len()]);*/

    for (_bind_index, _joint_index) in joints.iter().enumerate() {
        /*prefab
        .data_or_default(*joint_index)
        .skinnable
        .get_or_insert_with(SkinnablePrefab::default)
        .joint
        .get_or_insert_with(JointPrefab::default)
        .skins
        .push(skin_entity);*/
    }
    /*let joint_transforms = JointTransformsPrefab {
        skin: skin_entity,
        size: joints.len(),
    };*/
    for mesh_index in &meshes {
        /*prefab
        .data_or_default(*mesh_index)
        .skinnable
        .get_or_insert_with(SkinnablePrefab::default)
        .joint_transforms = Some(joint_transforms.clone());*/
    }

    /*let skin_prefab = SkinPrefab {
        joints,
        meshes,
        bind_shape_matrix: Matrix4::identity(),
        inverse_bind_matrices,
    };
    prefab
        .data_or_default(skin_entity)
        .skinnable
        .get_or_insert_with(SkinnablePrefab::default)
        .skin = Some(skin_prefab);*/

    Ok(())
}
