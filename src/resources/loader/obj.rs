use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use super::super::model::{Model, FromBuffers};
use super::super::super::Vertex;

pub struct ObjLoader {}

#[derive(Debug)]
pub enum LoadError {
    IoError(std::io::Error),
    Other
}

impl From<std::io::Error> for LoadError {
    fn from(error: std::io::Error) -> Self {
        LoadError::IoError(error)
    }
}

impl ObjLoader {
    pub fn load(filepath: &Path) -> Result<Model, LoadError> {
        let mut file = File::open(filepath)?;

        let mut file_string = String::new();
        file.read_to_string(&mut file_string)?;

        let mut vertex_buffer: Vec<Vertex> = Vec::new();
        let mut index_buffer: Vec<u32> = Vec::new();

        for line in file_string.lines() {
            let mut elements = line.split_ascii_whitespace();

            match elements.next() {
                Some("v") => ObjLoader::extract_vertex(&mut vertex_buffer, &mut elements),
                Some("f") => ObjLoader::extract_face(&mut index_buffer, &mut elements),
                None => continue,
                _ => {}
            }
        }

        let output = Model::from_buffers(vertex_buffer, index_buffer);

        Ok(output)
    }

    fn extract_vertex(vertex_buffer: &mut Vec<Vertex>, elements: &mut std::str::SplitAsciiWhitespace) {
        let x = f32::from_str(elements.next().unwrap()).unwrap();
        let y = f32::from_str(elements.next().unwrap()).unwrap();
        let z = f32::from_str(elements.next().unwrap()).unwrap();

        vertex_buffer.push(Vertex::new(x, y, z));
    }

    fn extract_face(index_buffer: &mut Vec<u32>, elements: &mut std::str::SplitAsciiWhitespace) {
        let index1 = u32::from_str(elements.next().unwrap().split("/").next().unwrap()).unwrap() - 1;
        let index2 = u32::from_str(elements.next().unwrap().split("/").next().unwrap()).unwrap() - 1;
        let index3 = u32::from_str(elements.next().unwrap().split("/").next().unwrap()).unwrap() - 1;

        index_buffer.push(index1);
        index_buffer.push(index2);
        index_buffer.push(index3);
    }
}