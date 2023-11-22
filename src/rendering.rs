use glium::{Display, glutin::surface::WindowSurface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f64; 3],
}
implement_vertex!(Vertex, position);

#[derive(Debug, Clone, Copy)]
pub struct Position {
    x: f64,
    y: f64,
    z: f64
}

impl Default for Position {
    fn default() -> Self{
        Position { x: (0.0), y: (0.0), z: (0.0) }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    x: f64,
    y: f64,
    z: f64
}

impl Default for Size {
    fn default() -> Self{
        Size { x: (1.0), y: (1.0), z: (1.0) }
    }
}

#[derive(Debug, Clone, Copy)] 
struct Rotation {

}

impl Default for Rotation {
    fn default() -> Self{
        Rotation {  }
    }
}

struct Transform {
    position: Position,
    rotation: Rotation
}

impl Transform {
    fn new(position: Position, rotation: Rotation) -> Self {
        Transform { position, rotation }
    }
}

impl Default for Transform {
    fn default() -> Self{
        Transform { position: (Position::default()), rotation: (Rotation::default()) }
    }
}

// Объект в 3D
trait Object3D {
    // Позиция объекта
    fn get_position(&mut self) -> Position;
    fn set_position(&mut self, new_position: Position);
    // Поворот объекта
    fn get_rotation(&mut self) -> Rotation;
    fn set_rotation(&mut self, new_rotation: Rotation);
}

impl dyn Object3D {
    fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
        let f = {
            let f = direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };
    
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
pub struct Cube{
    transform: Transform,
    size: Size
}

impl Cube {
    pub fn new(position: Position, size: Size) -> Self {
        Self { transform: Transform::new(position, Rotation::default()), size, .. Default::default() }
    }

    pub fn render(&mut self, frame: &glium::Frame, _display: &Display<WindowSurface>, ) {
        println!("{:?}", self.get_position())
    }
}

impl Object3D for Cube{
    #[inline]
    fn get_position(&mut self) -> Position {
        self.transform.position
    }

    #[inline]
    fn set_position(&mut self, position: Position) {
        self.transform.position = position
    }

    #[inline]
    fn get_rotation(&mut self) -> Rotation {
        self.transform.rotation
    }

    #[inline]
    fn set_rotation(&mut self, rotation: Rotation) {
        self.transform.rotation = rotation
    }
}