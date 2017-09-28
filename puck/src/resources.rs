use std::path::PathBuf;

use render::shader::ShaderPair;
use render::TextureDirectory;

pub struct FileResources {
    pub resources: PathBuf,
    pub shader_pair : ShaderPair,
    pub texture_directory: TextureDirectory,
    pub font_directory: PathBuf,
    pub sound_directory: PathBuf,
}

impl FileResources {
    pub fn check_reload(&self, paths: &[PathBuf]) -> Reload {
        let mut reload = Reload::none();

        for path in paths {
            if self.shader_pair.contains(&path) {
                reload.shader = true;
            } else if self.texture_directory.contains(&path) {
                reload.texture = true;
            }
        }

        reload
    }
}

pub struct Reload {
    pub shader: bool,
    pub texture: bool,
    pub font: bool,
    pub sound: bool,
}

impl Reload {
    pub fn none() -> Reload {
        Reload {
            shader: false,
            texture: false,
            font: false,
            sound: false,
        }
    }
}