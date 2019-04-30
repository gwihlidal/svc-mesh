#![allow(unused_variables)]
#![allow(dead_code)]

use crate::GltfBuffers;
use crate::GltfData;
use crate::GltfIndex;
use crate::Matrix4;
use crate::StdError;
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
    pub global_index: Option<GltfIndex>,
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
        data: &GltfData,
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

        // Find joint nodes
        let joints: Vec<_> = skin_ref
            .joints()
            .map(|joint_ref| GltfJointNode {
                node_index: joint_ref.index(),
                global_index: None,
            })
            .collect();

        let reader = skin_ref.reader(|buffer| data.buffer(&buffer));

        let inv_bind_matrices = reader
            .read_inverse_bind_matrices()
            .map(|matrices| matrices.map(|m| m.into()).collect())
            .unwrap_or(vec![Matrix4::identity().into(); joints.len()]);

        // Get inverse bind matrices from buffer
        /*let inv_bind_matrices = if let Some(accessor) = skin_ref.inverse_bind_matrices() {
            assert_eq!(accessor.count(), joints.len());
            match (accessor.data_type(), accessor.dimensions()) {
                //Mat4,
                (gltf::accessor::DataType::F32, gltf::accessor::Dimensions::Mat4) => {
                    let buffer_view = accessor.view();
                    let buffer_index = buffer_view.buffer().index();
                    let buffer_offset = accessor.offset() + buffer_view.offset();
                    let buffer_data = data.buffers[buffer_index].0.as_slice();
                    let buffer_data = &buffer_data[buffer_offset..(buffer_offset + 4)];

                    //let iter = gltf::accessor::Iter::<Matrix4<f32>>::new(accessor, buffer_data);
                    //for item in iter {
                    //    println!("{:?}", item);
                    //}
                }
                _ => {
                    println!(
                        "Invalid format for inv bind matrices! data type: {:?}, dimensions: {:?}",
                        accessor.data_type(),
                        accessor.dimensions()
                    );
                    unimplemented!();
                }
            }

            Vec::new()
        } else {
            Vec::new()
            //vec![Matrix4::identity().into(); joints.len()]
        };*/

        Rc::new(GltfSkin {
            skin_index: skin_ref.index(),
            name,
            skeleton_root,
            inv_bind_matrices,
            joints,
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
        // TODO:

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
