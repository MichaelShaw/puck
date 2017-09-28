use std::path::{PathBuf};
use std::fmt;

use image;
use image::{GenericImage, RgbaImage};

use puck_core::HashSet;
use read_directory_paths;

use PuckResult;
use PuckError;

#[derive(Debug)]
pub struct TextureDirectory {
    pub path: PathBuf,
    pub extensions: HashSet<String>,
}

impl TextureDirectory {
    pub fn for_path(path:&str, extensions: HashSet<String>) -> TextureDirectory {
        TextureDirectory {
            path: PathBuf::from(path), // convert to absolute here?
            extensions: extensions.iter().map(|s| s.to_lowercase()).collect(),
        }
    }

    pub fn load(&self) -> PuckResult<TextureArrayData> {
        let mut images : Vec<RgbaImage> = Vec::new();

        let mut dimensions : Option<Dimensions> = None;

        let mut paths = read_directory_paths(&self.path)?;
        paths.sort();

        println!("sorted paths -> {:?}", paths);

        for path in paths {
            if let Some(extension) = path.extension().and_then(|p| p.to_str()).map(|s|s.to_lowercase()) {
                // let ext : String = extension.into();
                if self.extensions.contains(&extension) {
                    println!("path -> {:?} with extension -> {:?}", path, extension);
                    let img = image::open(path.clone())?;

                    let d = img.dimensions();
                    let w = d.0 as u32;
                    let h = d.1 as u32;

                    if let Some(ed) = dimensions {
                        if ed != (w, h) {
                            return Err(PuckError::MismatchingDimensions);
                        }
                    } else {
                        dimensions = Some((w, h));
                    }

                    images.push(img.to_rgba());
                }
            }
        }

        if let Some((w, h))  = dimensions {
            Ok(TextureArrayData {
                dimensions: TextureArrayDimensions {
                    width: w,
                    height: h,
                    layers: images.len() as u32,
                },
                images: images,
            })
        } else {
            Err(PuckError::NoFiles)
        }
    }


}


type Dimensions = (u32, u32); // rename this as TextureDimensions?

// hrm, we currently load it all in to ram in uncompressed form :-/ zero reason why this isn't streamed in as a whole
#[derive(Clone)]
pub struct TextureArrayData {
    pub dimensions : TextureArrayDimensions,
    pub images: Vec<RgbaImage>,
}

impl fmt::Debug for TextureArrayData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextureArrayData {{  dimensions: {:?}, data: {} }}", self.dimensions, self.images.len())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TextureArrayDimensions {
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}