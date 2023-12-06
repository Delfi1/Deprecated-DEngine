#[path = "../src/teapot.rs"]
mod teapot;

use glium::{Surface, glutin::{display, surface::WindowSurface}, DrawParameters, GlObject, Frame, Display};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}
implement_vertex!(Vertex, position, normal);

#[derive(Default)]
pub struct Scene<'a> {
    camera: Camera,
    ambient: Ambient,
    global_light: [f32; 3],
    params: glium::DrawParameters<'a>,
    
    objects: Vec<Box<dyn Object3D>>,
}

impl Scene<'_> {
    pub fn new(camera: Camera, global_light: [f32; 3], params: DrawParameters) -> Scene<'_> {
        Scene { camera, objects: Vec::new(), global_light, params, ..Default::default() }
    }

    fn add_object(&mut self, object: Box<dyn Object3D>) {
        self.objects.push(object);
    }

    pub fn get_object(&self, id: i32) -> Result<&Box<dyn Object3D>, String> {
        for object in &self.objects {
            if object.get_id() == id {
                return Ok(&object);
            }
        }
        Err("Object not found".to_string())
    } 
    
    pub fn get_perspective(&self, _frame: &glium::Frame) -> [[f32; 4]; 4] {
        let (width, height) = _frame.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = self.camera.fov;
        let zfar = self.camera.zfar;
        let znear = self.camera.znear;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [   f / aspect_ratio  ,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        let f = {
            let f = self.camera.direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            (f[0] / len, f[1] / len, f[2] / len)
        };

        let up = self.camera.up;

        let s = (f.1 * up[2] - f.2 * up[1],
                 f.2 * up[0] - f.0 * up[2],
                 f.0 * up[1] - f.1 * up[0]);

        let s_norm = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (s_norm.1 * f.2 - s_norm.2 * f.1,
                 s_norm.2 * f.0 - s_norm.0 * f.2,
                 s_norm.0 * f.1 - s_norm.1 * f.0);

        let p = (-self.camera.position[0] * s.0 - self.camera.position[1] * s.1 - self.camera.position[2] * s.2,
                 -self.camera.position[0] * u.0 - self.camera.position[1] * u.1 - self.camera.position[2] * u.2,
                 -self.camera.position[0] * f.0 - self.camera.position[1] * f.1 - self.camera.position[2] * f.2);

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [s_norm.0, u.0, f.0, 0.0],
            [s_norm.1, u.1, f.1, 0.0],
            [s_norm.2, u.2, f.2, 0.0],
            [p.0, p.1,  p.2, 1.0],
        ]
    }

    pub fn render(&self, _display: &Display<WindowSurface>) {
        // Отрисовка кадра
        let mut frame = _display.draw();
        
        frame.clear_color_and_depth(self.ambient.get_color(), 0.8);

        for object in &self.objects {
            object.render(&self, _display, &mut frame);
        }

        // Окончание отрисовки кадра
        frame.finish().unwrap();
    }
}

pub trait Object3D {    
    fn get_id(&self) -> i32;

    fn get_position(&self) -> [f32; 3];
    fn set_position(&mut self, position: [f32; 3]);

    fn get_rotation(&self) -> [f32; 3];
    fn set_rotation(&mut self, rotation: [f32; 3]);   

    fn render(&self, parent_scene: &Scene, _display: &Display<WindowSurface>, _frame: &mut Frame);
}

#[derive(Default)]
pub struct Camera {
    position: [f32; 3],
    direction: [f32; 3],
    up: [f32; 3],

    fov: f32,
    zfar: f32,
    znear: f32
}

pub struct Ambient {
    color: [i64; 3],
    alpha: f32
}

impl Default for Ambient {
    fn default() -> Self {
        Self { color: [22, 22, 29], alpha: 1.0 }
    }
}

impl Ambient {
    fn get_color(&self) -> (f32, f32, f32, f32) {
        let r: f32 = self.color[0] as f32 / 255.0 as f32;
        let g: f32 = self.color[1] as f32 / 255.0 as f32;
        let b: f32 = self.color[2] as f32 / 255.0 as f32;
        (r, g, b, self.alpha)
    }

    fn set_color(&mut self, color: (i64, i64, i64)) {
        self.color = [color.0, color.1, color.2];
    }
}

impl Camera {
    pub fn new(position: [f32; 3], direction: [f32; 3], up: [f32; 3], fov: f32, zfar: f32, znear: f32) -> Self {
        Camera { position, direction, up, fov, zfar, znear }
    }
}

pub struct Cube {
    id: i32,

    position: [f32; 3],
    rotation: [f32; 3],
    size: [f32; 3]
}

impl Cube {
    pub fn new(scene: &mut Scene, position: [f32; 3], rotation: [f32; 3], size: [f32; 3]) {
        let _id = scene.objects.len() as i32;

        let cube = Cube {id: _id, position, rotation, size};

        scene.add_object(Box::new(cube));
    }
}

impl Object3D for Cube {
    fn get_id(&self) -> i32 {
        self.id
    }
    fn get_position(&self) -> [f32; 3] {
        self.position
    }
    fn get_rotation(&self) -> [f32; 3] {
        self.rotation
    }

    fn set_position(&mut self, position: [f32; 3]) {
        self.position = position
    }
    fn set_rotation(&mut self, rotation: [f32; 3]) {
        self.rotation = rotation
    }

    fn render(&self, parent_scene: &Scene, _display: &Display<WindowSurface>, _frame: &mut Frame) {
        //println!("Rendering object id<{}>", self.get_id());

        let shape = glium::vertex::VertexBuffer::new(_display, &[
            Vertex { position: [-1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [ 1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [ 1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0] },
        ]).unwrap();

        let model = [
            [1.0, 0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [1.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        let program = program!(_display,
            150 => {
                vertex: r#"
                    #version 150

                    in vec3 position;
                    in vec3 normal;

                    out vec3 v_normal;
                    out vec3 v_position;

                    uniform mat4 perspective;
                    uniform mat4 view;
                    uniform mat4 model;

                    void main() {
                        mat4 modelview = view * model;
                        v_normal = transpose(inverse(mat3(modelview))) * normal;
                        gl_Position = perspective * modelview * vec4(position, 1.0);
                        v_position = gl_Position.xyz / gl_Position.w;
                    }
                "#,
                fragment: r#"
                #version 150
        
                in vec3 v_normal;
                in vec3 v_position;
        
                out vec4 color;
        
                uniform vec3 u_light;
        
                const vec3 ambient_color = vec3(0.2, 0.0, 0.0);
                const vec3 diffuse_color = vec3(0.6, 0.0, 0.0);
                const vec3 specular_color = vec3(1.0, 1.0, 1.0);
        
                void main() {
                    float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);
        
                    vec3 camera_dir = normalize(-v_position);
                    vec3 half_direction = normalize(normalize(u_light) + camera_dir);
                    float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);
        
                    color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
                }
            "#
            }
        ).unwrap();

        let view = parent_scene.get_view();

        let perspective = parent_scene.get_perspective(_frame);

        _frame.draw(&shape, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &program,
            &uniform! { model: model, view: view, perspective: perspective, u_light: parent_scene.global_light },
            &parent_scene.params).unwrap();
    }
}