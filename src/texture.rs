use crate::GltfData;
use crate::GltfIndex;
//use crate::Vector3;
//use crate::Vector4;
use std::path::Path;

#[derive(Debug)]
pub struct GltfTexture {
    pub index: GltfIndex,
    pub name: Option<String>,

    //pub id: u32, // OpenGL id
    pub tex_coord: u32, // the tex coord set to use
}

impl GltfTexture {
    pub fn from_gltf(
        texture_ref: &gltf::Texture<'_>,
        tex_coord: u32,
        data: &GltfData,
        path: &Path,
    ) -> GltfTexture {
        let _buffers = &data.buffers;
        let _image_ref = texture_ref.source();
        GltfTexture {
            index: texture_ref.index(),
            name: texture_ref.name().map(|s| s.into()),
            tex_coord,
        }
    }
}
