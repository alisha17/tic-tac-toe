extern crate gl;
extern crate cpal;
#[macro_use]
extern crate glium;
extern crate image;
use std::io::Cursor;

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();


    let image = image::load(Cursor::new(&include_bytes!("/home/alisha/Desktop/cross.jpeg")[..]),
                          image::JPEG).unwrap().to_rgba();

     let image_dimensions = image.dimensions();
     let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(),
                                                                    image_dimensions);
     let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let normal_vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
        }

        implement_vertex!(Vertex, position);

        glium::VertexBuffer::new(&display,
                                 &[Vertex { position: [0.0, 0.0] },
                                   Vertex { position: [0.6, 0.0] },
                                   Vertex { position: [0.6, 0.6] },
                                   Vertex { position: [0.0, 0.6] },
                                   Vertex { position: [0.2, 0.8] },
                                   Vertex { position: [0.2, -0.2] },
                                   Vertex { position: [0.4, 0.8] },
                                   Vertex { position: [0.4, -0.2] }])
                .unwrap()
    };


    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::LinesList,&[1u8, 0, 3, 2]).unwrap();


    let texture_vertex_buffer = {

            #[derive(Copy, Clone, Debug)]
            struct Vertex {
                position: [f32; 2],
                tex_coords: [f32; 2],
            }

            implement_vertex!(Vertex, position, tex_coords);

            glium::VertexBuffer::new(&display, &[Vertex {
                position: [-0.5, -0.5],
                tex_coords: [0.0, 0.0],
            },
            Vertex{
                position: [0.0, 0.5],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.25],
                tex_coords: [1.0, 0.0],
            }
              ])
                .unwrap()
    }; 

    let texindices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    

    let program =
        glium::Program::from_source(&display,  r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "# ,
        r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#, 
        None)
            .unwrap();


    let tprogram =
        glium::Program::from_source(&display, 
        r#"
        #version 140
   
        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        void main() {
           v_tex_coords = tex_coords;
           gl_Position = vec4(position, 0.0, 1.0);
        }"#,  

       r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }"#
        , None)
            .unwrap();

    loop {
        let mut target = display.draw();
        let mut ttarget = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        ttarget.clear_color(0.0, 0.0, 1.0, 1.0);
        let uniforms = uniform! {
            tex: &texture,
        };

        target
            .draw(&normal_vertex_buffer,
                  &indices,
                  &program,
                  &glium::uniforms::EmptyUniforms,
                  &Default::default())
            .unwrap();
        target.finish().unwrap();
        ttarget
            .draw(&texture_vertex_buffer,
                  &texindices,
                  &tprogram,
                  &uniforms,
                  &Default::default())
            .unwrap();
        ttarget.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
