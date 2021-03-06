use crate::GltfData;
use crate::GltfIndex;
use crate::Result;
use crate::Vector3;
use crate::Vector4;
use std::path::Path;

#[derive(Debug)]
pub struct GltfMaterial {
    pub index: Option<GltfIndex>,
    pub name: Option<String>,

    pub material_uri: String,

    // pbr_metallic_roughness properties
    pub base_color_factor: Vector4,
    pub base_color_texture: Option<(GltfIndex, u32 /* uv set */)>,

    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub metallic_roughness_texture: Option<(GltfIndex, u32 /* uv set */)>,

    pub normal_scale: Option<f32>,
    pub normal_texture: Option<(GltfIndex, u32 /* uv set */)>,

    pub occlusion_strength: Option<f32>,
    pub occlusion_texture: Option<(GltfIndex, u32 /* uv set */)>,

    pub emissive_factor: Vector3,
    pub emissive_texture: Option<(GltfIndex, u32 /* uv set */)>,

    pub alpha_cutoff: f32,
    pub alpha_mode: gltf::material::AlphaMode,

    pub double_sided: bool,
}

impl GltfMaterial {
    pub fn from_gltf(
        material_ref: &gltf::material::Material<'_>,
        _data: &GltfData,
        _path: &Path,
    ) -> Result<GltfMaterial> {
        // TODO: Simplify this expansion (too verbose)
        // TODO: Make this generic
        let material_uri = if let Some(ref extras) = material_ref.extras() {
            let mut material_uri = String::new();
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
                                material_uri =
                                    format!("Content/{}.hcy-material:Material", url_value);
                            }
                        }
                    }
                }
            }
            material_uri
        } else {
            String::new()
        };

        let pbr = material_ref.pbr_metallic_roughness();

        let mut material = GltfMaterial {
            index: material_ref.index(), // None is returned if it's the default material
            name: material_ref.name().map(|s| s.into()),
            material_uri,
            base_color_factor: pbr.base_color_factor().into(),
            base_color_texture: None,
            metallic_factor: pbr.metallic_factor(),

            // A value of 1.0 means the material is completely rough.
            // A value of 0.0 means the material is completely smooth.
            roughness_factor: pbr.roughness_factor(),
            metallic_roughness_texture: None,

            normal_texture: None,
            normal_scale: None,

            occlusion_texture: None,
            occlusion_strength: None,

            emissive_factor: material_ref.emissive_factor().into(),
            emissive_texture: None,

            // The alpha cutoff value of the material.
            alpha_cutoff: material_ref.alpha_cutoff(),

            // The alpha rendering mode of the material.
            alpha_mode: material_ref.alpha_mode(),

            double_sided: material_ref.double_sided(),
        };

        /*match material.alpha_mode {
            gltf::material::AlphaMode::Opaque => {
            //The alpha value is ignored and the rendered output is fully opaque.
            }
            gltf::material::AlphaMode::Mask => {
            // The rendered output is either fully opaque or fully transparent depending on the alpha value and the specified alpha cutoff value.
            }
            gltf::material::AlphaMode::Blend => {
            // The rendered output is either fully opaque or fully transparent depending on the alpha value and the specified alpha cutoff value.
            }
        }*/

        if let Some(color_info) = pbr.base_color_texture() {
            material.base_color_texture =
                Some((color_info.texture().index(), color_info.tex_coord()));
        }

        if let Some(mr_info) = pbr.metallic_roughness_texture() {
            material.metallic_roughness_texture =
                Some((mr_info.texture().index(), mr_info.tex_coord()));
        }

        if let Some(normal_texture) = material_ref.normal_texture() {
            material.normal_texture =
                Some((normal_texture.texture().index(), normal_texture.tex_coord()));
            material.normal_scale = Some(normal_texture.scale());
        }

        if let Some(occ_texture) = material_ref.occlusion_texture() {
            material.occlusion_texture =
                Some((occ_texture.texture().index(), occ_texture.tex_coord()));
            material.occlusion_strength = Some(occ_texture.strength());
        }

        if let Some(em_info) = material_ref.emissive_texture() {
            material.emissive_texture = Some((em_info.texture().index(), em_info.tex_coord()));
        }

        Ok(material)
    }
}
