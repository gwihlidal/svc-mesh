//use gltf::{buffer::Source as BufferSource, image::Source as ImageSource, Gltf};
use std::path::Path;
//use std::rc::Rc;

mod animation;
mod data;
mod error;
mod format;
mod material;
mod math;
mod mesh;
mod model;
mod node;
mod primitive;
mod scene;
mod tangents;
mod texture;

use animation::*;
use data::*;
//use format::*;
use error::*;
use material::*;
use math::*;
use mesh::*;
use model::*;
use node::*;
use primitive::*;
use scene::*;
use tangents::*;
use texture::*;

#[derive(Debug, Clone, Default)]
pub struct GltfOptions {
    pub scene_index: Option<usize>,
    pub load_animations: bool,
    pub regenerate_tangents: bool,
    pub generate_tex_coords: (f32, f32),
    pub flip_v_coord: bool,
}

fn load_model(model_path: &Path) -> Result<()> {
    let _base_path = model_path.parent().unwrap_or(Path::new("./"));
    //let gltf_data = read_to_end(model_path)?;
    //let (gltf, gltf_buffers) = import(&gltf_data, base_path)?;
    //println!("gltf: {:?}", gltf);

    let (document, buffers, images) = gltf::import(model_path)?;

    /* for gltf_materal in &document.materials {
        let mat = Rc::new(GltfMaterial::from_gltf(&material_ref, model, data, path));
        model.materials.push(Rc::clone(&mat));
    }*/

    let options = GltfOptions {
        regenerate_tangents: true,
        ..Default::default()
    };

    let data = GltfData {
        options,
        document,
        buffers,
        images,
    };

    let mut model = GltfModel::from_gltf(&data, model_path)?;

    let scene_count = data.document.scenes().len();
    let scene_index = 0;
    if scene_index >= scene_count {
        //error!("Scene index too high - file has only {} scene(s)", scene_count);
        //process::exit(3)
        panic!("scene index is too high");
    }
    //println!("Scene count: {}", scene_count);

    let gltf_scene = data.document.scenes().nth(scene_index).unwrap();
    let scene = GltfScene::from_gltf(&gltf_scene, &mut model)?;
    println!("Scene Dimensions: {:?}", scene.dimensions);

    let _has_animations = model.animations.len() > 0;

    // Protect against shared triangle vertices being transformed multiple times
    //let mut transformed: Vec<bool> = vec![false; model.vertex_buffer.len()];


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
    load_model(&Path::new("data/Floor_Junk_Cluster_01.glb")).expect("runtime error");
    //load_model(&Path::new("data/Combat_Helmet.glb")).expect("runtime error");
    //println!("Model 2");
    //load_model(&Path::new("data/SciFiHelmet.gltf")).expect("runtime error");
    //load_model(&Path::new("data/EpicCitadel.glb")).expect("runtime error");
    //load_model(&Path::new("data/BoxAnimated.glb")).expect("runtime error");
    //load_model(&Path::new("data/RiggedFigure.glb")).expect("runtime error");
    println!("Done");
}
