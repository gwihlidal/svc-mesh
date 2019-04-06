use gltf::{buffer::Source as BufferSource, image::Source as ImageSource, Gltf};
use std::boxed::Box;
use std::error::Error as StdError;
use std::{fs, io};

struct VertexSkinningData {}

struct Vector3 {}

struct Vector2 {}

struct Vector4 {}

struct MeshData {
    bonesAndWeights: Vec<VertexSkinningData>,
    vertexPositions: Vec<Vector3>,
    vertexTexCoords: Vec<Vector2>,
    vertexNormals: Vec<Vector3>,
    vertexTangents: Vec<Vector3>,
    vertexBitangents: Vec<Vector3>,
    vertexColors: Vec<Vector4>,
    indices: Vec<u32>,
}

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
    let gltf = Gltf::from_reader(reader)?;
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
    let gltf = Gltf::from_reader(reader)?;
    println!("{:#?}", gltf);
    Ok(())
}

fn load_model(path: &str) -> Result<(), Box<StdError>> {
    let file = fs::File::open(&path)?;
    let reader = io::BufReader::new(file);
    let gltf = Gltf::from_reader(reader)?;

    for texture in gltf.textures() {
        //let image = texture.source;
    }

    for material in gltf.materials() {
        // TODO: Simplify this expansion (too verbose)
        let material_url = if let Some(extras) = material.extras() {
            let mut material_url = String::new();
            let from_fbx = &extras["fromFBX"];
            if from_fbx.is_object() {
                let user_props = &from_fbx["userProperties"];
                if user_props.is_object() {
                    let halcyon_url = &user_props["halcyonUrl"];
                    if halcyon_url.is_object() {
                        let url_type = &halcyon_url["type"];
                        let url_value = &halcyon_url["value"];
                        if url_type.is_string() && url_value.is_string() {
                            let url_type = url_type.as_str().unwrap();
                            if url_type == "eFbxString" {
                                let url_value = url_value.as_str().unwrap();
                                material_url =
                                    format!("Content/{}.hcy-material:Material", url_value);
                            }
                        }
                    }
                }
            }
            material_url
        } else {
            String::new()
        };

        let material_name = material.name().unwrap_or(&"");

        let metallic_roughness = material.pbr_metallic_roughness();

        if let Some(base_color_texture) = metallic_roughness.base_color_texture() {
            let set_index = base_color_texture.tex_coord();
            let texture = base_color_texture.texture();
            let texture_name = texture.name().unwrap_or(&"");
            let sampler = texture.sampler();
            let image = texture.source();
            let image_name = image.name().unwrap_or(&"");
            let source = image.source();
            match source {
                ImageSource::View {
                    ref view,
                    ref mime_type,
                } => {
                    println!("Base Color Texture View: {:?} - {:?}", view, mime_type);
                }
                ImageSource::Uri {
                    ref uri,
                    ref mime_type,
                } => {
                    println!("Base Color Texture Uri: {:?} - {:?}", uri, mime_type);
                }
            }
        }

        if let Some(metallic_roughness_texture) = metallic_roughness.metallic_roughness_texture() {
            let set_index = metallic_roughness_texture.tex_coord();
            let texture = metallic_roughness_texture.texture();
            let texture_name = texture.name().unwrap_or(&"");
            let sampler = texture.sampler();
            let image = texture.source();
            let image_name = image.name().unwrap_or(&"");
            let source = image.source();
            match source {
                ImageSource::View {
                    ref view,
                    ref mime_type,
                } => {
                    println!(
                        "Metallic Roughness Texture View: {:?} - {:?}",
                        view, mime_type
                    );
                }
                ImageSource::Uri {
                    ref uri,
                    ref mime_type,
                } => {
                    println!(
                        "Metallic Roughness Texture Uri: {:?} - {:?}",
                        uri, mime_type
                    );
                }
            }
        }

        if let Some(normal_texture) = material.normal_texture() {
            let set_index = normal_texture.tex_coord();
            let texture = normal_texture.texture();
            let texture_name = texture.name().unwrap_or(&"");
            let sampler = texture.sampler();
            let image = texture.source();
            let image_name = image.name().unwrap_or(&"");
            let source = image.source();
            match source {
                ImageSource::View {
                    ref view,
                    ref mime_type,
                } => {
                    println!("Normal Texture View: {:?} - {:?}", view, mime_type);
                }
                ImageSource::Uri {
                    ref uri,
                    ref mime_type,
                } => {
                    println!("Normal Texture Uri: {:?} - {:?}", uri, mime_type);
                }
            }
        }

        if let Some(emissive_texture) = material.emissive_texture() {
            let set_index = emissive_texture.tex_coord();
            let texture = emissive_texture.texture();
            let texture_name = texture.name().unwrap_or(&"");
            let sampler = texture.sampler();
            let image = texture.source();
            let image_name = image.name().unwrap_or(&"");
            let source = image.source();
            match source {
                ImageSource::View {
                    ref view,
                    ref mime_type,
                } => {
                    println!("Emissive Texture View: {:?} - {:?}", view, mime_type);
                }
                ImageSource::Uri {
                    ref uri,
                    ref mime_type,
                } => {
                    println!("Emissive Texture Uri: {:?} - {:?}", uri, mime_type);
                }
            }
        }

        if let Some(occlusion_texture) = material.occlusion_texture() {
            let set_index = occlusion_texture.tex_coord();
            let texture = occlusion_texture.texture();
            let texture_name = texture.name().unwrap_or(&"");
            let sampler = texture.sampler();
            let image = texture.source();
            let image_name = image.name().unwrap_or(&"");
            let source = image.source();
            match source {
                ImageSource::View {
                    ref view,
                    ref mime_type,
                } => {
                    println!("Occlusion Texture View: {:?} - {:?}", view, mime_type);
                }
                ImageSource::Uri {
                    ref uri,
                    ref mime_type,
                } => {
                    println!("Occlusion Texture Uri: {:?} - {:?}", uri, mime_type);
                }
            }
        }

        let base_color_factor = metallic_roughness.base_color_factor();

        // A value of 1.0 means the material is completely rough.
        // A value of 0.0 means the material is completely smooth.
        let roughness_factor = metallic_roughness.roughness_factor();

        let metallic_factor = metallic_roughness.metallic_factor();

        let emissive_factor = material.emissive_factor();

        /*if let Some(specular_glossiness) = material.pbr_specular_glossiness() {

        }*/

        // The alpha rendering mode of the material.
        let alpha_mode = material.alpha_mode();
        match alpha_mode {
            gltf::material::AlphaMode::Opaque => {
                //The alpha value is ignored and the rendered output is fully opaque.
            }
            gltf::material::AlphaMode::Mask => {
                // The rendered output is either fully opaque or fully transparent depending on the alpha value and the specified alpha cutoff value.
            }
            gltf::material::AlphaMode::Blend => {
                // The rendered output is either fully opaque or fully transparent depending on the alpha value and the specified alpha cutoff value.
            }
        }

        // The alpha cutoff value of the material.
        let alpha_cutoff = material.alpha_cutoff();

        println!("Material - Name: {}, Url: {}", material_name, material_url);
    }

    for scene in gltf.scenes() {
        for node in scene.nodes() {}
    }

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

    load_model(&"data/Floor_Junk_Cluster_01.glb").expect("runtime error");
    //load_model(&"data/Combat_Helmet.glb").expect("runtime error");
    load_model(&"data/SciFiHelmet.gltf").expect("runtime error");
}
