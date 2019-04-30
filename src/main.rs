//use gltf::{buffer::Source as BufferSource, image::Source as ImageSource, Gltf};
use std::boxed::Box;
use std::error::Error as StdError;
use std::path::Path;
use std::rc::Rc;

mod animation;
mod data;
mod format;
mod material;
mod math;
mod mesh;
mod model;
mod node;
mod primitive;
mod scene;
mod texture;

use animation::*;
use data::*;
use format::*;
use material::*;
use math::*;
use mesh::*;
use model::*;
use node::*;
use primitive::*;
use scene::*;
use texture::*;

fn load_model(model_path: &Path) -> Result<(), Box<StdError>> {
    let _base_path = model_path.parent().unwrap_or(Path::new("./"));
    //let gltf_data = read_to_end(model_path)?;
    //let (gltf, gltf_buffers) = import(&gltf_data, base_path)?;
    //println!("gltf: {:?}", gltf);

    let (document, buffers, images) = match gltf::import(model_path) {
        Ok(tuple) => tuple,
        Err(_err) => {
            //error!("glTF import failed: {:?}", err);
            //if let gltf::Error::Io(_) = err {
            //error!("Hint: Are the .bin file(s) referenced by the .gltf file available?")
            //}
            //process::exit(1)
            panic!("failed to load ")
        }
    };

    /* for gltf_materal in &document.materials {
        let mat = Rc::new(GltfMaterial::from_gltf(&material_ref, model, data, path));
        model.materials.push(Rc::clone(&mat));
    }*/

    let data = GltfData {
        document,
        buffers,
        images,
    };

    let mut model = GltfModel::from_gltf(&data, model_path);

    let scene_count = data.document.scenes().len();
    let scene_index = 0;
    if scene_index >= scene_count {
        //error!("Scene index too high - file has only {} scene(s)", scene_count);
        //process::exit(3)
        panic!("scene index is too high");
    }
    println!("Scene count: {}", scene_count);

    let gltf_scene = data.document.scenes().nth(scene_index).unwrap();
    let _scene = GltfScene::from_gltf(&gltf_scene, &mut model);

    //println!("Scene: {:?} - {:?}", scene, gltf_scene);

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

    //println!("Model 1");
    //load_model(&Path::new("data/Floor_Junk_Cluster_01.glb")).expect("runtime error");
    //load_model(&"data/Combat_Helmet.glb").expect("runtime error");
    //println!("Model 2");
    //load_model(&Path::new("data/SciFiHelmet.gltf")).expect("runtime error");
    //load_model(&Path::new("data/EpicCitadel.glb")).expect("runtime error");
    //load_model(&Path::new("data/BoxAnimated.glb")).expect("runtime error");
    load_model(&Path::new("data/RiggedFigure.glb")).expect("runtime error");
    println!("Done");
}
