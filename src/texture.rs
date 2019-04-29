use crate::GltfData;
use crate::GltfIndex;
//use crate::GltfModel;
//use crate::Vector3;
//use crate::Vector4;
use std::path::Path;

#[derive(Debug)]
pub struct GltfSampler {}

#[derive(Debug)]
pub struct GltfTexture {
    pub index: GltfIndex,
    pub name: Option<String>,
    pub sampler: GltfSampler,
    //pub tex_coord: u32, // the tex coord set to use
}

impl GltfTexture {
    pub fn from_gltf(
        texture_ref: &gltf::Texture<'_>,
        data: &GltfData,
        _base_path: &Path,
    ) -> GltfTexture {
        let _buffers = &data.buffers;
        let _image_ref = texture_ref.source();

        let texture_name = if let Some(ref name) = texture_ref.name() {
            Some(name.to_owned().to_string())
        } else {
            None
        };

        GltfTexture {
            index: texture_ref.index(),
            name: texture_name,
            sampler: GltfSampler {},
        }
    }
}

/*
pub fn load_texture(
    default_name: &str,
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
        default_name,
        texture_ref,
        tex_coord,
        data,
        base_path,
    ));
    model.textures.push(Rc::clone(&texture));
    println!("Texture: {:?}", texture);
    texture
}
*/
