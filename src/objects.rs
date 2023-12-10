use glium::{Frame, glutin::surface::WindowSurface, Display, Surface, index::NoIndices, VertexBuffer};

#[path ="../src/teapot.rs"]
mod teapot;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32),
    normal: (f32, f32, f32)
}

glium::implement_vertex!(Vertex, position);

#[derive(Clone, Copy, Default)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }

    pub fn get(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

#[derive(Clone, Copy, Default)]
pub struct Camera {
    position: Vec3,
    direction: Vec3,

    fov: f32,

    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new(position: Vec3, direction: Vec3, fov: f32, znear: f32, zfar: f32) -> Self {
        Self { position, direction, fov, znear, zfar }
    }

    fn get_perspective(&self, frame: &Frame) -> [[f32; 4]; 4] {
        let (width, height) = frame.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;
        
        let render = 1.0 / (self.fov / 2.0).tan();

        [
            [render * aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, render, 0.0, 0.0],
            [0.0, 0.0, (self.zfar+self.znear) / (self.zfar-self.znear), 1.0],
            [0.0, 0.0, -(2.0*self.zfar*self.znear) / (self.zfar-self.znear), 0.0],
        ]
    }
}

#[derive(Default)]
pub struct Scene {
    camera: Camera,
    global_light: Vec3,

    objects: Vec<Box<dyn Object3D>>
}

impl Scene {
    pub fn render_scene(&mut self, display: &Display<WindowSurface>) {
        let mut frame = display.draw();
        frame.clear_color_and_depth((22.0 / 255.0, 22.0 / 255.0, 29.0 / 255.0, 1.0), 1.0);
        
        for object in &self.objects {
            println!("{}", object.get_id());
            object.render(self, display, &mut frame);
        }

        frame.finish().unwrap();
    }

    fn add_object(&mut self, object: Box<dyn Object3D>) {
        self.objects.push(object);
    }

    pub fn set_global_light(&mut self, light: Vec3) {
        self.global_light = light;
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }
}

trait Object3D {
    fn get_id(&self) -> u32;
    //fn new(parent_scene: Scene) -> Self where Self: Sized;
    fn render(&self, _parent_scene: &Scene, _display: &Display<WindowSurface>, _frame: &mut Frame);
}

pub struct Teapot {
    id: u32,

    position: Vec3,
    rotation: Vec3,
}

impl Teapot {
    pub fn new(parent_scene: &mut Scene, position: Vec3, rotation: Vec3) {
        let id = parent_scene.objects.len() as u32;

        parent_scene.add_object(Box::new(Self { id, position, rotation }));
    }
}

impl Object3D for Teapot {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn render(&self, _parent_scene: &Scene, _display: &Display<WindowSurface>, _frame: &mut Frame) {
        let model: [[f32; 4]; 4] = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 2.0, 1.0f32]
        ];

        let positions = glium::VertexBuffer::new(_display, &teapot::VERTICES).unwrap();
        let normals = glium::VertexBuffer::new(_display, &teapot::NORMALS).unwrap();
        let indices = glium::IndexBuffer::new(_display, glium::index::PrimitiveType::TrianglesList,
                                        &teapot::INDICES).unwrap();

        let vertex_shader_src = r#"
            #version 150

            in vec3 position;
            in vec3 normal;

            out vec3 v_normal;

            uniform mat4 perspective;
            uniform mat4 model;

            void main() {
                v_normal = transpose(inverse(mat3(model))) * normal;
                gl_Position = perspective * model * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 150

            in vec3 v_normal;
            out vec4 color;
            uniform vec3 light;

            void main() {
                float brightness = dot(normalize(v_normal), normalize(light));
                vec3 dark_color = vec3(0.6, 0.0, 0.0);
                vec3 regular_color = vec3(1.0, 0.0, 0.0);
                color = vec4(mix(dark_color, regular_color, brightness), 1.0);
            }
        "#;

        let program = glium::Program::from_source(_display, vertex_shader_src, fragment_shader_src,
                                                None).unwrap();
        
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        let perspective = _parent_scene.camera.get_perspective(_frame);

        _frame.draw((&positions, &normals), &indices, &program,
            &glium::uniform! { model: model, perspective: perspective, light: _parent_scene.global_light.get() },
            &params).unwrap();
    }
}