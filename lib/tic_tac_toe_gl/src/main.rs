extern crate gl;
extern crate cpal;
#[macro_use]
extern crate glium;

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [0.0, 0.0] };
    let vertex2 = Vertex { position: [0.6, 0.0] };
    let vertex3 = Vertex { position: [0.6, 0.6] };
    let vertex4 = Vertex { position: [0.0, 0.6] };
    let vertex5 = Vertex { position: [0.2, 0.8] };
    let vertex6 = Vertex { position: [0.2, -0.2] };
    let vertex7 = Vertex { position: [0.4, 0.8] };
    let vertex8 = Vertex { position: [0.4, -0.2] };

    let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6, vertex7, vertex8];

    let shape_indices = vec![1u8, 0, 3, 2];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target
            .draw(&vertex_buffer,
                  &indices,
                  &program,
                  &glium::uniforms::EmptyUniforms,
                  &Default::default())
            .unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
