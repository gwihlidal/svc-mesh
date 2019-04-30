use crate::GltfAnimation;
use crate::GltfData;
use crate::GltfMaterial;
use crate::GltfMesh;
use crate::GltfNode;
use crate::GltfSkin;
use crate::GltfTexture;
use crate::Dimensions;
use crate::Vector3;
use std::path::Path;
use std::rc::Rc;

#[derive(Default, Debug)]
pub struct GltfModel {
    pub nodes: Vec<Rc<GltfNode>>,

    pub meshes: Vec<Rc<GltfMesh>>,
    pub textures: Vec<Rc<GltfTexture>>,
    pub materials: Vec<Rc<GltfMaterial>>,
    pub animations: Vec<Rc<GltfAnimation>>,
    pub skins: Vec<Rc<GltfSkin>>,

    pub dimensions: Dimensions,
}

impl GltfModel {
    pub fn from_gltf(data: &GltfData, path: &Path) -> Self {
        let mut model = GltfModel::default();

        // Load textures
        model.textures = data
            .document
            .textures()
            .map(|texture_ref| Rc::new(GltfTexture::from_gltf(&texture_ref, data, path)))
            .collect();

        // Load materials
        model.materials = data
            .document
            .materials()
            .map(|material_ref| Rc::new(GltfMaterial::from_gltf(&material_ref, data, path)))
            .collect();

        // Load nodes
        model.nodes = data
            .document
            .nodes()
            .map(|node_ref| GltfNode::from_gltf(None, &node_ref, &mut model, data, path))
            .collect();

        // Load cameras
        /*root.camera_nodes = root.nodes.iter()
        .filter(|node| node.camera.is_some())
        .map(|node| node.index)
        .collect();*/

        // Load animations
        model.animations = data
            .document
            .animations()
            .map(|animation_ref| GltfAnimation::from_gltf(&animation_ref, data, path))
            .filter(|animation| {
                // Only keep animations with valid channels
                animation.channels.len() > 0
            })
            .collect();

        // Load skins
        model.skins = data
            .document
            .skins()
            .map(|skin_ref| GltfSkin::from_gltf(&skin_ref, data, path))
            .collect();

        // Finalize
        model.merge_skins();
        model.compute_dimensions();
        println!("Dimensions: {:?}", model.dimensions);
        model
    }

    pub fn unsafe_get_node_mut(&mut self, index: usize) -> &'static mut Rc<GltfNode> {
        unsafe { &mut *(&mut self.nodes[index] as *mut Rc<GltfNode>) }
    }

    fn merge_skins(&mut self) {
        // A gltf model can contain multiple meshes with multiple primitives (or "parts").
        // We already merge these into a single asset with multiple parts, so we also need to merge the skins here.

        /*
        for (auto node : linearNodes)
        {
            // Assign skins
            if (node->skinIndex > -1)
            {
                node->skin = skins[node->skinIndex];
            }

            // Initial pose
            if (node->mesh)
            {
                //node->update();
            }
        }
        */
    }

    fn compute_dimensions(&mut self) {
        use std::f32;
        let mut min = Vector3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max = Vector3::new(f32::MIN, f32::MIN, f32::MIN);
        for node in &self.nodes {
            node.compute_dimensions(self, &mut min, &mut max);
        }
        self.dimensions = Dimensions::new(min, max);
    }
}
