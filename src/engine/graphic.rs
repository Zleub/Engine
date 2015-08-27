extern crate gl;
extern crate glfw;

use self::gl::types::*;
use self::glfw::Context as glfw_Content;
use self::glfw::{OpenGlProfileHint, WindowHint};
use std;
use std::ptr;
use std::str;
use std::ffi::CString;

pub static VERTEX_DATA: [GLfloat; 6] = [
     0.0,  0.5,
     0.5, -0.5,
    -0.5, -0.5
];

pub static VS_SRC: &'static str =
   "#version 150\n\
    in vec2 position;\n\
    void main() {\n\
       gl_Position = vec4(position, 0.0, 1.0);\n\
    }";

pub static FS_SRC: &'static str =
   "#version 150\n\
    out vec4 out_color;\n\
    void main() {\n\
       out_color = vec4(1.0, 1.0, 1.0, 1.0);\n\
    }";

pub struct Context {
	pub width   : u32,
	pub height  : u32,
	pub context : glfw::Glfw,
	pub window  : glfw::Window,
	pub events  : std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
}

pub fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8"));
        }
    }
    shader
}

pub fn link_program(vs: GLuint, fs: GLuint) -> GLuint { unsafe {
    let program = gl::CreateProgram();
    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);
    gl::LinkProgram(program);
    // Get the link status
    let mut status = gl::FALSE as GLint;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

    // Fail on error
    if status != (gl::TRUE as GLint) {
        let mut len: GLint = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
        let mut buf = Vec::with_capacity(len as usize);
        buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
        gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
        panic!("{}", str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8"));
    }
    program
} }

pub fn init(width: u32, height: u32) -> Context {
	println!("Hello Engine");

	let mut context = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

	context.window_hint(WindowHint::ContextVersion(3, 3));
	context.window_hint(WindowHint::OpenGlForwardCompat(true));
	context.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

	let (mut window, events) = context.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
		.expect("Failed to create GLFW window.");

	window.set_key_polling(true);
	window.make_current();

	// OpenGL linking
	gl::load_with(|s| window.get_proc_address(s));

	return Context {
		width : width,
		height : height,
		context : context,
		window : window,
		events : events,
	};
}
