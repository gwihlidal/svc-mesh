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
use error::*;
use format::*;
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

    let has_animations = model.animations.len() > 0;

    // Protect against shared triangle vertices being transformed multiple times
    let mut transformed: Vec<bool> = vec![false; model.vertex_buffer.len()];

    // Map from name of node to indices of all parts owned by that node in the global parts array
    //std::map<std::string, std::vector<uint32>> partMap;

    /*asset->parts.reserve(model.linearNodes.size());
    for (const auto& linearNode : model.linearNodes)
    {
        if (linearNode->mesh == nullptr)
            continue;

        glm::mat4 matrix = scale * linearNode->getMatrix();

        MeshAssetPart part{};
        part.baseTransform = toMatrix44(glm::mat4(1.0f));
        part.name = linearNode->name;

        std::vector<uint32> partIndices;

        for (const auto& primitive : linearNode->mesh->primitives)
        {
            part.firstIndex = primitive->firstIndex;
            part.indexCount = primitive->indexCount;
            part.materialIndex = primitive->materialIndex;

            if (hasAnimations)
                part.animationType = linearNode->skinIndex > -1 ? AnimationType::Skinned : AnimationType::Rigid;

            if (flatten_transforms)
            {
                for (uint32 index = part.firstIndex; index < part.firstIndex + part.indexCount; index += 3)
                {
                    const uint32 i0 = model.indexBuffer[index + 0];
                    const uint32 i1 = model.indexBuffer[index + 1];
                    const uint32 i2 = model.indexBuffer[index + 2];

                if (!transformed[i0])
                {
                    const glm::vec4 pos0 = glm::vec4(model.vertexBuffer[i0].pos, 1.0f);
                    model.vertexBuffer[i0].pos = glm::vec3(matrix * pos0);

                    const glm::vec4 norm0 = glm::vec4(model.vertexBuffer[i0].normal, 1.0f);
                    model.vertexBuffer[i0].normal = glm::normalize(glm::vec3(matrix * norm0));

                    transformed[i0] = true;
                }

                if (!transformed[i1])
                {
                    const glm::vec4 pos1 = glm::vec4(model.vertexBuffer[i1].pos, 1.0f);
                    model.vertexBuffer[i1].pos = glm::vec3(matrix * pos1);

                    const glm::vec4 norm1 = glm::vec4(model.vertexBuffer[i1].normal, 1.0f);
                    model.vertexBuffer[i1].normal = glm::normalize(glm::vec3(matrix * norm1));

                    transformed[i1] = true;
                }

                if (!transformed[i2])
                {
                    const glm::vec4 pos2 = glm::vec4(model.vertexBuffer[i2].pos, 1.0f);
                    model.vertexBuffer[i2].pos = glm::vec3(matrix * pos2);

                    const glm::vec4 norm2 = glm::vec4(model.vertexBuffer[i2].normal, 1.0f);
                    model.vertexBuffer[i2].normal = glm::normalize(glm::vec3(matrix * norm2));

                        transformed[i2] = true;
                    }
                }
            }

            asset->parts.push_back(part);
            partIndices.push_back(asset->parts.size() - 1);
        }

        partMap[linearNode->name] = partIndices;
    }*/

    let mut mesh_data = MeshData::default();

    mesh_data.indices = model.index_buffer.clone();

    mesh_data.positions.reserve(model.vertex_buffer.len());
    mesh_data.tex_coords.reserve(model.vertex_buffer.len());
    mesh_data.normals.reserve(model.vertex_buffer.len());
    mesh_data.tangents.reserve(model.vertex_buffer.len());
    mesh_data.bitangents.reserve(model.vertex_buffer.len());
    mesh_data.colors.reserve(model.vertex_buffer.len());
    mesh_data.indices.reserve(model.index_buffer.len());
    if has_animations {
        mesh_data.skinning_data.reserve(model.vertex_buffer.len());
    }

    for i in 0..model.vertex_buffer.len() {
        let vertex = &model.vertex_buffer[i];
        mesh_data.positions.push(Vector3::new(
            vertex.position[0],
            vertex.position[1],
            vertex.position[2],
        ));
        mesh_data
            .tex_coords
            .push(Vector2::new(vertex.uv0[0], vertex.uv0[1]));
        mesh_data.normals.push(Vector3::new(
            vertex.normal[0],
            vertex.normal[1],
            vertex.normal[2],
        ));
        mesh_data.tangents.push(Vector3::new(
            vertex.tangent[0],
            vertex.tangent[1],
            vertex.tangent[2],
        ));
        mesh_data.bitangents.push(Vector3::new(
            vertex.bitangent[0],
            vertex.bitangent[1],
            vertex.bitangent[2],
        ));
        mesh_data.colors.push(Vector4::new(
            vertex.color0[0],
            vertex.color0[1],
            vertex.color0[2],
            vertex.color0[3],
        ));

        if has_animations {
            // TODO
        }
    }

    if has_animations {
        // export the animations
        // TODO
    }

    if has_animations {
        // export bone offsets and per vertex skinning data
        // TODO
    }

    if mesh_data.positions.is_empty() || mesh_data.indices.is_empty() {
        dbg!(mesh_data.indices);
        return Err(Error::memory("no vertices found"));
    }

    // Calculate bounding box
    //Vector3 minPos, maxPos;
    //buildBoundingBox(meshData.vertexPositions, *asset);
    // TODO: Should be able to use model.dimensions

    // Setup streams
    // TODO

    // Setup materials
    // TODO

    // Serialize asset
    // TODO

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
