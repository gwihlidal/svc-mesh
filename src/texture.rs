use crate::GltfData;
use crate::GltfIndex;
use crate::GltfModel;
//use crate::Vector3;
//use crate::Vector4;
use std::path::Path;
use std::rc::Rc;

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
        _path: &Path,
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

pub fn load_texture(
    texture_ref: &gltf::texture::Texture<'_>,
    tex_coord: u32,
    model: &mut GltfModel,
    data: &GltfData,
    base_path: &Path,
) -> Rc<GltfTexture> {
    if let Some(tex) = model
        .textures
        .iter()
        .find(|tex| (***tex).index == texture_ref.index())
    {
        return Rc::clone(tex);
    }

    let texture = Rc::new(GltfTexture::from_gltf(
        texture_ref,
        tex_coord,
        data,
        base_path,
    ));
    model.textures.push(Rc::clone(&texture));
    texture
}
