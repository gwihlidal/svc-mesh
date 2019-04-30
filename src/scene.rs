use super::math::*;
use super::GltfModel;

#[derive(Debug)]
pub struct GltfScene {
    pub name: Option<String>,
    pub nodes: Vec<usize>,
    //pub bounds: Aabb3,
}

impl Default for GltfScene {
    fn default() -> Self {
        Self {
            name: None,
            nodes: vec![],
            //bounds: Aabb3::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0)),
        }
    }
}

impl GltfScene {
    pub fn from_gltf(scene_ref: &gltf::Scene<'_>, _model: &mut GltfModel) -> GltfScene {
        let mut scene = GltfScene {
            name: scene_ref.name().map(|s| s.to_owned()),
            ..Default::default()
        };

        scene.nodes = scene_ref.nodes().map(|node_ref| node_ref.index()).collect();

        // propagate transforms
        //let root_transform = Matrix4::identity();
        for _node_index in &scene.nodes {
            //let _node = model.unsafe_get_node_mut(*node_index);
            //node.update_transform(root, &root_transform);
            //node.update_bounds(root);
            //scene.bounds = scene.bounds.union(&node.bounds);
        }

        scene
    }
}
