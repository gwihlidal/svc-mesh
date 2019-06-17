use super::GltfData;
use crate::GltfIndex;
use crate::GltfModel;
use crate::GltfPrimitive;
use crate::Result;
use std::rc::Rc;

#[derive(Debug)]
pub struct GltfMesh {
    pub index: GltfIndex,
    pub primitives: Vec<GltfPrimitive>,
    pub name: Option<String>,
    //pub bounds: Aabb3,
}

impl GltfMesh {
    pub fn from_gltf(
        mesh_ref: &gltf::Mesh<'_>,
        model: &mut GltfModel,
        data: &GltfData,
    ) -> Result<Rc<GltfMesh>> {
        let primitives: Vec<GltfPrimitive> = mesh_ref
            .primitives()
            .enumerate()
            .map(|(i, prim_ref)| {
                GltfPrimitive::from_gltf(&prim_ref, i, mesh_ref.index(), model, data)
            })
            .collect::<Result<_>>()?;

        /*
        println!("Gltf Mesh: {}", mesh_ref.name().unwrap_or_default());
        for primitive in &primitives {
            println!("***");
            println!("Mesh Index: {:?}", primitive.mesh_index);
            println!("Primitive Index: {:?}", primitive.primitive_index);
            if let Some(ref material_index) = primitive.material_index {
                println!("Material Index: {:?}", material_index);
            } else {
                println!("Material Index: Default");
            }
            println!("Dimensions: {:?}", primitive.dimensions);
            println!("Mode: {:?}", primitive.mode);
            if let Some(ref faces) = primitive.faces {
                println!("Index Count: {:?}", faces.len());
                println!("Face Count: {:?}", faces.len() / 3);
            } else {
                println!("No Indices");
            }
            println!("Position Count: {:?}", primitive.positions.len());
            println!("Normals Count: {:?}", primitive.normals.len());
            println!("Tangents Count: {:?}", primitive.tangents.len());
            if let Some(ref color0) = primitive.color0 {
                println!("Color0 Count: {:?}", color0.len());
            }
            println!("Coord0 Count: {:?}", primitive.uv0.len());
            if let Some(ref joints0) = primitive.joints0 {
                println!("Joints0 Count: {:?}", joints0.len());
            }
            if let Some(ref joints1) = primitive.joints1 {
                println!("Joints1 Count: {:?}", joints1.len());
            }
            if let Some(ref joints2) = primitive.joints2 {
                println!("Joints2 Count: {:?}", joints2.len());
            }
            if let Some(ref joints3) = primitive.joints3 {
                println!("Joints3 Count: {:?}", joints3.len());
            }
            if let Some(ref weights0) = primitive.weights0 {
                println!("Weights0 Count: {:?}", weights0.len());
            }
            if let Some(ref weights1) = primitive.weights1 {
                println!("Weights1 Count: {:?}", weights1.len());
            }
            if let Some(ref weights2) = primitive.weights2 {
                println!("Weights2 Count: {:?}", weights2.len());
            }
            if let Some(ref weights3) = primitive.weights3 {
                println!("Weights3 Count: {:?}", weights3.len());
            }
        }
        */

        /*let bounds = primitives
        .iter()
        .fold(Aabb3::zero(), |bounds, prim| prim.bounds.union(&bounds));*/

        Ok(Rc::new(GltfMesh {
            index: mesh_ref.index(),
            primitives,
            name: mesh_ref.name().map(|s| s.into()),
            //bounds,
        }))
    }
}
