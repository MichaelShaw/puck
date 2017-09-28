use std::path::{PathBuf, Path};


use load_file_contents;

use PuckResult;

#[derive(Debug, Eq, PartialEq)]
pub struct ShaderPair {
    pub vertex_path: PathBuf,
    pub fragment_path: PathBuf,
}

impl ShaderPair {
    pub fn contains(&self, path:&Path) -> bool {
        path.ends_with(&self.vertex_path) || path.ends_with(&self.fragment_path)
    }

    pub fn for_paths(vertex_path: &str, fragment_path: &str) -> ShaderPair {
        ShaderPair {
            vertex_path: PathBuf::from(vertex_path),
            fragment_path: PathBuf::from(fragment_path),
        }
    }

    pub fn load(&self) -> PuckResult<ShaderData> {
        let vertex_data = load_file_contents(&self.vertex_path)?;
        let fragment_data = load_file_contents(&self.fragment_path)?;

        Ok(ShaderData {
            vertex_data,
            fragment_data,
        })
    }
}

pub struct ShaderData {
    pub vertex_data: Vec<u8>,
    pub fragment_data: Vec<u8>,
}