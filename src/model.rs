use crate::GltfData;
use crate::GltfMaterial;
use crate::GltfMesh;
use crate::GltfNode;
use std::path::Path;
use std::rc::Rc;

#[derive(Default, Debug)]
pub struct GltfModel {
    pub nodes: Vec<Rc<GltfNode>>,

    pub meshes: Vec<Rc<GltfMesh>>,
    //pub textures: Vec<Rc<GltfTexture>>,
    pub materials: Vec<Rc<GltfMaterial>>,
}

impl GltfModel {
    pub fn from_gltf(data: &GltfData, path: &Path) -> Self {
        let mut model = GltfModel::default();
        let nodes = data
            .document
            .nodes()
            .map(|node_ref| GltfNode::from_gltf(None, &node_ref, &mut model, data, path))
            .collect();
        model.nodes = nodes;
        /*root.camera_nodes = root.nodes.iter()
        .filter(|node| node.camera.is_some())
        .map(|node| node.index)
        .collect();*/
        model
    }

    pub fn unsafe_get_node_mut(&mut self, index: usize) -> &'static mut Rc<GltfNode> {
        unsafe { &mut *(&mut self.nodes[index] as *mut Rc<GltfNode>) }
    }
}
