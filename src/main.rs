//use gltf::{buffer::Source as BufferSource, image::Source as ImageSource, Gltf};
use std::boxed::Box;
use std::error::Error as StdError;
use std::path::Path;

mod animation;
mod material;
mod math;
mod mesh;
mod model;
mod node;
mod primitive;
mod scene;
mod texture;

use animation::*;
use material::*;
use math::*;
use mesh::*;
use model::*;
use node::*;
use primitive::*;
use scene::*;
use texture::*;

/*
struct VertexSkinningData {}

struct MeshData {
    bonesAndWeights: Vec<VertexSkinningData>,
    vertexPositions: Vec<Vector3>,
    vertexTexCoords: Vec<Vector2>,
    vertexNormals: Vec<Vector3>,
    vertexTangents: Vec<Vector3>,
    vertexBitangents: Vec<Vector3>,
    vertexColors: Vec<Vector4>,
    indices: Vec<u32>,
}*/

#[derive(Debug)]
pub struct GltfData {
    pub document: gltf::Document,
    pub buffers: Vec<gltf::buffer::Data>,
    pub images: Vec<gltf::image::Data>,
}

pub type GltfIndex = usize;

fn load_model(path: &Path) -> Result<(), Box<StdError>> {
    let (document, buffers, images) = match gltf::import(path) {
        Ok(tuple) => tuple,
        Err(err) => {
            //error!("glTF import failed: {:?}", err);
            //if let gltf::Error::Io(_) = err {
            //error!("Hint: Are the .bin file(s) referenced by the .gltf file available?")
            //}
            //process::exit(1)
            panic!("failed to load ")
        }
    };

    let data = GltfData {
        document,
        buffers,
        images,
    };

    let mut model = GltfModel::from_gltf(&data, path);

    let scene_index = 0;
    if scene_index >= data.document.scenes().len() {
        //error!("Scene index too high - file has only {} scene(s)", imp.doc.scenes().len());
        //process::exit(3)
        panic!("scene index is too high");
    }

    let scene = GltfScene::from_gltf(
        &data.document.scenes().nth(scene_index).unwrap(),
        &mut model,
    );

    /*
    if gltf.animations().len() > 0 {
    //load_animations();
    }

    //load_skins();

    // A gltf model can contain multiple meshes with multiple primitives (or "parts").
    // We already merge these into a single MeshAsset with multiple parts, so we also need to merge the skins here.
    //merge_skins();

    //for node in linear_nodes {
    // Assign skins
    //if (node->skinIndex > -1)
    //{
    //    node->skin = skins[node->skinIndex];
    //}
    //}

    //let dimensions = calc_dimensions();
     */
    Ok(())
}

fn main() {
    //list(&"data/Book_03.glb").expect("runtime error");
    //list(&"data/BoxAnimated.glb").expect("runtime error");
    //list(&"data/Combat_Helmet.glb").expect("runtime error");
    //list(&"data/EpicCitadel.glb").expect("runtime error");
    //list(&"data/Floor_Junk_Cluster_01.glb").expect("runtime error");
    //list(&"data/Machine_01.glb").expect("runtime error");
    //list(&"data/RiggedFigure.glb").expect("runtime error");
    //list(&"data/Warrok.glb").expect("runtime error");

    //display(&"data/RiggedFigure.glb").expect("runtime error");

    load_model(&Path::new("data/Floor_Junk_Cluster_01.glb")).expect("runtime error");
    //load_model(&"data/Combat_Helmet.glb").expect("runtime error");
    load_model(&Path::new("data/SciFiHelmet.gltf")).expect("runtime error");
}
