use super::GltfData;
use super::GltfIndex;
use super::GltfMesh;
use super::GltfModel;
use crate::{Matrix4, Quaternion, UnitQuaternion, Vector3, Vector4};
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct GltfNode {
    pub node_index: GltfIndex,
    pub joint_index: Option<GltfIndex>,

    pub parent: Option<GltfNodeRef>,
    pub children: Vec<GltfNodeRef>,

    pub name: Option<String>,
    pub mesh: Option<Rc<GltfMesh>>,

    pub matrix: Matrix4,
    pub translation: Vector3,
    pub scale: Vector3,
    pub rotation: UnitQuaternion,
}

pub type GltfNodeRef = Rc<RefCell<GltfNode>>;

impl GltfNode {
    pub fn from_gltf(
        parent: Option<GltfNodeRef>,
        node_ref: &gltf::Node<'_>,
        model: &mut GltfModel,
        data: &GltfData,
        base_path: &Path,
    ) -> GltfNodeRef {
        let matrix = node_ref.transform().matrix();
        let (trans, rot, scale) = node_ref.transform().decomposed();
        let r = rot;
        let rotation = Quaternion::new(r[0], r[1], r[2], r[3]);
        //let rotation = UnitQuaternion::new(r[3], r[0], r[1], r[2]); // NOTE: different element order!
        let rotation = UnitQuaternion::from_quaternion(rotation);

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

        let mut node = Rc::new(RefCell::new(GltfNode {
            node_index: node_ref.index(),
            joint_index: None,
            parent: parent.clone(),
            children: Vec::new(),
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
        }));

        let children: Vec<GltfNodeRef> = node_ref
            .children()
            .map(|ref node_ref| {
                GltfNode::from_gltf(
                    Some(node.clone()),
                    node_ref,
                    model,
                    data,
                    base_path,
                )
            }).collect();

        node.borrow_mut().children = children;

        node
    }

    pub fn local_matrix(&self) -> Matrix4 {
        let translation = Matrix4::new_translation(&self.translation);
        //let rotation = UnitQuaternion::new_unchecked(*self.rotation).to_homogeneous();
        let rotation = self.rotation.to_homogeneous();
        let scale = Matrix4::new_nonuniform_scaling(&self.scale);
        translation * rotation * scale
    }

    pub fn get_matrix(&self) -> Matrix4 {
        let mut matrix = self.local_matrix();
        let mut current_parent = &self.parent;
        println!("HERE");
        while let Some(ref parent) = current_parent {
            let parent = parent.borrow();
            matrix = parent.local_matrix() * matrix;
            println!("GW: {:?}", matrix);
            current_parent = &parent.parent;
        }
        matrix
    }

    pub fn compute_dimensions(&self, model: &GltfModel, min: &mut Vector3, max: &mut Vector3) {
        if let Some(ref mesh) = self.mesh {
            for primitive in &mesh.primitives {
               // let loc_min = Vector4::new(primitive.dimensions.min.x, primitive.dimensions.min.y, primitive.dimensions.min.z, 1.0);
                //let loc_max = Vector4::new(primitive.dimensions.max.x, primitive.dimensions.max.y, primitive.dimensions.max.z, 1.0);

                let node_matrix = self.get_matrix();
                let loc_min = node_matrix.transform_vector(&primitive.dimensions.min);
                let loc_max = node_matrix.transform_vector(&primitive.dimensions.max);
                
                min.x = min.x.min(loc_min.x);
                min.y = min.y.min(loc_min.y);
                min.z = min.z.min(loc_min.z);

                max.x = max.x.max(loc_max.x);
                max.y = max.y.max(loc_max.y);
                max.z = max.z.max(loc_max.z);
            }
        }

        for child_node in &self.children {
            child_node.borrow_mut().compute_dimensions(model, min, max);
        }
    }
}
