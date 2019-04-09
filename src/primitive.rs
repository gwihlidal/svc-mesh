use super::math::*;
use super::GltfData;
use super::GltfMaterial;
use super::GltfModel;
use log::{debug, warn};
use std::path::Path;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vector3,
    pub normal: Vector3,
    pub tangent: Vector4,
    pub tex_coord_0: Vector2,
    pub tex_coord_1: Vector2,
    pub color_0: Vector4,
    pub joints_0: [u16; 4],
    pub weights_0: Vector4,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vector3::zero(),
            normal: Vector3::zero(),
            tangent: Vector4::zero(),
            tex_coord_0: Vector2::zero(),
            tex_coord_1: Vector2::zero(),
            color_0: Vector4::zero(),
            joints_0: [0; 4],
            weights_0: Vector4::zero(),
        }
    }
}

#[derive(Debug)]
pub struct GltfPrimitive {
    pub mode: gltf::mesh::Mode,
    pub bounds: Aabb3,
    pub material: Rc<GltfMaterial>,
    pub vertices: Vec<Vertex>,
    pub index_count: u32,
}

impl GltfPrimitive {
    pub fn new(
        mode: gltf::mesh::Mode,
        bounds: Aabb3,
        vertices: &[Vertex],
        indices: Option<Vec<u32>>,
        material: Rc<GltfMaterial>,
    ) -> GltfPrimitive {
        let index_count = indices.as_ref().map(|i| i.len()).unwrap_or(0);
        GltfPrimitive {
            mode,
            bounds,
            material,
            vertices: vertices.to_vec(),
            index_count: index_count as u32,
        }
    }

    pub fn from_gltf(
        primitive_ref: &gltf::Primitive<'_>,
        primitive_index: usize,
        mesh_index: usize,
        model: &mut GltfModel,
        data: &GltfData,
        path: &Path,
    ) -> GltfPrimitive {
        let buffers = &data.buffers;
        let reader = primitive_ref.reader(|buffer| Some(&buffers[buffer.index()]));
        let positions = {
            let iter = reader.read_positions().unwrap_or_else(|| {
                panic!(
                    "primitives must have the POSITION attribute (mesh: {}, primitive: {})",
                    mesh_index, primitive_index
                )
            });
            iter.collect::<Vec<_>>()
        };

        let bounds = primitive_ref.bounding_box();
        let bounds = Aabb3 {
            min: bounds.min.into(),
            max: bounds.max.into(),
        };

        let mut vertices: Vec<Vertex> = positions
            .into_iter()
            .map(|position| Vertex {
                position: Vector3::from(position),
                ..Vertex::default()
            })
            .collect();

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

        // tangents
        if let Some(tangents) = reader.read_tangents() {
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
        }

        // texture coordinates
        let mut tex_coord_set = 0;
        while let Some(tex_coords) = reader.read_tex_coords(tex_coord_set) {
            if tex_coord_set > 1 {
                warn!(
                    "Ignoring texture coordinate set {}, \
                     only supporting 2 sets at the moment. (mesh: {}, primitive: {})",
                    tex_coord_set, mesh_index, primitive_index
                );
                tex_coord_set += 1;
                continue;
            }
            for (i, tex_coord) in tex_coords.into_f32().enumerate() {
                match tex_coord_set {
                    0 => vertices[i].tex_coord_0 = Vector2::from(tex_coord),
                    1 => vertices[i].tex_coord_1 = Vector2::from(tex_coord),
                    _ => unreachable!(),
                }
            }
            //shader_flags |= ShaderFlags::HAS_UV;
            tex_coord_set += 1;
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

        if let Some(joints) = reader.read_joints(0) {
            for (i, joint) in joints.into_u16().enumerate() {
                vertices[i].joints_0 = joint;
            }
        }
        if reader.read_joints(1).is_some() {
            warn!("Ignoring further joint attributes, only supporting JOINTS_0. (mesh: {}, primitive: {})",
                mesh_index, primitive_index);
        }

        if let Some(weights) = reader.read_weights(0) {
            for (i, weights) in weights.into_f32().enumerate() {
                vertices[i].weights_0 = weights.into();
            }
        }
        if reader.read_weights(1).is_some() {
            warn!("Ignoring further weight attributes, only supporting WEIGHTS_0. (mesh: {}, primitive: {})",
                mesh_index, primitive_index);
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

        let material_ref = primitive_ref.material();

        let mut material = None;
        if let Some(mat) = model
            .materials
            .iter()
            .find(|m| (***m).index == material_ref.index())
        {
            material = Rc::clone(mat).into()
        }

        if material.is_none() {
            let mat = Rc::new(GltfMaterial::from_gltf(&material_ref, model, data, path));
            model.materials.push(Rc::clone(&mat));
            material = Some(mat);
        };
        let material = material.unwrap();
        //shader_flags |= material.shader_flags();

        GltfPrimitive::new(mode, bounds, &vertices, indices, material)
    }
}
