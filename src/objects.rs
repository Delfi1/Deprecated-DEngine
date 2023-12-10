use glium::{Frame, glutin::surface::WindowSurface, Display, Surface, index::NoIndices, VertexBuffer, DrawParameters};

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

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
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

    fn get_view(&self) -> [[f32; 4]; 4] {
        let f = {
            let f = self.direction.get();
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };
        
        let up = [0.0, 1.0, 0.0f32];

        let s = [up[1] * f[2] - up[2] * f[1],
                 up[2] * f[0] - up[0] * f[2],
                 up[0] * f[1] - up[1] * f[0]];
    
        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        };
    
        let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
                 f[2] * s_norm[0] - f[0] * s_norm[2],
                 f[0] * s_norm[1] - f[1] * s_norm[0]];
    
        let position = self.position.get();

        let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
                 -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
                 -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];
    
        [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
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
    pub fn render_scene(&mut self, display: &Display<WindowSurface>, params: &DrawParameters<'_>) {
        let mut frame = display.draw();
        frame.clear_color_and_depth((22.0 / 255.0, 22.0 / 255.0, 29.0 / 255.0, 1.0), 1.0);
        
        for object in &self.objects {
            object.render(self, display, &mut frame, params);
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

    pub fn get_object(&mut self, id: u32) -> &mut Box<dyn Object3D> {
        self.objects.first_mut().unwrap()
    }
}

pub trait Object3D {
    fn get_id(&self) -> u32;
    //fn new(parent_scene: Scene) -> Self where Self: Sized;

    fn get_position(&self) -> Vec3;
    fn set_position(&mut self, position: Vec3);

    fn render(&self, _parent_scene: &Scene, _display: &Display<WindowSurface>, _frame: &mut Frame, params: &DrawParameters<'_>);
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

    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn set_position(&mut self, position: Vec3) {
        self.position = position
    }

    fn render(&self, _parent_scene: &Scene, _display: &Display<WindowSurface>, _frame: &mut Frame, params: &DrawParameters<'_>) {
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
            uniform mat4 view;
            uniform mat4 model;
            uniform vec3 pos;

            void main() {
                mat4 modelview = view * model;
                
                vec3 render_position = position + pos;
                v_normal = transpose(inverse(mat3(modelview))) * normal;
                gl_Position = perspective * modelview * vec4(render_position, 1.0);
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

        let perspective = _parent_scene.camera.get_perspective(_frame);

        let view = _parent_scene.camera.get_view();

        let new_position = self.position + Vec3::new(0.0, 0.01, 0.0);

        _frame.draw((&positions, &normals), &indices, &program,
            &glium::uniform! { model: model, pos: self.position.get(), perspective: perspective, view: view, light: _parent_scene.global_light.get() },
            params).unwrap();
    }
}