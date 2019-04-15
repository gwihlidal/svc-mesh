use super::GltfData;
use super::GltfIndex;
use super::GltfMesh;
use super::GltfModel;
use crate::{Matrix4, Quaternion, Vector3};
use std::path::Path;
use std::rc::Rc;

#[derive(Debug)]
pub struct GltfNode {
    pub node_index: GltfIndex,
    pub joint_index: Option<GltfIndex>,

    pub parent: Option<Rc<GltfNode>>,
    pub children: Vec<GltfIndex>,
    pub name: Option<String>,
    pub mesh: Option<Rc<GltfMesh>>,

    pub matrix: Matrix4,
    pub translation: Vector3,
    pub scale: Vector3,
    pub rotation: Quaternion,
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
        let matrix = node_ref.transform().matrix();

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
            node_index: node_ref.index(),
            joint_index: None,
            parent: parent.clone(),
            children,
            name: node_ref.name().map(|s| s.into()),
            mesh,
            matrix: Matrix4::new(
                matrix[0][0],
                matrix[0][1],
                matrix[0][2],
                matrix[0][3],
                matrix[1][0],
                matrix[1][1],
                matrix[1][2],
                matrix[1][3],
                matrix[2][0],
                matrix[2][1],
                matrix[2][2],
                matrix[2][3],
                matrix[3][0],
                matrix[3][1],
                matrix[3][2],
                matrix[3][3],
            ),
            translation: Vector3::new(trans[0], trans[1], trans[2]),
            scale: Vector3::new(scale[0], scale[1], scale[2]),
            rotation,
        });

        node
    }

    pub fn local_matrix(&self) -> Matrix4 {
        let translation = Matrix4::from_translation(self.translation);
        let rotation: Matrix4 = self.rotation.into();
        let scale = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        translation * rotation * scale
    }

    pub fn get_matrix(&self) -> Matrix4 {
        let mut matrix = self.local_matrix();
        let mut current_parent = &self.parent;
        while let Some(ref parent) = current_parent {
            matrix = parent.local_matrix() * matrix;
            current_parent = &parent.parent;
        }
        matrix
    }

    pub fn compute_dimensions(&self, model: &GltfModel, min: &mut Vector3, max: &mut Vector3) {
        if let Some(ref mesh) = self.mesh {
            for primitive in &mesh.primitives {
             /*   let bounds = primitives
            .iter()
            .fold(Aabb3::zero(), |bounds, prim| prim.bounds.union(&bounds));*/
            }
        }

        for i in 0..self.children.len() {
            let child_index = self.children[i];
            let child_node = &model.nodes[child_index];
            child_node.compute_dimensions(model, min, max);
        }

/*
        if (node->mesh)
        {
            for (GltfPrimitive* primitive : node->mesh->primitives)
            {
                glm::vec4 locMin = glm::vec4(primitive->dimensions.min, 1.0f) * node->getMatrix();
                glm::vec4 locMax = glm::vec4(primitive->dimensions.max, 1.0f) * node->getMatrix();
                if (locMin.x < min.x) { min.x = locMin.x; }
                if (locMin.y < min.y) { min.y = locMin.y; }
                if (locMin.z < min.z) { min.z = locMin.z; }
                if (locMax.x > max.x) { max.x = locMax.x; }
                if (locMax.y > max.y) { max.y = locMax.y; }
                if (locMax.z > max.z) { max.z = locMax.z; }
            }
        }
        */
    }
}
