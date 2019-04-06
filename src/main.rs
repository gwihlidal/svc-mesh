use std::boxed::Box;
use std::error::Error as StdError;
use std::{fs, io};

fn print_tree(node: &gltf::Node, depth: i32) {
    for _ in 0..(depth - 1) {
        print!("  ");
    }
    print!(" -");
    print!(" Node {}", node.index());
    #[cfg(feature = "names")]
    print!(" ({})", node.name().unwrap_or("<Unnamed>"));
    println!();

    for child in node.children() {
        print_tree(&child, depth + 1);
    }
}

fn list(path: &str) -> Result<(), Box<StdError>> {
    let file = fs::File::open(&path)?;
    let reader = io::BufReader::new(file);
    let gltf = gltf::Gltf::from_reader(reader)?;
    for scene in gltf.scenes() {
        print!("Scene {}", scene.index());
        #[cfg(feature = "names")]
        print!(" ({})", scene.name().unwrap_or("<Unnamed>"));
        println!();
        for node in scene.nodes() {
            print_tree(&node, 1);
        }
    }
    Ok(())
}

fn display(path: &str) -> Result<(), Box<StdError>> {
    let file = fs::File::open(&path)?;
    let reader = io::BufReader::new(file);
    let gltf = gltf::Gltf::from_reader(reader)?;
    println!("{:#?}", gltf);
    Ok(())
}

fn main() {
    //list(&"data/Book_03.glb").expect("runtime error");
    //list(&"data/BoxAnimated.glb").expect("runtime error");
    //list(&"data/Combat_Helmet.glb").expect("runtime error");
    //list(&"data/EpicCitadel.glb").expect("runtime error");
    //list(&"data/Floor_Junk_Cluster_01.glb").expect("runtime error");
    //list(&"data/Machine_01.glb").expect("runtime error");
    list(&"data/RiggedFigure.glb").expect("runtime error");
    //list(&"data/Warrok.glb").expect("runtime error");

    //display(&"data/RiggedFigure.glb").expect("runtime error");
}
