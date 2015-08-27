extern crate gl;
extern crate glfw;

use self::gl::types::*;
use self::glfw::Context as glfw_Content;
use std::mem;
use std::ptr;
use std::ffi::CString;

struct Context {
	g_context : graphic::Context,
}

pub mod graphic ;

pub fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
	match event {
		glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
			window.set_should_close(true)
		}
		_ => {}
	}
}

pub fn init(width: u32, height: u32) {
	let mut context: Context = Context {
		g_context : graphic::init(width, height)
	};
	println!("Engine Start");

	let vs = graphic::compile_shader(graphic::VS_SRC, gl::VERTEX_SHADER);
    let fs = graphic::compile_shader(graphic::FS_SRC, gl::FRAGMENT_SHADER);
    let program = graphic::link_program(vs, fs);

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (graphic::VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&graphic::VERTEX_DATA[0]),
                       gl::STATIC_DRAW);

        // Use shader program
        gl::UseProgram(program);
        gl::BindFragDataLocation(program, 0,
                                 CString::new("out_color").unwrap().as_ptr());

        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(program,
                                             CString::new("position").unwrap().as_ptr());
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT,
                                gl::FALSE as GLboolean, 0, ptr::null());
    }

	// while !context.g_context.window.should_close() {
	// 	context.g_context.context.poll_events();
	// 	for (_, event) in glfw::flush_messages(&context.g_context.events) {
	// 		println!("{:?}", event);
	// 		handle_window_event(&mut context.g_context.window, event);
	// 	}
	// }

	while !context.g_context.window.should_close() {
        // Poll events
        context.g_context.context.poll_events();
		for (_, event) in glfw::flush_messages(&context.g_context.events) {
				println!("{:?}", event);
				handle_window_event(&mut context.g_context.window, event);
			}
        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Draw a triangle from the 3 vertices
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // Swap buffers
        context.g_context.window.swap_buffers();
    }

    unsafe {
    // Cleanup
        gl::DeleteProgram(program);
        gl::DeleteShader(fs);
        gl::DeleteShader(vs);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}
