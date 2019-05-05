use super::math::*;
use super::GltfData;
use super::GltfIndex;
//use super::GltfMaterial;
use super::GltfModel;
use log::{debug, warn};
use std::path::Path;
//use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vector3,
    pub normal: Vector3,
    pub tangent: Vector4,
    pub tex_coord_0: Vector2,
    pub color_0: Vector4,

    pub joints_0: [u16; 4],
    pub joints_1: [u16; 4],
    pub joints_2: [u16; 4],
    pub joints_3: [u16; 4],
    pub weights_0: Vector4,
    pub weights_1: Vector4,
    pub weights_2: Vector4,
    pub weights_3: Vector4,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            tangent: Vector4::new(0.0, 0.0, 0.0, 0.0),
            tex_coord_0: Vector2::new(0.0, 0.0),
            color_0: Vector4::new(0.0, 0.0, 0.0, 0.0),
            joints_0: [0; 4],
            joints_1: [0; 4],
            joints_2: [0; 4],
            joints_3: [0; 4],
            weights_0: Vector4::new(0.0, 0.0, 0.0, 0.0),
            weights_1: Vector4::new(0.0, 0.0, 0.0, 0.0),
            weights_2: Vector4::new(0.0, 0.0, 0.0, 0.0),
            weights_3: Vector4::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}

#[derive(Debug)]
pub struct Dimensions {
    pub min: Vector3,
    pub max: Vector3,
    pub size: Vector3,
    pub center: Vector3,
    pub radius: f32,
}

impl Default for Dimensions {
    fn default() -> Dimensions {
        use std::f32;
        Dimensions {
            min: Vector3::new(f32::MAX, f32::MAX, f32::MAX),
            max: Vector3::new(f32::MIN, f32::MIN, f32::MIN),
            size: Vector3::new(0.0, 0.0, 0.0),
            center: Vector3::new(0.0, 0.0, 0.0),
            radius: 0.0,
        }
    }
}

impl Dimensions {
    pub fn new(min: Vector3, max: Vector3) -> Dimensions {
        let distance = (max - min).norm();
        Dimensions {
            min,
            max,
            size: max - min,
            center: (min + max) / 2.0,
            radius: distance / 2.0,
        }
    }
}

#[derive(Debug)]
pub struct GltfPrimitive {
    pub mode: gltf::mesh::Mode,
    pub material: Option<GltfIndex>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub dimensions: Dimensions,
}

impl GltfPrimitive {
    pub fn new(
        mode: gltf::mesh::Mode,
        dimensions: Dimensions,
        vertices: &[Vertex],
        indices: Option<Vec<u32>>,
        material: Option<GltfIndex>,
    ) -> GltfPrimitive {
        //let index_count = indices.as_ref().map(|i| i.len()).unwrap_or(0);
        let indices = if let Some(ref indices) = indices {
            indices.to_owned()
        } else {
            Vec::new()
        };
        GltfPrimitive {
            mode,
            material,
            vertices: vertices.to_vec(),
            indices,
            dimensions,
        }
    }

    pub fn from_gltf(
        primitive_ref: &gltf::Primitive<'_>,
        primitive_index: usize,
        mesh_index: usize,
        _model: &mut GltfModel,
        data: &GltfData,
        _path: &Path,
    ) -> GltfPrimitive {
        use std::f32;

        let buffers = &data.buffers;
        let reader = primitive_ref.reader(|buffer| Some(&buffers[buffer.index()]));

        let faces = reader
            .read_indices()
            .map(|indices| indices.into_u32())
            .map(|mut indices| {
                let mut faces = vec![];
                while let (Some(a), Some(b), Some(c)) =
                    (indices.next(), indices.next(), indices.next())
                {
                    faces.push(a as usize);
                    faces.push(b as usize);
                    faces.push(c as usize);
                }
                faces
            });

        /*let positions = reader
            .read_positions()
            .map(|positions| match faces {
                Some(ref faces) => {
                    let vertices = positions.collect::<Vec<_>>();
                    faces.iter().map(|i| vertices[*i]).collect::<Vec<_>>()
                }
                None => positions.collect(),
            })
            //.ok_or(error::Error::MissingPositions)?;*/
        let positions = {
            let iter = reader.read_positions().unwrap_or_else(|| {
                panic!(
                    "primitives must have the POSITION attribute (mesh: {}, primitive: {})",
                    mesh_index, primitive_index
                )
            });
            iter.collect::<Vec<_>>()
        };

        //let bounds = primitive_ref.bounding_box();
        /*let bounds = Aabb3 {
            min: bounds.min.into(),
            max: bounds.max.into(),
        };*/

        //println!("Bounds: {:?}", bounds);

        let mut pos_min: Vector3 = Vector3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut pos_max: Vector3 = Vector3::new(f32::MIN, f32::MIN, f32::MIN);

        let mut vertices: Vec<Vertex> = positions
            .into_iter()
            .map(|position| {
                let v = Vertex {
                    position: Vector3::from(position),
                    ..Vertex::default()
                };
                pos_min = Vector3::new(
                    pos_min.x.min(v.position.x),
                    pos_min.y.min(v.position.y),
                    pos_min.z.min(v.position.z),
                );
                pos_max = Vector3::new(
                    pos_max.x.max(v.position.x),
                    pos_max.y.max(v.position.y),
                    pos_max.z.max(v.position.z),
                );
                v
            })
            .collect();

        //println!("Vertex Count: {}", vertices.len());

        //println!("Min: {:?}, Max: {:?}", pos_min, pos_max);

        //let mut shader_flags = ShaderFlags::empty();

        // normals
        if let Some(normals) = reader.read_normals() {
            for (i, normal) in normals.enumerate() {
                vertices[i].normal = Vector3::from(normal);
            }
        //shader_flags |= ShaderFlags::HAS_NORMALS;
        } else {
            debug!(
                "Found no NORMALs for primitive {} of mesh {} \
                 (flat normal calculation not implemented yet)",
                primitive_index, mesh_index
            );
        }

        // texture coordinates
        if let Some(tex_coords) = reader.read_tex_coords(0) {
            for (i, tex_coord) in tex_coords.into_f32().enumerate() {
                vertices[i].tex_coord_0 = Vector2::from(tex_coord);
            }
            //shader_flags |= ShaderFlags::HAS_UV;
        }
        if reader.read_tex_coords(1).is_some() {
            warn!("Ignoring further tex coord attributes, only supporting TEXCOORD_0. (mesh: {}, primitive: {})",
                mesh_index, primitive_index);
        }

        // colors
        if let Some(colors) = reader.read_colors(0) {
            let colors = colors.into_rgba_f32();
            for (i, c) in colors.enumerate() {
                vertices[i].color_0 = c.into();
            }
            //shader_flags |= ShaderFlags::HAS_COLORS;
        }
        if reader.read_colors(1).is_some() {
            warn!("Ignoring further color attributes, only supporting COLOR_0. (mesh: {}, primitive: {})",
                mesh_index, primitive_index);
        }

        // tangents
        /*if let Some(tangents) = reader.read_tangents() {
            for (i, tangent) in tangents.enumerate() {
                vertices[i].tangent = Vector4::from(tangent);
            }
        //shader_flags |= ShaderFlags::HAS_TANGENTS;
        } else {
            debug!(
                "Found no TANGENTS for primitive {} of mesh {} \
                 (tangent calculation not implemented yet)",
                primitive_index, mesh_index
            );
        }*/
        /*let tangents = reader
            .read_tangents()
            .map(|tangents| match faces {
                Some(ref faces) => {
                    let tangents = tangents.collect::<Vec<_>>();
                    faces
                        .iter()
                        .map(|i| [tangents[*i][0], tangents[*i][1], tangents[*i][2]])
                        .collect()
                }
                None => tangents.map(|t| [t[0], t[1], t[2]]).collect(),
            })
            .unwrap_or_else(|| calculate_tangents(&positions, &normals, &tex_coord));*/

        /*fn iterate_slices_counted_2<T>(xs: &[T], ys: &[T]) {
            let len = cmp::min(xs.len(), ys.len());
            let xs = &xs[..len];
            let ys = &ys[..len];
            for i in 0..len {
                let x = &xs[i];
                let y = &ys[i];
            }
        }*/

        let mut joint_set = 0;
        while let Some(joints) = reader.read_joints(joint_set) {
            if joint_set > 3 {
                warn!(
                    "Ignoring joint set {}, \
                     only supporting 4 joints at the moment. (mesh: {}, primitive: {})",
                    joint_set, mesh_index, primitive_index
                );
                joint_set += 1;
                continue;
            }
            for (i, joint) in joints.into_u16().enumerate() {
                match joint_set {
                    0 => vertices[i].joints_0 = joint,
                    1 => vertices[i].joints_1 = joint,
                    2 => vertices[i].joints_2 = joint,
                    3 => vertices[i].joints_3 = joint,
                    _ => unreachable!(),
                }
            }
            //shader_flags |= ShaderFlags::HAS_UV;
            joint_set += 1;
        }

        let mut weight_set = 0;
        while let Some(weights) = reader.read_weights(weight_set) {
            if weight_set > 3 {
                warn!(
                    "Ignoring weight set {}, \
                     only supporting 4 weights at the moment. (mesh: {}, primitive: {})",
                    weight_set, mesh_index, primitive_index
                );
                weight_set += 1;
                continue;
            }
            for (i, weights) in weights.into_f32().enumerate() {
                match weight_set {
                    0 => vertices[i].weights_0 = weights.into(),
                    1 => vertices[i].weights_1 = weights.into(),
                    2 => vertices[i].weights_2 = weights.into(),
                    3 => vertices[i].weights_3 = weights.into(),
                    _ => unreachable!(),
                }
            }
            //shader_flags |= ShaderFlags::HAS_UV;
            weight_set += 1;
        }

        let indices = reader
            .read_indices()
            .map(|read_indices| read_indices.into_u32().collect::<Vec<_>>());

        let mode = primitive_ref.mode();
        match mode {
            gltf::mesh::Mode::Points => {
                unimplemented!();
            }
            gltf::mesh::Mode::Lines => {
                unimplemented!();
            }
            gltf::mesh::Mode::LineLoop => {
                unimplemented!();
            }
            gltf::mesh::Mode::LineStrip => {
                unimplemented!();
            }
            gltf::mesh::Mode::Triangles => {}
            gltf::mesh::Mode::TriangleStrip => {
                unimplemented!();
            }
            gltf::mesh::Mode::TriangleFan => {
                unimplemented!();
            }
        }

        //shader_flags |= material.shader_flags();

        GltfPrimitive::new(
            mode,
            Dimensions::new(pos_min, pos_max),
            &vertices,
            indices,
            primitive_ref.material().index(),
        )
    }
}
