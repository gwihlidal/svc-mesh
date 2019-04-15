use crate::StdError;
use std::path::Path;

#[derive(Debug)]
pub struct GltfData {
    pub document: gltf::Document,
    pub buffers: Vec<gltf::buffer::Data>,
    pub images: Vec<gltf::image::Data>,
}

pub type GltfIndex = usize;

#[derive(Debug)]
pub enum ImageFormat {
    Png,
    Jpeg,
}

impl ImageFormat {
    fn from_mime_type(mime: &str) -> Self {
        match mime {
            "image/jpeg" => ImageFormat::Jpeg,
            "image/png" => ImageFormat::Png,
            _ => unreachable!(),
        }
    }
}

pub fn get_image_data(
    image: &gltf::Image<'_>,
    buffers: &GltfBuffers,
    base_path: &Path,
) -> Result<(Vec<u8>, ImageFormat), Box<StdError>> {
    use gltf::image::Source;
    match image.source() {
        Source::View { view, mime_type } => {
            let data = buffers
                .view(&view)
                .expect("`view` of image data points to a buffer which does not exist");
            Ok((data.to_vec(), ImageFormat::from_mime_type(mime_type)))
        }

        Source::Uri { uri, mime_type } => {
            if uri.starts_with("data:") {
                let data = parse_data_uri(uri)?;
                if let Some(ty) = mime_type {
                    Ok((data, ImageFormat::from_mime_type(ty)))
                } else {
                    let mimetype = uri
                        .split(',')
                        .nth(0)
                        .expect("Unreachable: `split` will always return at least one element")
                        .split(':')
                        .nth(1)
                        .expect("URI does not contain ':'")
                        .split(';')
                        .nth(0)
                        .expect("Unreachable: `split` will always return at least one element");
                    Ok((data, ImageFormat::from_mime_type(mimetype)))
                }
            } else {
                let path = base_path.parent().unwrap_or(Path::new("./")).join(uri);
                let data = load_data(&path)?;
                if let Some(ty) = mime_type {
                    Ok((data, ImageFormat::from_mime_type(ty)))
                } else {
                    let ext = path
                        .extension()
                        .and_then(|s| s.to_str())
                        .map_or("".to_string(), |s| s.to_ascii_lowercase());
                    let format = match &ext[..] {
                        "jpg" | "jpeg" => ImageFormat::Jpeg,
                        "png" => ImageFormat::Png,
                        _ => unreachable!(),
                    };
                    Ok((data, format))
                }
            }
        }
    }
}

/// Buffer data returned from `import`.
#[derive(Clone, Debug)]
pub struct GltfBuffers(Vec<Vec<u8>>);

#[allow(unused)]
impl GltfBuffers {
    /// Obtain the contents of a loaded buffer.
    pub fn buffer(&self, buffer: &gltf::Buffer<'_>) -> Option<&[u8]> {
        self.0.get(buffer.index()).map(Vec::as_slice)
    }

    /// Obtain the contents of a loaded buffer view.
    pub fn view(&self, view: &gltf::buffer::View<'_>) -> Option<&[u8]> {
        self.buffer(&view.buffer()).map(|data| {
            let begin = view.offset();
            let end = begin + view.length();
            &data[begin..end]
        })
    }

    /// Take the loaded buffer data.
    pub fn take(self) -> Vec<Vec<u8>> {
        self.0
    }
}

fn load_external_buffers(
    base_path: &Path,
    gltf: &gltf::Gltf,
    mut bin: Option<Vec<u8>>,
) -> Result<Vec<Vec<u8>>, Box<StdError>> {
    use gltf::buffer::Source;
    let mut buffers = vec![];
    for (index, buffer) in gltf.buffers().enumerate() {
        let data = match buffer.source() {
            Source::Uri(uri) => {
                if uri.starts_with("data:") {
                    parse_data_uri(uri)?
                } else {
                    let path = base_path.parent().unwrap_or(Path::new("./")).join(uri);
                    load_data(&path)?
                }
            }
            Source::Bin => bin
                .take()
                .expect("`BIN` section of binary glTF file is empty or used by another buffer"),
        };

        if data.len() < buffer.length() {
            //let path = json::Path::new().field("buffers").index(index);
            //return Err(error::Error::BufferLength(path).into());
            panic!("invalid buffer length!");
        }
        buffers.push(data);
    }
    Ok(buffers)
}

pub fn load_data<P>(path: P) -> Result<Vec<u8>, Box<StdError>>
where
    P: AsRef<Path>,
{
    use std::io::Read;

    let mut v = Vec::new();
    let mut file = std::fs::File::open(&path)?;
    file.read_to_end(&mut v)?;

    Ok(v)
}

fn import_standard(
    data: &[u8],
    base_path: &Path,
) -> Result<(gltf::Gltf, GltfBuffers), Box<StdError>> {
    let gltf = gltf::Gltf::from_slice(data)?;
    let buffers = GltfBuffers(load_external_buffers(base_path, &gltf, None)?);
    Ok((gltf, buffers))
}

fn import_binary(
    data: &[u8],
    base_path: &Path,
) -> Result<(gltf::Gltf, GltfBuffers), Box<StdError>> {
    let gltf::binary::Glb {
        header: _,
        json,
        bin,
    } = gltf::binary::Glb::from_slice(data)?;
    let gltf = gltf::Gltf::from_slice(&json)?;
    let bin = bin.map(|x| x.to_vec());
    let buffers = GltfBuffers(load_external_buffers(base_path, &gltf, bin)?);
    Ok((gltf, buffers))
}

pub fn _import<P>(data: &[u8], path: P) -> Result<(gltf::Gltf, GltfBuffers), Box<StdError>>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if data.starts_with(b"glTF") {
        import_binary(&data, path)
    } else {
        import_standard(&data, path)
    }
}

fn parse_data_uri(uri: &str) -> Result<Vec<u8>, Box<StdError>> {
    let encoded = uri.split(",").nth(1).expect("URI does not contain ','");
    let decoded = base64::decode(&encoded)?;
    Ok(decoded)
}
