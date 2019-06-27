//use gltf::{buffer::Source as BufferSource, image::Source as ImageSource, Gltf};
use std::collections::HashMap;
use std::path::Path;
//use std::rc::Rc;

mod animation;
mod data;
mod error;
mod format;
mod generated;
mod material;
mod math;
mod mesh;
mod model;
mod node;
mod primitive;
mod scene;
mod tangents;
mod texture;

use animation::*;
use data::*;
use error::*;
use format::*;
use generated::service::mesh::schema;
use material::*;
use math::*;
use mesh::*;
use model::*;
use node::*;
use primitive::*;
use scene::*;
use tangents::*;
use texture::*;

use math::Vector4;

#[derive(Debug, Clone, Default)]
pub struct GltfOptions {
    pub scene_index: Option<usize>,
    pub load_animations: bool,
    pub regenerate_tangents: bool,
    pub generate_tex_coords: (f32, f32),
    pub flip_v_coord: bool,
}

#[inline(always)]
pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}

fn load_model<'a>(
    mut builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    model_path: &Path,
) -> Result<flatbuffers::WIPOffset<schema::Mesh<'a>>> {
    let _base_path = model_path.parent().unwrap_or(Path::new("./"));

    let (document, buffers, images) = gltf::import(model_path)?;

    let options = GltfOptions {
        regenerate_tangents: true,
        ..Default::default()
    };

    let data = GltfData {
        options,
        document,
        buffers,
        images,
    };

    let mut model = GltfModel::from_gltf(&data, model_path)?;

    let scene_count = data.document.scenes().len();
    let scene_index = 0;
    if scene_index >= scene_count {
        //error!("Scene index too high - file has only {} scene(s)", scene_count);
        //process::exit(3)
        panic!("scene index is too high");
    }
    //println!("Scene count: {}", scene_count);

    let gltf_scene = data.document.scenes().nth(scene_index).unwrap();
    let scene = GltfScene::from_gltf(&gltf_scene, &mut model)?;
    println!("Scene Dimensions: {:?}", scene.dimensions);

    let has_animations = model.animations.len() > 0;
    let flatten_transforms = !has_animations;

    // Protect against shared triangle vertices being transformed multiple times
    let mut transformed: Vec<bool> = if flatten_transforms {
        vec![false; model.vertex_buffer.len()]
    } else {
        Vec::new()
    };

    // Map from name of node to indices of all parts owned by that node in the global parts array
    let mut part_map: HashMap<String, Vec<u32>> = HashMap::new();

    let global_scale = 1.0;
    let scale = Matrix4::identity();
    scale.prepend_nonuniform_scaling(&Vector3::new(global_scale, global_scale, global_scale));

    let mut parts: Vec<MeshAssetPart> = Vec::with_capacity(model.linear_nodes.len());
    for linear_node in &model.linear_nodes {
        let linear_node = linear_node.borrow();
        if let Some(ref mesh) = linear_node.mesh {
            let matrix = scale * linear_node.get_matrix();

            let mut part = MeshAssetPart {
                index_start: 0,
                index_count: 0,
                material_index: None,
                node_index: None,
                base_transform: Matrix4::identity(),
                animation_type: AnimationType::None,
                name: linear_node.name.clone(),
            };

            let mut part_indices: Vec<u32> = Vec::new();
            for primitive in &mesh.primitives {
                part.index_start = primitive.index_start;
                part.index_count = primitive.index_count;
                part.material_index = if let Some(index) = primitive.material_index {
                    Some(index as u32)
                } else {
                    None
                };
                part.animation_type = if has_animations {
                    if linear_node.skin_index.is_some() {
                        AnimationType::Skinned
                    } else {
                        AnimationType::Rigid
                    }
                } else {
                    AnimationType::None
                };

                if flatten_transforms {
                    for index in
                        (part.index_start..(part.index_start + part.index_count)).step_by(3)
                    {
                        let i0 = model.index_buffer[index as usize + 0] as usize;
                        let i1 = model.index_buffer[index as usize + 1] as usize;
                        let i2 = model.index_buffer[index as usize + 2] as usize;

                        if !transformed[i0] {
                            let pos = Vector4::new(
                                model.vertex_buffer[i0].position[0],
                                model.vertex_buffer[i0].position[1],
                                model.vertex_buffer[i0].position[2],
                                1.0,
                            );
                            let pos = matrix * pos;
                            model.vertex_buffer[i0].position = [pos.x, pos.y, pos.z];

                            let norm = Vector4::new(
                                model.vertex_buffer[i0].normal[0],
                                model.vertex_buffer[i0].normal[1],
                                model.vertex_buffer[i0].normal[2],
                                1.0,
                            );
                            let norm = matrix * norm;
                            let norm = norm.normalize();
                            model.vertex_buffer[i0].normal = [norm.x, norm.y, norm.z];

                            transformed[i0] = true;
                        }

                        if !transformed[i1] {
                            let pos = Vector4::new(
                                model.vertex_buffer[i1].position[0],
                                model.vertex_buffer[i1].position[1],
                                model.vertex_buffer[i1].position[2],
                                1.0,
                            );
                            let pos = matrix * pos;
                            model.vertex_buffer[i1].position = [pos.x, pos.y, pos.z];

                            let norm = Vector4::new(
                                model.vertex_buffer[i1].normal[0],
                                model.vertex_buffer[i1].normal[1],
                                model.vertex_buffer[i1].normal[2],
                                1.0,
                            );
                            let norm = matrix * norm;
                            let norm = norm.normalize();
                            model.vertex_buffer[i1].normal = [norm.x, norm.y, norm.z];

                            transformed[i1] = true;
                        }

                        if !transformed[i2] {
                            let pos = Vector4::new(
                                model.vertex_buffer[i2].position[0],
                                model.vertex_buffer[i2].position[1],
                                model.vertex_buffer[i2].position[2],
                                1.0,
                            );
                            let pos = matrix * pos;
                            model.vertex_buffer[i2].position = [pos.x, pos.y, pos.z];

                            let norm = Vector4::new(
                                model.vertex_buffer[i2].normal[0],
                                model.vertex_buffer[i2].normal[1],
                                model.vertex_buffer[i2].normal[2],
                                1.0,
                            );
                            let norm = matrix * norm;
                            let norm = norm.normalize();
                            model.vertex_buffer[i2].normal = [norm.x, norm.y, norm.z];

                            transformed[i2] = true;
                        }
                    }
                }

                parts.push(part.clone());
                part_indices.push((parts.len() - 1) as u32);
            }

            assert!(linear_node.name.is_some());
            let name = if let Some(ref name_ref) = linear_node.name {
                name_ref.clone()
            } else {
                "UNNAMED".to_string()
            };

            part_map.insert(name, part_indices);
        }
    }

    let mut mesh_data = MeshData::default();

    mesh_data.indices = model.index_buffer.clone();

    mesh_data.positions.reserve(model.vertex_buffer.len());
    mesh_data.tex_coords.reserve(model.vertex_buffer.len());
    mesh_data.normals.reserve(model.vertex_buffer.len());
    mesh_data.tangents.reserve(model.vertex_buffer.len());
    mesh_data.bitangents.reserve(model.vertex_buffer.len());
    mesh_data.colors.reserve(model.vertex_buffer.len());
    mesh_data.indices.reserve(model.index_buffer.len());
    if has_animations {
        mesh_data.skinning_data.reserve(model.vertex_buffer.len());
    }

    for i in 0..model.vertex_buffer.len() {
        let vertex = &model.vertex_buffer[i];
        mesh_data.positions.push(vertex.position);
        mesh_data.tex_coords.push(vertex.uv0);
        mesh_data.normals.push(vertex.normal);
        mesh_data.tangents.push(vertex.tangent);
        mesh_data.bitangents.push(vertex.bitangent);
        mesh_data.colors.push(vertex.color0);

        if has_animations {
            // TODO
            let mut skinning_data = SkinningData {
                bone_count: vertex.influence_count,
                weights: [0.0; MAX_BONE_INFLUENCES],
                bone_ids: [0; MAX_BONE_INFLUENCES],
            };

            let mut sum = 0.0;

            // Largest index with a non zero weight
            let mut max_non_zero = std::u32::MAX;

            for bone in 0..skinning_data.bone_count {
                if bone < 4 {
                    skinning_data.weights[bone as usize] = vertex.weight0[bone as usize];
                    skinning_data.bone_ids[bone as usize] = vertex.joint0[bone as usize] as u32;
                } else if bone < 8 {
                    skinning_data.weights[bone as usize] = vertex.weight1[bone as usize - 4];
                    skinning_data.bone_ids[bone as usize] = vertex.joint1[bone as usize - 4] as u32;
                } else if bone < 12 {
                    skinning_data.weights[bone as usize] = vertex.weight2[bone as usize - 8];
                    skinning_data.bone_ids[bone as usize] = vertex.joint2[bone as usize - 8] as u32;
                } else {
                    skinning_data.weights[bone as usize] = vertex.weight3[bone as usize - 12];
                    skinning_data.bone_ids[bone as usize] =
                        vertex.joint3[bone as usize - 12] as u32;
                }

                if skinning_data.weights[bone as usize] > 0.0 {
                    max_non_zero = bone;
                }

                sum += skinning_data.weights[bone as usize];
            }

            skinning_data.bone_count = max_non_zero + 1;

            // Re-balance the weights
            for bone in 0..skinning_data.bone_count {
                skinning_data.weights[bone as usize] /= sum;
            }

            mesh_data.skinning_data.push(skinning_data);
        }
    }

    if has_animations {
        // export the animations
        // TODO
    }

    if has_animations {
        // export bone offsets and per vertex skinning data
        // TODO
    }

    if mesh_data.positions.is_empty() || mesh_data.indices.is_empty() {
        dbg!(mesh_data.indices);
        return Err(Error::memory("no vertices found"));
    }

    // Setup streams
    let mut streams: Vec<_> = Vec::new();

    // Positions
    let position_data = mesh_data.positions.as_ptr() as *const u8;
    let position_data = unsafe {
        std::slice::from_raw_parts(
            position_data,
            mesh_data.positions.len() * std::mem::size_of::<[f32; 3]>(),
        )
    };
    let position_data = Some(builder.create_vector_direct(&position_data));
    let position_stream = schema::MeshStream::create(
        &mut builder,
        &schema::MeshStreamArgs {
            format: schema::StreamFormat::Vector3,
            type_: schema::StreamType::Positions,
            elements: mesh_data.positions.len() as u64,
            data: position_data,
        },
    );
    streams.push(position_stream);

    // Normals
    let normal_data = mesh_data.normals.as_ptr() as *const u8;
    let normal_data = unsafe {
        std::slice::from_raw_parts(
            normal_data,
            mesh_data.normals.len() * std::mem::size_of::<[f32; 3]>(),
        )
    };
    let normal_data = Some(builder.create_vector_direct(&normal_data));
    let normal_stream = schema::MeshStream::create(
        &mut builder,
        &schema::MeshStreamArgs {
            format: schema::StreamFormat::Vector3,
            type_: schema::StreamType::Normals,
            elements: mesh_data.normals.len() as u64,
            data: normal_data,
        },
    );
    streams.push(normal_stream);

    // Tangents
    let tangent_data = mesh_data.tangents.as_ptr() as *const u8;
    let tangent_data = unsafe {
        std::slice::from_raw_parts(
            tangent_data,
            mesh_data.tangents.len() * std::mem::size_of::<[f32; 3]>(),
        )
    };
    let tangent_data = Some(builder.create_vector_direct(&tangent_data));
    let tangent_stream = schema::MeshStream::create(
        &mut builder,
        &schema::MeshStreamArgs {
            format: schema::StreamFormat::Vector3,
            type_: schema::StreamType::Tangents,
            elements: mesh_data.tangents.len() as u64,
            data: tangent_data,
        },
    );
    streams.push(tangent_stream);

    // Bitangents
    let bitangent_data = mesh_data.bitangents.as_ptr() as *const u8;
    let bitangent_data = unsafe {
        std::slice::from_raw_parts(
            bitangent_data,
            mesh_data.bitangents.len() * std::mem::size_of::<[f32; 3]>(),
        )
    };
    let bitangent_data = Some(builder.create_vector_direct(&bitangent_data));
    let bitangent_stream = schema::MeshStream::create(
        &mut builder,
        &schema::MeshStreamArgs {
            format: schema::StreamFormat::Vector3,
            type_: schema::StreamType::Bitangents,
            elements: mesh_data.bitangents.len() as u64,
            data: bitangent_data,
        },
    );
    streams.push(bitangent_stream);

    // Texture Coords
    let tex_coord_data = mesh_data.tex_coords.as_ptr() as *const u8;
    let tex_coord_data = unsafe {
        std::slice::from_raw_parts(
            tex_coord_data,
            mesh_data.tex_coords.len() * std::mem::size_of::<[f32; 2]>(),
        )
    };
    let tex_coord_data = Some(builder.create_vector_direct(&tex_coord_data));
    let tex_coord_stream = schema::MeshStream::create(
        &mut builder,
        &schema::MeshStreamArgs {
            format: schema::StreamFormat::Vector2,
            type_: schema::StreamType::TextureCoordinates,
            elements: mesh_data.tex_coords.len() as u64,
            data: tex_coord_data,
        },
    );
    streams.push(tex_coord_stream);

    // Colors
    let color_data = mesh_data.colors.as_ptr() as *const u8;
    let color_data = unsafe {
        std::slice::from_raw_parts(
            color_data,
            mesh_data.colors.len() * std::mem::size_of::<[f32; 4]>(),
        )
    };
    let color_data = Some(builder.create_vector_direct(&color_data));
    let color_stream = schema::MeshStream::create(
        &mut builder,
        &schema::MeshStreamArgs {
            format: schema::StreamFormat::Vector4,
            type_: schema::StreamType::Colors,
            elements: mesh_data.colors.len() as u64,
            data: color_data,
        },
    );
    streams.push(color_stream);

    // Indices
    let index_data = mesh_data.indices.as_ptr() as *const u8;
    let index_data = unsafe {
        std::slice::from_raw_parts(
            index_data,
            mesh_data.indices.len() * std::mem::size_of::<u32>(),
        )
    };
    let index_data = Some(builder.create_vector_direct(&index_data));
    let index_stream = schema::MeshStream::create(
        &mut builder,
        &schema::MeshStreamArgs {
            format: schema::StreamFormat::Int,
            type_: schema::StreamType::Indices,
            elements: mesh_data.indices.len() as u64,
            data: index_data,
        },
    );
    streams.push(index_stream);

    let streams = Some(builder.create_vector(&streams));

    // Setup materials
    let mut materials: Vec<_> = Vec::new();
    for material in &model.materials {
        let name = if let Some(ref name) = material.name {
            name.to_owned()
        } else {
            "".to_string()
        };
        let name = Some(builder.create_string(&name));
        let uri = Some(builder.create_string(&material.material_uri));
        let albedo_tint = Some(builder.create_vector_direct(&[
            material.base_color_factor[0],
            material.base_color_factor[1],
            material.base_color_factor[2],
        ]));
        materials.push(schema::MeshMaterial::create(
            &mut builder,
            &schema::MeshMaterialArgs {
                name,
                material: uri,
                albedo_tint,
                roughness: material.roughness_factor,
            },
        ));
    }
    let materials = Some(builder.create_vector(&materials));

    // Setup parts
    //let parts: Vec<_> = Vec::new();
    //let parts = Some(builder.create_vector(&parts));

    // Calculate bounding box
    let bounding_min = Some(builder.create_vector_direct(&[
        model.dimensions.min[0],
        model.dimensions.min[1],
        model.dimensions.min[2],
    ]));
    let bounding_max = Some(builder.create_vector_direct(&[
        model.dimensions.max[0],
        model.dimensions.max[1],
        model.dimensions.max[2],
    ]));
    let name = Some(builder.create_string(&model_path.to_string_lossy()));
    let identity = "123456-ident";
    let identity = Some(builder.create_string(&identity));
    let mesh = schema::Mesh::create(
        &mut builder,
        &schema::MeshArgs {
            name,
            identity,
            streams,
            materials,
            animations: None,
            //parts,
            parts: None,
            skinning_data: None,
            bounding_min,
            bounding_max,
        },
    );

    Ok(mesh)
}

fn main() {
    let meshes = ["data/Combat_Helmet.glb", "data/Floor_Junk_Cluster_01.glb"];

    let mut manifest_builder = flatbuffers::FlatBufferBuilder::new();

    //list(&"data/Book_03.glb").expect("runtime error");
    //list(&"data/BoxAnimated.glb").expect("runtime error");
    //list(&"data/Combat_Helmet.glb").expect("runtime error");
    //list(&"data/EpicCitadel.glb").expect("runtime error");
    //list(&"data/Floor_Junk_Cluster_01.glb").expect("runtime error");
    //list(&"data/Machine_01.glb").expect("runtime error");
    //list(&"data/RiggedFigure.glb").expect("runtime error");
    //list(&"data/Warrok.glb").expect("runtime error");
    //load_model(&Path::new("data/SciFiHelmet.gltf")).expect("runtime error");
    //load_model(&Path::new("data/EpicCitadel.glb")).expect("runtime error");
    //load_model(&Path::new("data/BoxAnimated.glb")).expect("runtime error");
    //load_model(&Path::new("data/RiggedFigure.glb")).expect("runtime error");
    //display(&"data/RiggedFigure.glb").expect("runtime error");

    let mut manifest_meshes: Vec<_> = Vec::with_capacity(meshes.len());
    for mesh_name in &meshes {
        let mesh = load_model(&mut manifest_builder, &Path::new(mesh_name)).expect("runtime error");
        manifest_meshes.push(mesh);
    }

    let manifest_meshes = Some(manifest_builder.create_vector(&manifest_meshes));
    let manifest = schema::Manifest::create(
        &mut manifest_builder,
        &schema::ManifestArgs {
            meshes: manifest_meshes,
        },
    );

    manifest_builder.finish(manifest, None);
    let manifest_data = manifest_builder.finished_data();

    println!("Done - {} bytes", manifest_data.len());
}
