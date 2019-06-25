#![allow(dead_code)]

use super::GltfIndex;
use crate::Dimensions;
use crate::GltfAnimation;
use crate::GltfData;
use crate::GltfMaterial;
use crate::GltfMesh;
use crate::GltfNode;
use crate::GltfNodeRef;
use crate::GltfSkin;
use crate::GltfTexture;
use crate::Result;
use crate::Vector3;
use std::path::Path;
use std::rc::Rc;

#[repr(C)]
#[derive(Debug)]
pub struct GltfVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv0: [f32; 2],
    pub color0: [f32; 4],
    pub joint0: [u16; 4],
    pub joint1: [u16; 4],
    pub joint2: [u16; 4],
    pub joint3: [u16; 4],
    pub weight0: [f32; 4],
    pub weight1: [f32; 4],
    pub weight2: [f32; 4],
    pub weight3: [f32; 4],

    pub tangent: [f32; 3],
    pub bitangent: [f32; 3],

    pub influence_count: u32,
    pub skin_index: i32,
}

#[derive(Default, Debug)]
pub struct GltfModel {
    pub root_nodes: Vec<GltfNodeRef>,   // root nodes
    pub linear_nodes: Vec<GltfNodeRef>, // all nodes

    pub meshes: Vec<Rc<GltfMesh>>,
    pub textures: Vec<Rc<GltfTexture>>,
    pub materials: Vec<Rc<GltfMaterial>>,
    pub animations: Vec<Rc<GltfAnimation>>,
    pub skins: Vec<Rc<GltfSkin>>,

    pub dimensions: Dimensions,

    pub index_buffer: Vec<u32>,
    pub vertex_buffer: Vec<GltfVertex>,
}

impl GltfModel {
    pub fn print_nodes(_node: &GltfNodeRef, indent: String) {
        let mut name = "".to_string();
        if let Some(name_ref) = &_node.borrow().name {
            name = name_ref.clone();
        }

        println!(
            "{}node_index:{} node_str:{} children:{}",
            indent,
            _node.borrow().node_index.to_string(),
            name,
            _node.borrow().children.len().to_string()
        );

        let child_indent = indent + " ";
        for node in _node.borrow().children.iter() {
            GltfModel::print_nodes(&node, child_indent.clone());
        }
    }

    pub fn node_from_index(&self, index: GltfIndex) -> Option<GltfNodeRef> {
        let mut found_node = None;
        if let Some(existing_node) = self
            .linear_nodes
            .iter()
            .find(|node| (***node).borrow().node_index == index)
        {
            found_node = Some(Rc::clone(existing_node));
        }
        found_node
    }

    fn collect_nodes(&self, _node: &GltfNodeRef, res: &mut Vec<GltfNodeRef>) {
        res.push(_node.clone());
        for node in _node.borrow().children.iter() {
            self.collect_nodes(node, res);
        }
    }

    pub fn from_gltf(data: &GltfData, path: &Path) -> Result<Self> {
        let mut model = GltfModel::default();

        // Load textures
        model.textures = data
            .document
            .textures()
            .map(|texture_ref| Ok(Rc::new(GltfTexture::from_gltf(&texture_ref, data, path)?)))
            .collect::<Result<_>>()?;

        // Load materials
        model.materials = data
            .document
            .materials()
            .map(|material_ref| Ok(Rc::new(GltfMaterial::from_gltf(&material_ref, data, path)?)))
            .collect::<Result<_>>()?;

        // Load nodes
        for scene in data.document.scenes() {
            let mut nodev: Vec<GltfNodeRef> = scene
                .nodes()
                .map(|node_ref| GltfNode::from_gltf(None, &node_ref, &mut model, data, path))
                .collect::<Result<_>>()?;
            model.root_nodes.append(&mut nodev);
        }

        let mut res_nodes: Vec<GltfNodeRef> = Vec::new();
        for node in &model.root_nodes {
            model.collect_nodes(node, &mut res_nodes);
        }
        model.linear_nodes.append(&mut res_nodes);

        // Print Nodes
        // for node in model.root_nodes.iter() {
        //     GltfModel::print_nodes(&node, "".to_string());
        // }

        // for node in model.linear_nodes.iter() {
        //      println!("Node: Children {}",node.borrow().children.len());
        // }

        // Load cameras
        /*root.camera_nodes = root.nodes.iter()
        .filter(|node| node.camera.is_some())
        .map(|node| node.index)
        .collect();*/

        // Load animations
        model.animations = data
            .document
            .animations()
            .map(|animation_ref| GltfAnimation::from_gltf(&animation_ref, data, path, &model))
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
        Ok(model)
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
        for node in &self.linear_nodes {
            node.borrow().compute_dimensions(self, &mut min, &mut max);
        }
        self.dimensions = Dimensions::new(min, max);
    }
}
