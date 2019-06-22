use crate::math::*;
use crate::Dimensions;
use crate::GltfIndex;
use crate::GltfModel;
use crate::Result;

#[derive(Debug)]
pub struct GltfScene {
    pub name: Option<String>,
    pub nodes: Vec<GltfIndex>,
    pub dimensions: Dimensions,
}

impl Default for GltfScene {
    fn default() -> Self {
        Self {
            name: None,
            nodes: vec![],
            dimensions: Default::default(),
        }
    }
}

impl GltfScene {
    pub fn from_gltf(scene_ref: &gltf::Scene<'_>, model: &mut GltfModel) -> Result<GltfScene> {
        let mut scene = GltfScene {
            name: scene_ref.name().map(|s| s.to_owned()),
            ..Default::default()
        };

        scene.nodes = scene_ref.nodes().map(|node_ref| node_ref.index()).collect();

        let mut min = Vector3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX);
        let mut max = Vector3::new(std::f32::MIN, std::f32::MIN, std::f32::MIN);
        for node_index in &scene.nodes {
            let node_ref = model
                .linear_nodes
                .iter()
                .find(|&node_ref| node_ref.borrow().node_index == *node_index);
            if let Some(ref node_ref) = node_ref {
                node_ref
                    .borrow()
                    .compute_dimensions(model, &mut min, &mut max);
            }
        }
        scene.dimensions = Dimensions::new(min, max);
        Ok(scene)
    }
}
