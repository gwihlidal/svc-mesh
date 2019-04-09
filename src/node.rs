use super::GltfData;
use super::GltfIndex;
use super::GltfMesh;
use super::GltfModel;
use crate::Quaternion;
use std::path::Path;
use std::rc::Rc;

#[derive(Default, Debug)]
pub struct GltfNode {
    pub index: GltfIndex,
    pub parent: Option<Rc<GltfNode>>,
    pub children: Vec<GltfIndex>,
    pub name: Option<String>,
    pub mesh: Option<Rc<GltfMesh>>,
}

impl GltfNode {
    pub fn from_gltf(
        parent: Option<Rc<GltfNode>>,
        node_ref: &gltf::Node<'_>,
        model: &mut GltfModel,
        data: &GltfData,
        base_path: &Path,
    ) -> Rc<GltfNode> {
        let (trans, rot, scale) = node_ref.transform().decomposed();
        let r = rot;
        let rotation = Quaternion::new(r[3], r[0], r[1], r[2]); // NOTE: different element order!

        let mut mesh = None;
        if let Some(mesh_ref) = node_ref.mesh() {
            if let Some(existing_mesh) = model
                .meshes
                .iter()
                .find(|mesh| (***mesh).index == mesh_ref.index())
            {
                mesh = Some(Rc::clone(existing_mesh));
            }

            if mesh.is_none() {
                mesh = Some(GltfMesh::from_gltf(&mesh_ref, model, data, base_path));
                model.meshes.push(mesh.clone().unwrap());
            }
        }

        let children: Vec<usize> = node_ref
            .children()
            .map(|node_ref| node_ref.index())
            .collect();

        /*let children: Vec<Rc<GltfNode>> = children
        .iter()
        .map(|node_index| model.unsafe_get_node_mut(*node_index).clone())
        .collect();*/

        let node = Rc::new(GltfNode {
            index: node_ref.index(),
            parent: parent.clone(),
            children,
            name: node_ref.name().map(|s| s.into()),
            mesh,
        });

        node
    }
}
