use sdl2::{Sdl, EventPump, video::{Window, GLContext, SwapInterval}};
use std::{ffi::{CStr, CString}, ptr::{null, null_mut}};
use gl::types::{GLuint, GLint, GLchar, GLenum};

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
        
        Ok(SxWindow {
            sdl,
            gl_context,
            gl,
            handle,
            event,
        })
    }
}

fn create_whitespace_cstring_with_len(len:usize) -> CString {
    let mut buffer:Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}

struct Shader {
    id:GLuint,
}

impl Shader {
    fn from_source(source: &CStr, kind: GLenum) -> Result<Self, String> {
        let id = unsafe {gl::CreateShader(kind)};
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), null());
            gl::CompileShader(id);
        }

        let mut success:GLint = 1;
        unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success); }
        if success == 0 {
            let mut len:GLint = 0;
            unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len); }

            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe { gl::GetShaderInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar); }
            return Err(error.to_string_lossy().into_owned());
        }
        
        Ok(Shader {id})
    }

    fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id); }
    }
}

struct Program {
    id: GLuint,
}

impl Program {
    fn from_shaders(shaders: &[Shader]) -> Result<Self, String> {
        let id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(id, shader.id()); }
        }

        unsafe { gl::LinkProgram(id); }

        let mut success:GLint = 1;
        unsafe { gl::GetProgramiv(id, gl::LINK_STATUS, &mut success); }
        if success == 0 {
            let mut len:GLint = 0;
            unsafe { gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len); }

            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe { gl::GetProgramInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar); }
            return Err(error.to_string_lossy().into_owned());
        }
        
        Ok(Program {id})
    }

    fn set(&self) {
        unsafe { gl::UseProgram(self.id); }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id); }
    }
}

struct Vbo {
    id:GLuint,
}

impl Vbo {
    fn gen() -> Self {
        let mut id: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut id); }
        Vbo {id}
    }

    fn set(&self, data: &Vec<f32>) {
        self.bind();
        self.data(data);
    }

    fn data(&self, data: &Vec<f32>) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }
    
    fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id); }
    }

    fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0); }
    }

    fn delete(&self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}

impl Drop for Vbo {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

struct Ebo {
    id:GLuint,
}

impl Ebo {
    fn gen() -> Self {
        let mut id: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut id); }
        Ebo {id}
    }

    fn set(&self, data: &Vec<u32>) {
        self.bind();
        self.data(data);
    }

    fn data(&self, indices: &Vec<u32>) {
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }
    
    fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id); }
    }

    fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); }
    }

    fn delete(&self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}

impl Drop for Ebo {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

struct Vao {
    id:GLuint,
}

impl Vao {
    fn gen() -> Self {
        let mut id: GLuint = 0;
        unsafe { gl::GenVertexArrays(1, &mut id); }
        Vao {id}
    }

    fn set(&self) {
        self.bind();
        self.setup();
    }    

    fn setup(&self) {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (2 * std::mem::size_of::<f32>()) as GLint,
                null());
        }
    }
    
    fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id); }
    }

    fn unbind(&self) {
        unsafe { gl::BindVertexArray(0); }        
    }

    fn delete(&self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id) }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        self.unbind();
        self.delete();
    }
}

fn main() -> Result<(), String> {
    let mut window = SxWindow::new("RUST GAME", 960, 540).unwrap();

    let vertex_shader = Shader::from_source(&CString::new(include_str!("shader.vert")).unwrap(), gl::VERTEX_SHADER).unwrap();
    let fragment_shader = Shader::from_source(&CString::new(include_str!("shader.frag")).unwrap(), gl::FRAGMENT_SHADER).unwrap();
    let shader_program = Program::from_shaders(&[vertex_shader, fragment_shader]).unwrap();
    shader_program.set();

    let vertices:Vec<f32> = vec![
         0.0,  0.5,
         0.5, -0.5,
        -0.5, -0.5,
    ];

    let indices:Vec<u32> = vec![
        0, 1, 2,
    ];
    
    let vbo = Vbo::gen();
    vbo.set(&vertices);

    let vao = Vao::gen();
    vao.set();
    
    let ebo = Ebo::gen();
    ebo.set(&indices);
        
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

                gl::DrawElements(
                    gl::TRIANGLES,
                    indices.len() as i32,
                    gl::UNSIGNED_INT,
                    0 as *const _);
            }

            window.handle.gl_swap_window();
        }
    }

    Ok(())
}
