pub mod basic {
    pub mod vertex {
        vulkano_shaders::shader! {
            ty: "vertex",
            path: "res/shaders/basic.vs"
        }
    }
    
    pub mod fragment {
        vulkano_shaders::shader! {
            ty: "fragment",
            path: "res/shaders/basic.fs"
        }
    }
}

pub mod gizmo {
    pub mod vertex {
        vulkano_shaders::shader! {
            ty: "vertex",
            path: "res/shaders/gizmo.vs"
        }
    }
    
    pub mod fragment {
        vulkano_shaders::shader! {
            ty: "fragment",
            path: "res/shaders/gizmo.fs"
        }
    }
}