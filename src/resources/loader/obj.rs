use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::collections::HashMap;

use super::super::model::{Model, FromBuffers};
use super::super::super::vertex::Vertex;

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

        let mut temp_vertex_buffer: Vec<Vertex> = Vec::new();
        let mut temp_vertex_normal_buffer: Vec<[f32; 3]> = Vec::new();
        let mut temp_index_buffer: Vec<[u32; 2]> = Vec::new();

        let mut material_library: Option<MaterialLibrary> = None;
        let mut current_material: Option<ObjMaterial> = None;

        for line in file_string.lines() {
            let mut elements = line.split_ascii_whitespace();

            match elements.next() {
                Some("v") => ObjLoader::extract_vertex(&mut temp_vertex_buffer, &mut elements, &current_material),
                Some("vn") => ObjLoader::extract_vertex_normal(&mut temp_vertex_normal_buffer, &mut elements),
                Some("f") => ObjLoader::extract_face(&mut temp_index_buffer, &mut elements),
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

        // Load with normal
        let mut vertex_buffer: Vec<Vertex> = Vec::with_capacity(temp_index_buffer.len() * 3);

        for [vertex_index, vertex_normal_index] in temp_index_buffer {
            let vertex = Vertex {
                position: temp_vertex_buffer[vertex_index as usize].position,
                ambient: temp_vertex_buffer[vertex_index as usize].ambient,
                diffuse: temp_vertex_buffer[vertex_index as usize].diffuse,
                specular_exponent: temp_vertex_buffer[vertex_index as usize].specular_exponent,
                normal: temp_vertex_normal_buffer[vertex_normal_index as usize]
            };

            vertex_buffer.push(vertex);
        }

        let index_buffer: Vec<u32> = (0..vertex_buffer.len() as u32).into_iter().collect();

        let output = Model::from_buffers(vertex_buffer, index_buffer);

        Ok(output)
    }

    fn extract_vertex(vertex_buffer: &mut Vec<Vertex>, elements: &mut std::str::SplitAsciiWhitespace, current_material: &Option<ObjMaterial>) {
        let x = f32::from_str(elements.next().unwrap()).unwrap();
        let y = f32::from_str(elements.next().unwrap()).unwrap();
        let z = f32::from_str(elements.next().unwrap()).unwrap();

        if let Some(material) = current_material {
            let r = material.diffuse[0];
            let g = material.diffuse[1];
            let b = material.diffuse[2];
            vertex_buffer.push(Vertex::new_with_color(x, y, z, r, g, b));
        } else {
            vertex_buffer.push(Vertex::new(x, y, z));
        }

    }

    fn extract_vertex_normal(vertex_normal_buffer: &mut Vec<[f32; 3]>, elements: &mut std::str::SplitAsciiWhitespace) {
        let x = f32::from_str(elements.next().unwrap()).unwrap();
        let y = f32::from_str(elements.next().unwrap()).unwrap();
        let z = f32::from_str(elements.next().unwrap()).unwrap();
        vertex_normal_buffer.push([x, y, z]);

    }

    fn extract_face(index_buffer: &mut Vec<[u32; 2]>, elements: &mut std::str::SplitAsciiWhitespace) {
        let mut indices = elements.next().unwrap().split("/");
        let vertex_index1 = u32::from_str(indices.next().unwrap()).unwrap() - 1;
        indices.next();
        let vertex_normal_index1 = u32::from_str(indices.next().unwrap()).unwrap() - 1;

        let mut indices = elements.next().unwrap().split("/");
        let vertex_index2 = u32::from_str(indices.next().unwrap()).unwrap() - 1;
        indices.next();
        let vertex_normal_index2 = u32::from_str(indices.next().unwrap()).unwrap() - 1;

        let mut indices = elements.next().unwrap().split("/");
        let vertex_index3 = u32::from_str(indices.next().unwrap()).unwrap() - 1;
        indices.next();
        let vertex_normal_index3 = u32::from_str(indices.next().unwrap()).unwrap() - 1;

        index_buffer.push([vertex_index1, vertex_normal_index1]);
        index_buffer.push([vertex_index2, vertex_normal_index2]);
        index_buffer.push([vertex_index3, vertex_normal_index3]);
    }
}

#[derive(Debug, Clone)]
struct ObjMaterial {
    name: String,
    ambient: [f32; 3],
    diffuse: [f32; 3] ,
    specular_exponent: f32
}

impl ObjMaterial {}

struct ObjMaterialBuilder {
    name: Option<String>,
    diffuse: Option<[f32; 3]>,
    ambient: Option<[f32; 3]>,
    specular_exponent: Option<f32>,
}

impl ObjMaterialBuilder {
    fn start() -> Self {
        Self {
            name: None,
            ambient: None,
            diffuse: None,
            specular_exponent: None
        }
    }

    fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    fn with_diffuse_rgb(mut self, r: f32, g: f32, b: f32) -> Self {
        self.diffuse = Some([r, g, b]);
        self
    }

    fn with_ambient_rgb(mut self, r: f32, g: f32, b: f32) -> Self {
        self.ambient = Some([r, g, b]);
        self
    }

    fn with_specular_exponent(mut self, e: f32) -> Self {
        self.specular_exponent = Some(e);
        self
    }

    fn build(self) -> Result<ObjMaterial, ObjMaterialBuilderError> {
        use ObjMaterialBuilderError::{NameError, ColorError};
        if self.name.is_none() {
            return Err(NameError)
        }

        if self.diffuse.is_none() {
            return Err(ColorError)
        }

        let material = ObjMaterial {
            name: self.name.unwrap(),
            ambient: self.ambient.unwrap_or([0.0, 0.0, 0.0]),
            diffuse: self.diffuse.unwrap(),
            specular_exponent: self.specular_exponent.unwrap_or(1.0),
        };

        Ok(material)
    }

    fn is_valid(&self) -> bool {
        self.name.is_some() && self.diffuse.is_some()
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

                    material_builder = material_builder.with_diffuse_rgb(r, g, b);
                },
                Some("Ka") => {
                    let r = f32::from_str(elements.next().unwrap()).unwrap();
                    let g = f32::from_str(elements.next().unwrap()).unwrap();
                    let b = f32::from_str(elements.next().unwrap()).unwrap();

                    material_builder = material_builder.with_ambient_rgb(r, g, b);
                },
                Some("Ns") => {
                    let specular_exponent = f32::from_str(elements.next().unwrap()).unwrap();

                    material_builder = material_builder.with_specular_exponent(specular_exponent);
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