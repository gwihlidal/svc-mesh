#![allow(unused_variables)]
#![allow(dead_code)]

//use crate::GltfBuffers;
use crate::GltfData;
use crate::GltfIndex;
use crate::GltfModel;
use crate::GltfNodeRef;
use crate::Matrix4;
use crate::Vector4;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

#[derive(Debug)]
pub enum GltfInterpolationType {
    Linear,
    Step,
    CatmullRomSpline,
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
pub struct GltfAnimationSampler {
    pub interpolation_type: GltfInterpolationType,
    pub inputs: Vec<f32>,
    pub outputs: Vec<Vector4>,
}

impl GltfAnimationSampler {
    pub fn from_gltf(
        sampler_ref: &gltf::animation::Sampler<'_>,
        data: &GltfData,
    ) -> GltfAnimationSampler {
        use gltf::animation::Interpolation;
        let interpolation = match sampler_ref.interpolation() {
            Interpolation::Linear => GltfInterpolationType::Linear,
            Interpolation::Step => GltfInterpolationType::Step,
            Interpolation::CatmullRomSpline => GltfInterpolationType::CatmullRomSpline,
            Interpolation::CubicSpline => GltfInterpolationType::CubicSpline,
        };

        // Read sampler input time values
        let input = sampler_ref.input();
        assert_eq!(input.data_type(), gltf::accessor::DataType::F32);
        let buffer_view = input.view();
        let buffer = buffer_view.buffer();

        let inputs: Vec<f32> = match (input.data_type(), input.dimensions()) {
            (gltf::accessor::DataType::F32, gltf::accessor::Dimensions::Scalar) => {
                let buffer_index = input.view().buffer().index();
                let buffer_data = data.buffers[buffer_index].0.as_slice();
                let iter = gltf::accessor::Iter::<f32>::new(input, buffer_data);
                iter.collect()
            }
            _ => unimplemented!(),
        };

        // Read sampler output T/R/S values
        let output = sampler_ref.output();
        let buffer_view = output.view();
        let buffer = buffer_view.buffer();

        let outputs: Vec<Vector4> = match (output.data_type(), output.dimensions()) {
            (gltf::accessor::DataType::F32, gltf::accessor::Dimensions::Scalar) => {
                let buffer_index = output.view().buffer().index();
                let buffer_data = data.buffers[buffer_index].0.as_slice();
                let iter = gltf::accessor::Iter::<f32>::new(output, buffer_data);
                iter.map(|x| Vector4::new(x, x, x, x)).collect()
            }
            (gltf::accessor::DataType::F32, gltf::accessor::Dimensions::Vec3) => {
                let buffer_index = output.view().buffer().index();
                let buffer_data = data.buffers[buffer_index].0.as_slice();
                let iter = gltf::accessor::Iter::<[f32; 3]>::new(output, buffer_data);
                iter.map(|[x, y, z]| Vector4::new(x, y, z, 0.0)).collect()
            }
            (gltf::accessor::DataType::F32, gltf::accessor::Dimensions::Vec4) => {
                let buffer_index = output.view().buffer().index();
                let buffer_data = data.buffers[buffer_index].0.as_slice();
                let iter = gltf::accessor::Iter::<[f32; 4]>::new(output, buffer_data);
                iter.map(|[x, y, z, w]| Vector4::new(x, y, z, w)).collect()
            }
            _ => unimplemented!(),
        };

        GltfAnimationSampler {
            interpolation_type: interpolation,
            inputs,
            outputs,
        }
    }
}

#[derive(Debug)]
pub struct GltfAnimationChannel {
    pub node_ref: GltfNodeRef,
    pub rotation_sampler: Option<GltfAnimationSampler>,
    pub translation_sampler: Option<GltfAnimationSampler>,
    pub scale_sampler: Option<GltfAnimationSampler>,
}

// impl GltfAnimationChannel {
//     pub fn from_gltf(channel_ref: &gltf::animation::Channel<'_>, animation_ref: &gltf::Animation<'_>, model: &GltfModel) -> Option<GltfAnimationChannel> {
//     }
// }

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
        model: &GltfModel,
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

        let samplers: Vec<GltfAnimationSampler> = animation_ref
            .samplers()
            .map(|sampler_ref| GltfAnimationSampler::from_gltf(&sampler_ref, data))
            .collect();

        for sampler in &samplers {
            for input in &sampler.inputs {
                start = start.min(*input);
                end = end.max(*input);
            }
        }

        // for sampler in &samplers {
        //     println!("Samplers {:?}",sampler);
        // }

        let mut channels: Vec<GltfAnimationChannel> = Vec::new();

        // What we want to do here is create channels, and let them own the sampler data associated with the channel.
        // Additionally, in our representation a single channel is associated with each node, with T, R and S transformations all owned by the same channel
        for channel_ref in animation_ref.channels() {
            // Find the node associated with the channel index
            let node_ref = model.node_from_index(channel_ref.target().node().index());
            match node_ref {
                Some(x) => {
                    // Find existing channel associated with the node, if it doesn't exist, create a new channel
                    let target_channel: &mut GltfAnimationChannel = match channels
                        .iter_mut()
                        .find(|channel_i| GltfNodeRef::ptr_eq(&channel_i.node_ref, &x))
                    {
                        Some(y) => y,
                        None => {
                            let new_channel = GltfAnimationChannel {
                                node_ref: x,
                                rotation_sampler: None,
                                translation_sampler: None,
                                scale_sampler: None,
                            };
                            channels.push(new_channel);
                            channels.last_mut().unwrap()
                        }
                    };
                    match channel_ref.target().property() {
                        gltf::animation::Property::Translation => {
                            target_channel.translation_sampler = Some(
                                GltfAnimationSampler::from_gltf(&channel_ref.sampler(), data),
                            )
                        }
                        gltf::animation::Property::Rotation => {
                            target_channel.rotation_sampler = Some(GltfAnimationSampler::from_gltf(
                                &channel_ref.sampler(),
                                data,
                            ))
                        }
                        gltf::animation::Property::Scale => {
                            target_channel.scale_sampler = Some(GltfAnimationSampler::from_gltf(
                                &channel_ref.sampler(),
                                data,
                            ))
                        }
                        _ => println!("Unimplemented: Found morph target channel"),
                    }
                }
                None => {
                    println!("No node found to match with channel.");
                }
            }
        }

        let mut node_to_channel = HashMap::new();

        for (i, channel) in channels.iter().enumerate() {
            // println!("Channel with node: {}",channel.node_ref.borrow().name.clone().unwrap());
            node_to_channel.insert(channel.node_ref.borrow().node_index, i);
        }

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
