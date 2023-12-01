#[path = "../src/teapot.rs"]
mod teapot;

use glium::{Surface, glutin::{display, surface::WindowSurface}, DrawParameters};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}
implement_vertex!(Vertex, position, normal);

pub trait Object3D {
    fn get_id(&self) -> u64;

    fn get_position(&self, ) -> [f32; 3];
    fn get_rotation(&self, ) -> [f32; 3];
    fn get_size(&self, ) -> [f32; 3];

    fn set_position(&mut self, position: &[f32; 3]);
    fn set_rotation(&mut self, rotation: &[f32; 3]);
    fn set_size(&mut self, size: &[f32; 3]);

    fn render(&self, _display: &glium::Display<WindowSurface>, _frame: &mut glium::Frame, camera_view: [[f32; 4]; 4], perspective: [[f32; 4]; 4], global_light: [f32; 3], params: &glium::DrawParameters);
}

pub struct Camera {
    position: [f32; 3],
    direction: [f32; 3],
    up: [f32; 3]
}

impl Camera {
    pub fn new(position: [f32; 3], direction: [f32; 3], up: [f32; 3]) -> Self {
        Camera {position, direction, up}
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        let f = {
            let f = self.direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };
    
        let s = [self.up[1] * f[2] - self.up[2] * f[1],
                 self.up[2] * f[0] - self.up[0] * f[2],
                 self.up[0] * f[1] - self.up[1] * f[0]];
    
        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        };
    
        let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
                 f[2] * s_norm[0] - f[0] * s_norm[2],
                 f[0] * s_norm[1] - f[1] * s_norm[0]];
    
        let p = [-self.position[0] * s_norm[0] - self.position[1] * s_norm[1] - self.position[2] * s_norm[2],
                 -self.position[0] * u[0] - self.position[1] * u[1] - self.position[2] * u[2],
                 -self.position[0] * f[0] - self.position[1] * f[1] - self.position[2] * f[2]];
    
        [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }
}

#[derive(Clone, Copy, Default)]
pub struct Cube {
    id: u64,

    position: [f32; 3],
    rotation: [f32; 3],
    size: [f32; 3],
}

impl Cube {
    pub fn new(position: [f32; 3], rotation: [f32; 3], size: [f32; 3]) -> Self {
        Self { position, rotation, size, ..Default::default()}
    }
}

impl Object3D for Cube {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_position(&self) -> [f32; 3] {
        self.position
    }

    fn get_rotation(&self) -> [f32; 3] {
        self.rotation
    }

    fn get_size(&self) -> [f32; 3] {
        self.size
    }

    fn set_position(&mut self, position: &[f32; 3]) {
        self.position = *position
    }
    
    fn set_rotation(&mut self, rotation: &[f32; 3]) {
        self.rotation = *rotation
    }
    
    fn set_size(&mut self, size: &[f32; 3]) {
        self.size = *size
    }

    // Отрисовка куба
    fn render(&self, _display: &glium::Display<WindowSurface>, _frame: &mut glium::Frame, screen: [[f32; 4]; 4], perspective: [[f32; 4]; 4], global_light: [f32; 3], params: &DrawParameters)  {
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

        let model = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 2.0, 1.0f32]
        ];
        
        let program = glium::Program::from_source(_display, vertex_shader_src, fragment_shader_src,
            None).unwrap();
        
        let shape =  glium::vertex::VertexBuffer::new(_display, &[
            Vertex { position: [-size_x / 2.0, size_y / 2.0, size_z / 2.0], normal: [0.0, 0.0, -1.0]},
            Vertex { position: [size_x / 2.0, size_y / 2.0, size_z / 2.0], normal: [0.0, 0.0, -1.0]},
            Vertex { position: [-size_x / 2.0, -size_y / 2.0, -size_z / 2.0], normal: [0.0, 0.0, -1.0]},
            Vertex { position: [size_x / 2.0, -size_y / 2.0, -size_z / 2.0], normal: [0.0, 0.0, -1.0]}
        ]).unwrap();
        
        _frame.draw(&shape, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &program,
        &uniform! { model: screen, perspective: perspective, u_light: global_light },
        params).unwrap();
    }
}

#[derive(Clone, Copy, Default)]
pub struct Teapod {
    id: u64,

    position: [f32; 3],
    rotation: [f32; 3],
    size: [f32; 3],
}

impl Teapod {
    pub fn new(position: [f32; 3], rotation: [f32; 3], size: [f32; 3]) -> Self {
        Self { position, rotation, size, ..Default::default()}
    }
}

impl Object3D for Teapod {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_position(&self, ) -> [f32; 3] {
        self.position
    }
    fn get_rotation(&self, ) -> [f32; 3] {
        self.rotation
    }
    fn get_size(&self, ) -> [f32; 3] {
        self.size
    }
    fn set_position(&mut self, position: &[f32; 3]) {
        self.position = *position
    }
    fn set_rotation(&mut self, rotation: &[f32; 3]) {
        self.rotation = *rotation
    }
    fn set_size(&mut self, size: &[f32; 3]) {
        self.size = *size
    }

    fn render(&self, _display: &glium::Display<WindowSurface>, _frame: &mut glium::Frame, camera_view: [[f32; 4]; 4], perspective: [[f32; 4]; 4], global_light: [f32; 3], params: &glium::DrawParameters) {
    
        let program = program!(_display,
            150 => {
                vertex: r#"
                #version 150

                in vec3 position;
                in vec3 normal;

                out vec3 v_normal;

                uniform mat4 perspective;
                uniform mat4 view;
                uniform mat4 model;

                void main() {
                    mat4 modelview = view * model;
                    v_normal = transpose(inverse(mat3(modelview))) * normal;
                    gl_Position = perspective * modelview * vec4(position, 1.0);
                }
                "#,
                fragment: r#"
                #version 150

                in vec3 v_normal;
                out vec4 color;
                uniform vec3 u_light;

                void main() {
                    float brightness = dot(normalize(v_normal), normalize(u_light));
                    vec3 dark_color = vec3(0.6, 0.0, 0.0);
                    vec3 regular_color = vec3(1.0, 0.0, 0.0);
                    color = vec4(mix(dark_color, regular_color, brightness), 1.0);
                }
                "#
            },
        ).unwrap();

        let positions = glium::VertexBuffer::new(_display, &teapot::VERTICES).unwrap();
        let normals = glium::VertexBuffer::new(_display, &teapot::NORMALS).unwrap();
        let indices = glium::IndexBuffer::new(_display, glium::index::PrimitiveType::TrianglesList,
                                            &teapot::INDICES).unwrap();
        
        let model = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 2.0, 1.0f32]
        ];

        _frame.draw((&positions, &normals), &indices, &program, &uniform! { model: model, view: camera_view, perspective: perspective, u_light: global_light },
        &params).unwrap();
    }
}