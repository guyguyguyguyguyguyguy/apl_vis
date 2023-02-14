extern crate glium;

use glium::glutin;
use glium::Surface;
use ndarray;

use std::{
    io::{prelude::*},
    net::TcpListener,
};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

glium::implement_vertex!(Vertex, position, tex_coords);

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn set_next_tex(shape: &Vec<usize>) -> impl Fn(Vec<u8>) -> glium::texture::RawImage2d<'static, u8> + 'static {
    let s = shape.clone();
    let [x, y, z] = s[..] else {panic!{"Not a 3d array"}};
    move |flat_vec: Vec<u8>| {
        let rgb = 255 * ndarray::Array::from_shape_vec((x, y, z), flat_vec).expect("Cannot create ndarray");
        // println!("{:?}", rgb);
        // let mut rgb = ndarray::Array3::zeros((x, y, 3));
        // println!("{:?}", rgb);
        // for ((x, y, z), v) in rgb.indexed_iter_mut() {
        //     if x < 10 {*v = 255;}
        //     else if x > 10 && x < 20 {*v = 100;}
        //     else {*v = 0;}
        //     }
        glium::texture::RawImage2d::from_raw_rgb(rgb.into_raw_vec(), (x as u32, y as u32))
    }
}


fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7342").expect("Could not bind");
    let (mut client, _) = listener
        .accept()
        .expect("Listener couldn't being accepting.");

    let mut s_buf: [u8; 3] = [0; 3];
    let _ = client.read(&mut s_buf);
    println!("{:?}", s_buf);
    // let shape: Vec<usize> = String::from_utf8_lossy(&s_buf)
    //     .split(" ")
    //     .map(|x| x.parse::<usize>().unwrap())
    //     .collect();
    let shape: Vec<usize> = s_buf.iter().map(|&e| e as usize).collect();
    let arr_size: usize = shape
        .clone()
        .into_iter()
        .reduce(|a, b| a * b)
        .unwrap();
    
    let next_frame = set_next_tex(&shape);

    let mut arr_buf = vec![0; arr_size * 2 + 1];

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("APL simulations")
        .with_resizable(true)
        .with_inner_size(glium::glutin::dpi::PhysicalSize::new(800, 800));
    //.with_maximized(true);
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // let start: ndarray::Array3<u8> = ndarray::Array3::zeros((shape[1], shape[2], 3));
    // let image = glium::texture::RawImage2d::from_raw_rgb(start.into_raw_vec(), (shape[1] as u32, shape[2] as u32));
    // let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    let behaviour = glium::uniforms::SamplerBehavior {
        minify_filter: glium::uniforms::MinifySamplerFilter::Nearest,
        magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
        ..Default::default()
    };

    // Vertex to display texture
    let canvas: Vec<Vertex> = vec![
        Vertex {
            position: [-0.95, -0.95],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [-0.95, 0.95],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [0.95, -0.95],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: [0.95, 0.95],
            tex_coords: [1.0, 1.0],
        },
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &canvas).unwrap();
    let indices: [u16; 6] = [0, 1, 2, 1, 3, 2];
    let index_buffer = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &indices,
    )
    .unwrap();
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 colour;
        uniform sampler2D tex;

        void main() {
            colour = texture(tex, v_tex_coords);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }

        _ = client.read(&mut arr_buf);
        let vect: Vec<u8> = serde_json::from_slice(&arr_buf).expect("Cannot read arr json");
        let image = next_frame(vect);
        let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();
        // let texture = glium::texture::Texture2d::new(&display, image).unwrap();

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let mut target = display.draw();
        target.clear_color(0.0, 0.4, 0.4, 0.3);

        let uniforms = glium::uniform! {
            tex: glium::uniforms::Sampler(&texture, behaviour),
        };

        target
            .draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}
