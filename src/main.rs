extern crate sdl2;
extern crate gl;

mod render_gl;

use gl::types::*;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // set GL attributes
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem.window("Epic", 320, 240)
        .opengl() // Add openGL flag
        .resizable()
        .build()
        .unwrap();
    
    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // load shaders
    let vert_shader = render_gl::Shader::from_vert_source(
        &std::ffi::CString::new(include_str!("triangle.vert")).unwrap()
    ).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(
        &std::ffi::CString::new(include_str!("triangle.frag")).unwrap()
    ).unwrap();


    let program_id = unsafe { gl::CreateProgram() };

    unsafe {
        gl::AttachShader(program_id, vert_shader.id());
        gl::AttachShader(program_id, frag_shader.id());
        gl::LinkProgram(program_id);
        gl::DetachShader(program_id, vert_shader.id());
        gl::DetachShader(program_id, frag_shader.id());
    }

    unsafe {
        gl::UseProgram(program_id);
    }

    let vertices: Vec<f32> = vec![
        0.5, -0.5, 0.0,   1.0, 0.0, 0.0,   // bottom right
        -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,   // bottom left
        0.0,  0.5, 0.0,   0.0, 0.0, 1.0    // top
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );

        gl::EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            1, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
        );

        // unbind vbo and vao
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }






    // clear screen
    unsafe {
        gl::Viewport(0, 0, 320, 240);
        gl::ClearColor(1.0, 0.7, 0.7, 1.0);
    }

    // event_pump? yuck!
    let mut event_pump = sdl_context.event_pump().unwrap();

    'mainloop : loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'mainloop,
                _ => {},
            }
        }
        // render window stuff here 
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
            window.gl_swap_window();
        }
    }
} 