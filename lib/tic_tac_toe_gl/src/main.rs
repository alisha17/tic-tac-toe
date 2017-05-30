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


   // Load cross
    let image = image::load(Cursor::new(&include_bytes!("/home/alisha/Desktop/cross.jpeg")[..]),
                          image::JPEG).unwrap().to_rgba();

     let image_dimensions = image.dimensions();
     let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(),
                                                                    image_dimensions);
     let texture = glium::texture::Texture2d::new(&display, image).unwrap();

   // Load zero  

    let image_zero = image::load(Cursor::new(&include_bytes!("/home/alisha/Desktop/images.png")[..]),
                          image::PNG).unwrap().to_rgba();

     let image_dimensions_zero = image_zero.dimensions();
     let image_zero = glium::texture::RawImage2d::from_raw_rgba_reversed(image_zero.into_raw(),
                                                                    image_dimensions_zero);
     let texture_zero = glium::texture::Texture2d::new(&display, image_zero).unwrap();

   // Buffer for board  

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

       

    let shape_indices = vec![1u8, 0, 3, 2];
   
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

   // Buffer for cross 

    let texture_vertex_buffer = {

            #[derive(Copy, Clone, Debug)]
            struct Vertex {
                position: [f32; 2],
                tex_coords: [f32; 2],
            }

            implement_vertex!(Vertex, position, tex_coords);

            glium::VertexBuffer::new(&display, &[Vertex {
                position: [0.5, 0.3],
                tex_coords: [0.0, 0.0],
            },
            Vertex{
                position: [0.5, 0.5],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [0.7, 0.5],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [0.7, 0.3],
                tex_coords: [1.0, 0.0],
            }
              ])
                .unwrap()
    }; 
 
    let shape_indices = vec![1u8, 0, 2, 0, 3, 2];

    let texindices = glium::index::IndexBuffer::new(&display,
                                                 glium::index::PrimitiveType::TrianglesList,
                                                 &shape_indices).unwrap();

    //let texindices = glium::index::NoIndices(glium::index::PrimitiveType::LineStrip);

// Buffer for zero   

 let texture_vertex_buffer_zero = {

            #[derive(Copy, Clone, Debug)]
            struct Vertex {
                position: [f32; 2],
                tex_coords: [f32; 2],
            }

            implement_vertex!(Vertex, position, tex_coords);

            glium::VertexBuffer::new(&display, &[Vertex {
                position: [0.5, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex{
                position: [0.5, 0.2],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [0.7, 0.2],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [0.7, 0.0],
                tex_coords: [1.0, 0.0],
            }
              ])
                .unwrap()
    }; 
 
    let shape_indices = vec![1u8, 0, 2, 0, 3, 2];

    let texindices_zero = glium::index::IndexBuffer::new(&display,
                                                 glium::index::PrimitiveType::TrianglesList,
                                                 &shape_indices).unwrap();

// Program 1

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

    let tprogram1 =
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
        let mut ttarget_zero = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        ttarget.clear_color(0.0, 0.0, 1.0, 1.0);
        ttarget_zero.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = uniform! {
            tex: &texture,
        };

        let uniforms_zero = uniform! {
            tex: &texture_zero,
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

         ttarget_zero
            .draw(&texture_vertex_buffer_zero,
                  &texindices_zero,
                  &tprogram1,
                  &uniforms_zero,
                  &Default::default())
            .unwrap();
        ttarget_zero.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
