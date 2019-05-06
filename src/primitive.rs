use crate::calculate_tangents;
use crate::math::*;
use crate::GltfData;
use crate::GltfIndex;
use crate::Result;

#[derive(Debug)]
pub struct GltfPrimitive {
    pub mode: gltf::mesh::Mode,
    pub mesh_index: GltfIndex,
    pub primitive_index: GltfIndex,
    pub material_index: Option<GltfIndex>,
    pub dimensions: Dimensions,

    pub faces: Option<Vec<usize>>,

    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub tangents: Vec<[f32; 3]>,

    pub color0: Option<Vec<[f32; 4]>>,
    //pub color1: Option<Vec<[f32; 4]>>,
    //pub color2: Option<Vec<[f32; 4]>>,
    //pub color3: Option<Vec<[f32; 4]>>,
    pub uv0: Vec<[f32; 2]>,
    //pub uv1: Option<Vec<[f32; 2]>>,
    //pub uv2: Option<Vec<[f32; 2]>>,
    //pub uv3: Option<Vec<[f32; 2]>>,
    pub joints0: Option<Vec<[u16; 4]>>,
    pub joints1: Option<Vec<[u16; 4]>>,
    pub joints2: Option<Vec<[u16; 4]>>,
    pub joints3: Option<Vec<[u16; 4]>>,

    pub weights0: Option<Vec<[f32; 4]>>,
    pub weights1: Option<Vec<[f32; 4]>>,
    pub weights2: Option<Vec<[f32; 4]>>,
    pub weights3: Option<Vec<[f32; 4]>>,
}

/*fn iterate_slices_counted_2<T>(xs: &[T], ys: &[T]) {
    let len = cmp::min(xs.len(), ys.len());
    let xs = &xs[..len];
    let ys = &ys[..len];
    for i in 0..len {
        let x = &xs[i];
        let y = &ys[i];
    }
}*/

impl GltfPrimitive {
    /*pub fn new(
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
    }*/

    pub fn from_gltf(
        primitive_ref: &gltf::Primitive<'_>,
        primitive_index: GltfIndex,
        mesh_index: GltfIndex,
        data: &GltfData,
    ) -> Result<GltfPrimitive> {
        use std::f32;

        let buffers = &data.buffers;
        let reader = primitive_ref.reader(|buffer| Some(&buffers[buffer.index()]));

        // Indices / Faces

        //let indices = reader
        //    .read_indices()
        //    .map(|read_indices| read_indices.into_u32().collect::<Vec<_>>());

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

        // Positions

        let positions = reader
            .read_positions()
            .map(|positions| match faces {
                Some(ref faces) => {
                    let vertices = positions.collect::<Vec<_>>();
                    faces.iter().map(|i| vertices[*i]).collect::<Vec<_>>()
                }
                None => positions.collect(),
            })
            .unwrap_or_default(); // ok_or

        /*let positions = {
            let iter = reader.read_positions().unwrap_or_else(|| {
                panic!(
                    "primitives must have the POSITION attribute (mesh: {}, primitive: {})",
                    mesh_index, primitive_index
                )
            });
            iter.collect::<Vec<_>>()
        };*/

        // Normals

        let normals = reader
            .read_normals()
            .map(|normals| match faces {
                Some(ref faces) => {
                    let normals = normals.collect::<Vec<_>>();
                    faces.iter().map(|i| normals[*i]).collect()
                }
                None => normals.collect(),
            })
            .unwrap_or_else(|| {
                use std::iter::once;
                let f = faces
                    .as_ref()
                    .map(|f| f.clone())
                    .unwrap_or_else(|| (0..positions.len()).collect::<Vec<_>>());
                f.chunks(3)
                    .flat_map(|chunk| {
                        let a = Vector3::from(positions[chunk[0]]);
                        let ab = Vector3::from(positions[chunk[1]]) - a;
                        let ac = Vector3::from(positions[chunk[2]]) - a;
                        let normal: [f32; 3] = ab.cross(&ac).into();
                        once(normal.clone())
                            .chain(once(normal.clone()))
                            .chain(once(normal))
                    })
                    .collect::<Vec<_>>()
            });

        // Texture Coordinates

        let uv0 = reader
            .read_tex_coords(0)
            .map(|tex_coords| tex_coords.into_f32().collect::<Vec<[f32; 2]>>())
            .unwrap_or_else(|| {
                vec![
                    [
                        data.options.generate_tex_coords.0,
                        data.options.generate_tex_coords.1
                    ];
                    positions.len()
                ]
            });
        let uv0: Vec<[f32; 2]> = match faces {
            Some(ref faces) => faces
                .iter()
                .map(|i| flip_check(uv0[*i], data.options.flip_v_coord))
                .collect(),
            None => uv0
                .into_iter()
                .map(|t| flip_check(t, data.options.flip_v_coord))
                .collect(),
        };

        // TODO: uv1, uv2, uv3, ...

        // Tangents

        let tangents = if data.options.regenerate_tangents {
            calculate_tangents(&positions, &normals, &uv0)
        } else {
            reader
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
                .unwrap_or_else(|| calculate_tangents(&positions, &normals, &uv0))
        };

        // Vertex Colors

        let color0 = reader
            .read_colors(0)
            .map(|colors| colors.into_rgba_f32())
            .map(|colors| match faces {
                Some(ref faces) => {
                    let colors = colors.collect::<Vec<_>>();
                    faces.iter().map(|i| colors[*i]).collect()
                }
                None => colors.collect(),
            });

        // Skinning Joints

        let joints0 =
            reader
                .read_joints(0)
                .map(|joints| joints.into_u16())
                .map(|joints| match faces {
                    Some(ref faces) => {
                        let joints = joints.collect::<Vec<_>>();
                        faces.iter().map(|i| joints[*i]).collect()
                    }
                    None => joints.collect(),
                });

        let joints1 =
            reader
                .read_joints(1)
                .map(|joints| joints.into_u16())
                .map(|joints| match faces {
                    Some(ref faces) => {
                        let joints = joints.collect::<Vec<_>>();
                        faces.iter().map(|i| joints[*i]).collect()
                    }
                    None => joints.collect(),
                });

        let joints2 =
            reader
                .read_joints(2)
                .map(|joints| joints.into_u16())
                .map(|joints| match faces {
                    Some(ref faces) => {
                        let joints = joints.collect::<Vec<_>>();
                        faces.iter().map(|i| joints[*i]).collect()
                    }
                    None => joints.collect(),
                });

        let joints3 =
            reader
                .read_joints(3)
                .map(|joints| joints.into_u16())
                .map(|joints| match faces {
                    Some(ref faces) => {
                        let joints = joints.collect::<Vec<_>>();
                        faces.iter().map(|i| joints[*i]).collect()
                    }
                    None => joints.collect(),
                });

        // Skinning Weights

        let weights0 = reader
            .read_weights(0)
            .map(|weights| weights.into_f32())
            .map(|weights| match faces {
                Some(ref faces) => {
                    let weights = weights.collect::<Vec<_>>();
                    faces.iter().map(|i| weights[*i]).collect()
                }
                None => weights.collect(),
            });

        let weights1 = reader
            .read_weights(1)
            .map(|weights| weights.into_f32())
            .map(|weights| match faces {
                Some(ref faces) => {
                    let weights = weights.collect::<Vec<_>>();
                    faces.iter().map(|i| weights[*i]).collect()
                }
                None => weights.collect(),
            });

        let weights2 = reader
            .read_weights(2)
            .map(|weights| weights.into_f32())
            .map(|weights| match faces {
                Some(ref faces) => {
                    let weights = weights.collect::<Vec<_>>();
                    faces.iter().map(|i| weights[*i]).collect()
                }
                None => weights.collect(),
            });

        let weights3 = reader
            .read_weights(3)
            .map(|weights| weights.into_f32())
            .map(|weights| match faces {
                Some(ref faces) => {
                    let weights = weights.collect::<Vec<_>>();
                    faces.iter().map(|i| weights[*i]).collect()
                }
                None => weights.collect(),
            });

        // Bounding Dimensions and Meta Data

        let bounds = primitive_ref.bounding_box();
        let dimensions = Dimensions::new(bounds.min.into(), bounds.max.into());

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

        let material_index = primitive_ref.material().index();

        Ok(GltfPrimitive {
            mode,
            primitive_index,
            mesh_index,
            material_index,
            dimensions,
            faces,
            positions,
            normals,
            tangents,
            color0,
            uv0,
            joints0,
            joints1,
            joints2,
            joints3,
            weights0,
            weights1,
            weights2,
            weights3,
        })
    }
}
