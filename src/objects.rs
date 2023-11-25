use glium::{Surface, glutin::{display, surface::WindowSurface}, DrawParameters};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f64; 3],
    normal: [f32; 3],
}
implement_vertex!(Vertex, position, normal);

pub trait Object3D {
    fn get_id(&self) -> i64;

    fn get_position(&self, ) -> [f64; 3];
    fn get_rotation(&self, ) -> [f64; 3];
    fn get_size(&self, ) -> [f64; 3];

    fn set_position(&mut self, position: &[f64; 3]);
    fn set_rotation(&mut self, rotation: &[f64; 3]);
    fn set_size(&mut self, size: &[f64; 3]);

    fn render(&self, _display: &glium::Display<WindowSurface>, _frame: &mut glium::Frame, screen: [[f64; 4]; 4], camera_view: [[f64; 4]; 4], perspective: [[f64; 4]; 4], global_light: [f64; 3], params: &glium::DrawParameters);
}

#[derive(Clone, Copy, Default)]
pub struct Cube {
    id: i64,

    position: [f64; 3],
    rotation: [f64; 3],
    size: [f64; 3],
}

impl Cube {
    pub fn new(position: [f64; 3], rotation: [f64; 3], size: [f64; 3]) -> Self {
        Self { position, rotation, size, ..Default::default()}
    }
}

impl Object3D for Cube {
    fn get_id(&self) -> i64 {
        self.id
    }

    fn get_position(&self) -> [f64; 3] {
        self.position
    }

    fn get_rotation(&self) -> [f64; 3] {
        self.rotation
    }

    fn get_size(&self) -> [f64; 3] {
        self.size
    }

    fn set_position(&mut self, position: &[f64; 3]) {
        self.position = *position
    }
    
    fn set_rotation(&mut self, rotation: &[f64; 3]) {
        self.rotation = *rotation
    }
    
    fn set_size(&mut self, size: &[f64; 3]) {
        self.size = *size
    }

    // Отрисовка куба
    fn render(&self, _display: &glium::Display<WindowSurface>, _frame: &mut glium::Frame, screen: [[f64; 4]; 4], camera_view: [[f64; 4]; 4], perspective: [[f64; 4]; 4], global_light: [f64; 3], params: &DrawParameters)  {
        let size_x = self.size[0];
        let size_y = self.size[1];
        let size_z = self.size[2];

        let vertex_shader_src = r#"
            #version 140

            in vec3 position;
            in vec3 normal;
            
            uniform mat4 matrix;
            
            void main() {
                gl_Position = matrix * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140

            out vec4 color;
            
            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        let program = glium::Program::from_source(_display, vertex_shader_src, fragment_shader_src,
            None).unwrap();
        println!("{size_x}, {size_y}, {size_z}");
        let shape =  glium::vertex::VertexBuffer::new(_display, &[
            Vertex { position: [-size_x / 2.0, size_y / 2.0, size_z / 2.0], normal: [0.0, 0.0, -1.0]},
            Vertex { position: [size_x / 2.0, size_y / 2.0, size_z / 2.0], normal: [0.0, 0.0, -1.0]},
            Vertex { position: [-size_x / 2.0, -size_y / 2.0, -size_z / 2.0], normal: [0.0, 0.0, -1.0]},
            Vertex { position: [size_x / 2.0, -size_y / 2.0, -size_z / 2.0], normal: [0.0, 0.0, -1.0]}
        ]).unwrap();
        
        _frame.draw(&shape, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &program,
        &uniform! { model: screen, view: camera_view, perspective: perspective, u_light: global_light },
        params).unwrap();
    }
}