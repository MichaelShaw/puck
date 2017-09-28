use std::path::PathBuf;
use std::path::Path;


use std::fs::{self, File};
use std::io::{self, Read};


use render::shader::ShaderPair;
use render::TextureDirectory;

use PuckResult;

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
            } else if path_contains(self.texture_directory.path.as_path(), path) {
                reload.texture = true;
            } else if path_contains(self.sound_directory.as_path(), path) {
                reload.sound = true;
            } else if path_contains(self.font_directory.as_path(), path) {
                reload.font = true;
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


pub fn path_contains(parent_path:&Path, path:&Path) -> bool {
    use std::path;
    let my_components : Vec<path::Component> = parent_path.components().collect();
    let components : Vec<path::Component> = path.components().collect();

    components.windows(my_components.len()).position(|window| {
        window == &my_components[..]
    }).is_some()
}

pub fn read_directory_paths(path:&Path) -> PuckResult<Vec<PathBuf>> {
    let mut paths : Vec<PathBuf> = Vec::new();

    for entry in try!(fs::read_dir(path)) {
        let entry = try!(entry);
        let file_path = entry.path().to_path_buf();
        paths.push(file_path);
    }

    Ok(paths)
}

pub fn load_file_contents(path:&Path) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer : Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}