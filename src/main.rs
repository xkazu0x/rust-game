use sdl2::{
    Sdl, video::{Window, GLContext, SwapInterval}, EventPump
};

struct SxWindow {
    sdl:Sdl,
    gl_context: GLContext,
    gl: (),
    handle:Window,
    event:EventPump,
}

impl SxWindow {
    fn new(title:&str, width:u32, height:u32) -> Result<Self, &'static str> {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_version(3, 3);
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        
        let handle = video_subsystem
            .window(title, width, height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let gl_context = handle.gl_create_context().unwrap();
        let gl = gl::load_with(|s| {
            video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        });

        handle
            .subsystem()
            .gl_set_swap_interval(SwapInterval::VSync)
            .unwrap();
        
        let event = sdl.event_pump().unwrap();
        
        Ok(SxWindow{
            sdl,
            gl_context,
            gl,
            handle,
            event,
        })
    }
}

fn main() {
    let mut window = SxWindow::new("RUST GAME", 960, 540).unwrap();
    
    'running: loop {
        for event in window.event.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'running,
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape), ..} => {
                    break 'running
                },
                _ => {}
            }

            unsafe {
                gl::ClearColor(0.25, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            window.handle.gl_swap_window();
        }
    }
}
