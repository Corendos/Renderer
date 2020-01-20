use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::collections::HashMap;

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

        let mut material_library: Option<MaterialLibrary> = None;
        let mut current_material: Option<ObjMaterial> = None;

        for line in file_string.lines() {
            let mut elements = line.split_ascii_whitespace();

            match elements.next() {
                Some("v") => ObjLoader::extract_vertex(&mut vertex_buffer, &mut elements, &current_material),
                Some("f") => ObjLoader::extract_face(&mut index_buffer, &mut elements),
                Some("usemtl") => {
                    let material_name = elements.next().unwrap();

                    if let Some(library) = &material_library {
                        if let Some(material) = library.map.get(&String::from(material_name)) {
                            current_material = Some(material.clone());
                        }
                    }
                },
                Some("mtllib") => {
                    let filename = elements.next().unwrap();
                    
                    let full_path = filepath.parent().unwrap().join(filename);

                    let library = MaterialLibrary::load(full_path.as_path()).unwrap();
                    material_library = Some(library);
                },
                None => continue,
                _ => {}
            }
        }

        let output = Model::from_buffers(vertex_buffer, index_buffer);

        Ok(output)
    }

    fn extract_vertex(vertex_buffer: &mut Vec<Vertex>, elements: &mut std::str::SplitAsciiWhitespace, current_material: &Option<ObjMaterial>) {
        let x = f32::from_str(elements.next().unwrap()).unwrap();
        let y = f32::from_str(elements.next().unwrap()).unwrap();
        let z = f32::from_str(elements.next().unwrap()).unwrap();

        if let Some(material) = current_material {
            let r = material.color[0];
            let g = material.color[1];
            let b = material.color[2];
            vertex_buffer.push(Vertex::new_with_color(x, y, z, r, g, b));
        } else {
            vertex_buffer.push(Vertex::new(x, y, z));
        }

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

#[derive(Debug, Clone)]
struct ObjMaterial {
    name: String,
    color: [f32; 3] 
}

impl ObjMaterial {
    fn new(name: String, r: f32, g: f32, b: f32) -> Self {
        Self {
            name,
            color: [r, g, b]
        }
    }
}

struct ObjMaterialBuilder {
    name: Option<String>,
    color: Option<[f32; 3]>
}

impl ObjMaterialBuilder {
    fn start() -> Self {
        Self {
            name: None,
            color: None
        }
    }

    fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    fn with_color(mut self, color: [f32; 3]) -> Self {
        self.color = Some(color);
        self
    }

    fn with_color_rgb(mut self, r: f32, g: f32, b: f32) -> Self {
        self.color = Some([r, g, b]);
        self
    }

    fn build(self) -> Result<ObjMaterial, ObjMaterialBuilderError> {
        use ObjMaterialBuilderError::{NameError, ColorError};
        if self.name.is_none() {
            return Err(NameError)
        }

        if self.color.is_none() {
            return Err(ColorError)
        }

        let material = ObjMaterial {
            name: self.name.unwrap(),
            color: self.color.unwrap(),
        };

        Ok(material)
    }

    fn is_valid(&self) -> bool {
        self.name.is_some() && self.color.is_some()
    }
}

#[derive(Debug)]
enum ObjMaterialBuilderError {
    NameError,
    ColorError
}

#[derive(Debug)]
struct MaterialLibrary {
    map: HashMap<String, ObjMaterial>,
}

impl MaterialLibrary {
    fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    fn load(path: &Path) -> Result<MaterialLibrary, LoadError> {
        let mut file = File::open(path)?;

        let mut file_string = String::new();
        file.read_to_string(&mut file_string)?;

        let mut material_builder = ObjMaterialBuilder::start();

        let mut material_library = MaterialLibrary::new();

        for line in file_string.lines() {
            let mut elements = line.split_ascii_whitespace();

            match elements.next() {
                Some("newmtl") => {
                    if material_builder.is_valid() {
                        let material = material_builder.build().unwrap();
                        material_library.map.insert(material.name.clone(), material);
                        material_builder = ObjMaterialBuilder::start();
                    }
                    
                    let name = String::from(elements.next().unwrap());
                    material_builder = material_builder.with_name(name);
                },
                Some("Kd") => {
                    let r = f32::from_str(elements.next().unwrap()).unwrap();
                    let g = f32::from_str(elements.next().unwrap()).unwrap();
                    let b = f32::from_str(elements.next().unwrap()).unwrap();

                    material_builder = material_builder.with_color_rgb(r, g, b);
                }
                _ => {}
            }
        }

        if material_builder.is_valid() {
            let material = material_builder.build().unwrap();
            material_library.map.insert(material.name.clone(), material);
        }

        Ok(material_library)
    }
}