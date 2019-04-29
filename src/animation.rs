#![allow(unused_variables)]
#![allow(dead_code)]

use crate::GltfBuffers;
use crate::GltfData;
use crate::GltfIndex;
use crate::Matrix4;
use crate::StdError;
//use cgmath::SquareMatrix;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

#[derive(Debug)]
pub enum GltfInterpolationType {
    Linear,
    Step,
    CubicSpline,
}

#[derive(Debug)]
pub struct GltfJointNode {
    pub node_index: GltfIndex,
    pub global_index: i32, // -1
}

#[derive(Debug)]
pub struct GltfSkin {
    pub skin_index: GltfIndex,
    pub name: Option<String>,
    pub skeleton_root: Option<GltfIndex>,
    pub inv_bind_matrices: Vec<Matrix4>,
    pub joints: Vec<GltfJointNode>,
}

impl GltfSkin {
    pub fn from_gltf(
        skin_ref: &gltf::Skin<'_>,
        _data: &GltfData,
        _base_path: &Path,
    ) -> Rc<GltfSkin> {
        let name = if let Some(ref name) = skin_ref.name() {
            Some(name.to_owned().to_string())
        } else {
            None // Some(animation_ref.index().to_string())
        };

        let skeleton_root = if let Some(skeleton_root) = skin_ref.skeleton() {
            Some(skeleton_root.index())
        } else {
            None
        };

        Rc::new(GltfSkin {
            skin_index: skin_ref.index(),
            name,
            skeleton_root,
            inv_bind_matrices: Vec::new(),
            joints: Vec::new(),
        })
    }
}

/*
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
*/

#[derive(Debug)]
pub struct GltfAnimationSampler {}

impl GltfAnimationSampler {
    pub fn from_gltf(animation_ref: &gltf::animation::Sampler<'_>) -> GltfAnimationSampler {
        GltfAnimationSampler {}
    }
}

#[derive(Debug)]
pub struct GltfAnimationChannel {}

impl GltfAnimationChannel {
    pub fn from_gltf(animation_ref: &gltf::animation::Channel<'_>) -> GltfAnimationChannel {
        GltfAnimationChannel {}
    }
}

#[derive(Debug)]
pub struct GltfAnimation {
    pub index: GltfIndex,
    pub name: Option<String>,
    pub samplers: Vec<GltfAnimationSampler>,
    pub channels: Vec<GltfAnimationChannel>,
    pub node_to_channel: HashMap</*GltfNode*/ GltfIndex, usize>, // Map node index to animation channel
    pub start: f32,
    pub end: f32,
}

impl GltfAnimation {
    pub fn from_gltf(
        animation_ref: &gltf::Animation<'_>,
        data: &GltfData,
        _base_path: &Path,
    ) -> Rc<GltfAnimation> {
        use std::f32;

        let _buffers = &data.buffers;

        let animation_name = if let Some(ref name) = animation_ref.name() {
            Some(name.to_owned().to_string())
        } else {
            None // Some(animation_ref.index().to_string())
        };

        let mut start = f32::MAX;
        let mut end = f32::MIN;

        let samplers = animation_ref
            .samplers()
            .map(|sampler_ref| GltfAnimationSampler::from_gltf(&sampler_ref))
            .collect();

        let channels = animation_ref
            .channels()
            .map(|channel_ref| GltfAnimationChannel::from_gltf(&channel_ref))
            .collect();

        let node_to_channel = HashMap::new();

        Rc::new(GltfAnimation {
            index: animation_ref.index(),
            name: animation_name,
            samplers,
            channels,
            node_to_channel,
            start,
            end,
        })
    }
}
