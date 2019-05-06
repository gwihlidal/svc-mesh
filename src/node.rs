use super::GltfData;
use super::GltfIndex;
use super::GltfMesh;
use super::GltfModel;
use crate::Result;
use crate::{Matrix4, Quaternion, Unit, UnitQuaternion, Vector3 /*, Vector4*/};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

#[derive(Debug)]
pub struct GltfNode {
    pub node_index: GltfIndex,
    pub joint_index: Option<GltfIndex>,

    pub parent: Option<GltfNodeRef>,
    pub children: Vec<GltfNodeRef>,

    pub name: Option<String>,
    pub mesh: Option<Rc<GltfMesh>>,

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
    ) -> Result<GltfNodeRef> {
        // Load transformation data, default will be identity
        let (translation, rotation, scale) = node_ref.transform().decomposed();
        //let matrix = node_ref.transform().matrix();

        // gltf quat format: [x, y, z, w], argument order expected by our quaternion: (w, x, y, z)
        let rotation = Unit::new_normalize(Quaternion::new(
            rotation[3],
            rotation[0],
            rotation[1],
            rotation[2],
        ));

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
                mesh = Some(GltfMesh::from_gltf(&mesh_ref, data)?);
                model.meshes.push(mesh.clone().unwrap());
            }
        }

        let node = Rc::new(RefCell::new(GltfNode {
            node_index: node_ref.index(),
            joint_index: None,
            parent: parent.clone(),
            children: Vec::new(),
            name: node_ref.name().map(|s| s.into()),
            mesh,
            translation: Vector3::new(translation[0], translation[1], translation[2]),
            scale: Vector3::new(scale[0], scale[1], scale[2]),
            rotation,
        }));

        //println!("Node Rotation: {:?}", node.borrow().rotation);

        let children = node_ref
            .children()
            .map(|ref node_ref| {
                GltfNode::from_gltf(Some(node.clone()), node_ref, model, data, base_path)
            })
            .collect::<Result<Vec<GltfNodeRef>>>()?;

        node.borrow_mut().children = children;

        Ok(node)
    }

    pub fn local_matrix(&self) -> Matrix4 {
        let translation = Matrix4::new_translation(&self.translation);
        //println!("Local Translation: {:?}", translation);
        //let rotation = UnitQuaternion::new_unchecked(*self.rotation).to_homogeneous();
        //println!("Local Rotation1: {:?}", self.rotation);
        let rotation = self.rotation.to_homogeneous();
        //println!("Local Rotation: {:?}", rotation);
        let scale = Matrix4::new_nonuniform_scaling(&self.scale);
        //println!("Local Scale: {:?}", scale);
        translation * rotation * scale
    }

    pub fn get_matrix(&self) -> Matrix4 {
        let local_matrix = self.local_matrix();
        //println!("Local Matrix: {:?}", local_matrix);
        let chained_matrix = self.get_matrix_chain(self.parent.clone(), &local_matrix);
        //println!("Chained Matrix: {:?}", chained_matrix);
        chained_matrix
    }

    fn get_matrix_chain(
        &self,
        parent_ref: Option<GltfNodeRef>,
        current_matrix: &Matrix4,
    ) -> Matrix4 {
        if let Some(parent) = parent_ref {
            let next_parent = parent.borrow().parent.clone();

            let matrix = parent.borrow().local_matrix();
            //println!("Local Matrix: {:?}", matrix);
            let matrix = matrix * current_matrix;
            //println!("Local Matrix2: {:?}", matrix);
            self.get_matrix_chain(next_parent, &matrix)

        //let matrix = self.get_matrix_chain(next_parent, &current_matrix);
        //parent.borrow().local_matrix() * matrix
        } else {
            current_matrix.clone()
        }
    }

    pub fn compute_dimensions(&self, model: &GltfModel, min: &mut Vector3, max: &mut Vector3) {
        if let Some(ref mesh) = self.mesh {
            let node_matrix = self.get_matrix();
            for primitive in &mesh.primitives {
                //println!("Min: {:?}, Max: {:?}", &primitive.dimensions.min, &primitive.dimensions.max);

                //println!("Node Matrix: {:?}", node_matrix);
                let loc_min = node_matrix.transform_vector(&primitive.dimensions.min);
                let loc_max = node_matrix.transform_vector(&primitive.dimensions.max);

                //println!("LocMin: {:?}, LocMax: {:?}", &loc_min, &loc_max);

                min.x = loc_min.x.min(min.x);
                min.y = loc_min.y.min(min.y);
                min.z = loc_min.z.min(min.z);

                max.x = loc_max.x.max(max.x);
                max.y = loc_max.y.max(max.y);
                max.z = loc_max.z.max(max.z);

                //println!("min[{}, {}, {}], max[{}, {}, {}]", min.x, min.y, min.z, max.x, max.y, max.z);
            }
        }

        for child_node in &self.children {
            child_node.borrow().compute_dimensions(model, min, max);
        }
    }
}
