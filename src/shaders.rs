pub mod basic_vertex_shader {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "res/shaders/basic.vs"
    }
}

pub mod basic_fragment_shader {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "res/shaders/basic.fs"
    }
}