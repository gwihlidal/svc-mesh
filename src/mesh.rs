use super::GltfData;
use super::GltfModel;
//use crate::math::*;
use crate::GltfIndex;
use crate::GltfPrimitive;
//use collision::{Aabb, Union};
use std::path::Path;
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
        base_path: &Path,
    ) -> Rc<GltfMesh> {
        let primitives: Vec<GltfPrimitive> = mesh_ref
            .primitives()
            .enumerate()
            .map(|(i, prim_ref)| {
                GltfPrimitive::from_gltf(&prim_ref, i, mesh_ref.index(), model, data, base_path)
            })
            .collect();

        /*let bounds = primitives
        .iter()
        .fold(Aabb3::zero(), |bounds, prim| prim.bounds.union(&bounds));*/

        Rc::new(GltfMesh {
            index: mesh_ref.index(),
            primitives,
            name: mesh_ref.name().map(|s| s.into()),
            //bounds,
        })
    }
}
