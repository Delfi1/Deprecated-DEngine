#[derive(Debug, Clone, Copy)]
struct Position {
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
struct Size {
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

}

#[derive(Default)]
struct Cube{
    transform: Transform,
    size: Size
}

impl Cube {
    pub fn new(position: Position, size: Size) -> Self {
        Self { transform: Transform::new(position, Rotation::default()), size, .. Default::default() }
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