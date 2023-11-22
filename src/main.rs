#[macro_use]
extern crate glium;
extern crate winit;

use glium::{
    backend::glutin::SimpleWindowBuilder,
    Surface
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop
};

mod rendering;
mod teapot;

fn main() {
    let event_loop = EventLoop::new(); // Главный цикл отрисовки окна

    let (_window, _display) = SimpleWindowBuilder::new()
        .with_inner_size(500, 500)
        .with_title("Engine")
        .build(&event_loop);

    _window.set_min_inner_size(Some(winit::dpi::LogicalSize::new(350.0, 250.0)));

    let mut cube1 = rendering::Cube::new(rendering::Position::default(), rendering::Size::default());

    let positions = glium::VertexBuffer::new(&_display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&_display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&_display, glium::index::PrimitiveType::TrianglesList,
                                      &teapot::INDICES).unwrap();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        control_flow.set_wait();
    
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Closing Application");
                control_flow.set_exit();
            },
            Event::MainEventsCleared => {
                _window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                // Отрисовка кадра
                let mut frame = _display.draw();
                frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
                
                // Матрица
                let matrix = [
                    [0.01, 0.0, 0.0, 0.0],
                    [0.0, 0.01, 0.0, 0.0],
                    [0.0, 0.0, 0.01, 0.0],
                    [0.0, 0.0, 2.0, 1.0f32]
                ];
                
                // Направление света
                let light = [-1.0, 0.4, 0.9f32];

                let program = program!(&_display, 
                    150 => {
                        vertex: r#"#version 140

                        in vec3 position;
                        in vec3 normal;
                        
                        out vec3 v_normal;
                        
                        uniform mat4 perspective;
                        uniform mat4 matrix;
                        
                        void main() {
                            v_normal = transpose(inverse(mat3(matrix))) * normal;
                            gl_Position = perspective * matrix * vec4(position, 1.0);
                        }"#,
                        fragment: r#"#version 150
                        
                        in vec3 v_normal;
                        out vec4 color;
                        uniform vec3 u_light;

                        void main() {
                            float brightness = dot(normalize(v_normal), normalize(u_light));
                            vec3 dark_color = vec3(0.6, 0.0, 0.0);
                            vec3 regular_color = vec3(1.0, 0.0, 0.0);
                            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
                        }"# 
                    }
                ).unwrap();
                
                let params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::DepthTest::IfLess,
                        write: true,
                        .. Default::default()
                    },
                    backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                    .. Default::default()
                };

                let perspective = {
                    let (width, height) = frame.get_dimensions();
                    let aspect_ratio = height as f32 / width as f32;
                
                    let fov: f32 = 3.141592 / 3.0;
                    let zfar = 1024.0;
                    let znear = 0.1;
                
                    let f = 1.0 / (fov / 2.0).tan();
                
                    [
                        [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                        [         0.0         ,     f ,              0.0              ,   0.0],
                        [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                        [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
                    ]
                };

                frame.draw((&positions, &normals), &indices, &program, &uniform! { matrix: matrix, perspective: perspective, u_light: light }, &params).unwrap();

                // Окончание отрисовки кадра
                frame.finish().unwrap();
            },
            _ => ()
        }
    });
    
}