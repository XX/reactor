extern crate gl;
extern crate glfw;
extern crate cgmath;

#[macro_use]
mod engine;
mod lang;

use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;
use gl::types::*;
use cgmath::prelude::*;

use engine::camera::Camera;
use engine::window::Window;
use engine::shader::Shader;
use lang::{ObjectPar, ObjectMethods, Matrix4};

pub fn main() {
    let camera = ObjectPar::construct(Camera::default());
    let mut window = Window::new("Reactor", 800, 600);
    window.controls.push(camera.clone());

    let (shader, cube_vao) = unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::MULTISAMPLE);

        // build and compile our shader program
        // ------------------------------------
        let shader = Shader::new(
            "shaders/main.vs",
            "shaders/main.fs");

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        let cube_vertices: [f32; 108] = [
            // positions
            -0.5, -0.5, -0.5,
            0.5, -0.5, -0.5,
            0.5,  0.5, -0.5,
            0.5,  0.5, -0.5,
            -0.5,  0.5, -0.5,
            -0.5, -0.5, -0.5,

            -0.5, -0.5,  0.5,
            0.5, -0.5,  0.5,
            0.5,  0.5,  0.5,
            0.5,  0.5,  0.5,
            -0.5,  0.5,  0.5,
            -0.5, -0.5,  0.5,

            -0.5,  0.5,  0.5,
            -0.5,  0.5, -0.5,
            -0.5, -0.5, -0.5,
            -0.5, -0.5, -0.5,
            -0.5, -0.5,  0.5,
            -0.5,  0.5,  0.5,

            0.5,  0.5,  0.5,
            0.5,  0.5, -0.5,
            0.5, -0.5, -0.5,
            0.5, -0.5, -0.5,
            0.5, -0.5,  0.5,
            0.5,  0.5,  0.5,

            -0.5, -0.5, -0.5,
            0.5, -0.5, -0.5,
            0.5, -0.5,  0.5,
            0.5, -0.5,  0.5,
            -0.5, -0.5,  0.5,
            -0.5, -0.5, -0.5,

            -0.5,  0.5, -0.5,
            0.5,  0.5, -0.5,
            0.5,  0.5,  0.5,
            0.5,  0.5,  0.5,
            -0.5,  0.5,  0.5,
            -0.5,  0.5, -0.5,
        ];

        // setup cube VAO
        let (mut cube_vao, mut cube_vbo) = (0, 0);
        gl::GenVertexArrays(1, &mut cube_vao);
        gl::GenBuffers(1, &mut cube_vbo);
        gl::BindVertexArray(cube_vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, cube_vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (cube_vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &cube_vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);
        let stride = 3 * mem::size_of::<GLfloat>() as GLsizei;
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());

        (shader, cube_vao)
    };

    let render = |window: &mut Window| {
        unsafe {
            gl::ClearColor(0.23, 0.23, 0.23, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // set transformation matrices
            if let Ok(camera) = camera.lock() {
                shader.useProgram();
                let (width, height) = window.glfw_window().get_framebuffer_size();
                shader.setMat4(c_str!("projection"), &camera.projection_matrix(width, height));
                shader.setMat4(c_str!("view"), &camera.view_matrix());
                shader.setMat4(c_str!("model"), &Matrix4::identity());
            }

            gl::BindVertexArray(cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    };

    window.events_loop(Some(render));
}
