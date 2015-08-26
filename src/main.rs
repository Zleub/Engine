pub mod engine {

    extern crate glfw;

    struct Context {
        g_context : graphic::Context,
    }

    pub mod graphic {
        extern crate gl;
        extern crate glfw;

        use std;
        use self::glfw::Context as glfw_Context;

        pub struct RenderHandler {
            pub send        : std::sync::mpsc::Sender<()>,
            pub task_done   : std::result::Result<std::thread::JoinHandle<()>, std::io::Error>
        }

        pub struct Context {
            pub width   : u32,
            pub height  : u32,
            pub context : glfw::Glfw,
            pub window  : glfw::Window,
            pub events  : std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
            pub handler : RenderHandler
        }

        pub fn render(mut context: glfw::RenderContext, finish: std::sync::mpsc::Receiver<()>) {
            context.make_current();
            loop {
                // Check if the rendering should stop.
                if finish.try_recv() == Ok(()) { break };

                // Perform rendering calls
                unsafe {
                    gl::ClearColor(0.3, 0.3, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    gl::DrawArrays(gl::TRIANGLES, 0, 3);
                }
            }

            // required on some platforms
            glfw::make_context_current(None);
        }

        pub fn init(width: u32, height: u32) -> Context {
            println!("Hello Engine");

            let context = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
            let (mut window, events) = context.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
                .expect("Failed to create GLFW window.");

            window.set_key_polling(true);

            // OpenGL linking
            gl::load_with(|s| window.get_proc_address(s));

            // Rendering Thread
            let render_context = window.render_context();
            let (send, recv) = std::sync::mpsc::channel();
            let render_task = std::thread::Builder::new().name("render task".to_string());

            let task_done = render_task.spawn(move || {
                render(render_context, recv);
            });

            let handler: RenderHandler = RenderHandler {
                task_done : task_done,
                send : send
            };

            return Context {
                width : width,
                height : height,
                context : context,
                window : window,
                events : events,
                handler : handler
            };
        }
    }

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

        while !context.g_context.window.should_close() {
            context.g_context.context.poll_events();
            for (_, event) in glfw::flush_messages(&context.g_context.events) {
                println!("{:?}", event);
                handle_window_event(&mut context.g_context.window, event);
            }
        }

        // Tell the render task to exit.
        context.g_context.handler.send.send(()).ok().expect("Failed signal to render thread.");

        // Wait for acknowledgement that the rendering was completed.
        let _ = context.g_context.handler.task_done;
    }
}

fn main() {
    engine::init(200, 200);
}
