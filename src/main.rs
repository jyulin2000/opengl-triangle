extern crate sdl2;
extern crate gl;

pub mod render_gl;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsytem = sdl.video().unwrap();

    let gl_attr = video_subsytem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 0);

    let window = video_subsytem
        .window("triangle", 1000, 1000)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::load_with(|s| video_subsytem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let mut event_pump = sdl.event_pump().unwrap();

    let mut nAttributes: gl::types::GLint = 3;
    unsafe { gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nAttributes as *mut i32); }
    println!("nAttributes: {:?}", nAttributes);

    unsafe {
        //gl::Viewport(0, 0, 1000, 1000);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        //gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    //window.gl_set_context_to_current().unwrap();
    window.gl_swap_window();

    use std::ffi::CString;

    let vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("triangle.vert")).unwrap()
    ).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("triangle.frag")).unwrap()
    ).unwrap();

    let shader_program = render_gl::Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();

    shader_program.set_used();

    let mut vertices: Vec<f32> = vec![
        // positions        // colors
        -0.5, -0.25, 0.0,    0.7, 0.2, 0.2, // top left
        0.0, -0.25, 0.0,     0.2, 0.7, 0.2, // top right
        -0.75, -0.75, 0.0,    0.2, 0.2, 0.7, // bottom left
        -0.25, -0.75, 0.0,     1.0, 1.0, 1.0, // bottom right
    ];

    let mut indices: Vec<gl::types::GLuint> = vec![
        0, 1, 2,
        1, 2, 3
    ];

    // START VBO and VAO

    // Vertex Buffer Object
    /*
    Request OpenGL to give us one buffer name (as integer), and write it into Vertex Buffer Object (vbo) variable:
    For GenBuffers, we have to provide a pointer to array which it will overwrite with a new value. 
    Rust references (&mut and &) are pointers, so we can simply pass them along. We must, of course, 
    limit the number of buffers to 1 so it does not overwrite unknown memory nearby. That would be bad. 
    This unsafe block means something.
    */
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    /*
    I suggest docs.gl site for OpenGL documentation. For example, glBufferData function page contains 
    comprehensive documentation with examples and information about supported OpenGL versions. Very useful. 
    */
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, //target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, //size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, //pointer to data
            gl::STATIC_DRAW, //usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); //unbind the buffer
    }

    // Vertex Array Object

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    } 

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        
        // Position vertex attribute
        gl::EnableVertexAttribArray(0); // this is "layout (location=0)" in vertex shader
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );

        // Color vertex attribute
        gl::EnableVertexAttribArray(1); // "layout (location=1)" in vertex shader
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
        // END VBO AND VAO
    }
    
    let mut ebo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<gl::types::GLuint>()) as gl::types::GLsizeiptr,
            indices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }




    // Second triangle
    let vert_shader2 = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("triangle2.vert")).unwrap()
    ).unwrap();

    let frag_shader2 = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("triangle2.frag")).unwrap()
    ).unwrap();

    let shader_program2 = render_gl::Program::from_shaders(
        &[vert_shader2, frag_shader2]
    ).unwrap();

    shader_program2.set_used();

    let uniform_location = unsafe { gl::GetUniformLocation(shader_program2.id(), CString::new("ourColor").unwrap().as_ptr() as *const gl::types::GLchar) };
    println!("Uniform location {:?}", uniform_location);
    unsafe {
        gl::Uniform4f(uniform_location, 1.0, 0.6, 0.93, 1.0);
    }

    let vertices2: Vec<f32> = vec![
        // positions
        0.5, 0.6, 0.0,
        0.3, 0.3, 0.0,
        0.7, 0.3, 0.0,
    ];
//     let vertices2: Vec<f32> = vec![
//     -0.5, -0.5, 0.0,
//     0.5, -0.5, 0.0,
//     0.0, 0.5, 0.0
// ];

    let mut vbo2: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo2);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo2);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices2.len() * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
            vertices2.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
        
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vao2: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao2);
        println!("{}", vao2);
        gl::BindVertexArray(vao2);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo2); // See what happens if this is removed

        gl::EnableVertexAttribArray(0); // this is "layout (location=0)" in vertex shader
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<gl::types::GLfloat>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );

        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    
    unsafe {
        //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            //vertices[7] *= 1.01;

            shader_program.set_used();

            // unsafe {
            //     gl::BindVertexArray(vao);
            //     gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            //     gl::DrawArrays(
            //         gl::TRIANGLES, //mode
            //         0, // starting index in the enabled arrays
            //         3 // number of indices to be rendered
            //     );
            //     gl::BindVertexArray(0);
            //     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            // }
            
            // Draw parallelogram in bottom-left
            unsafe {
                gl::BindVertexArray(vao);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
                gl::BindVertexArray(0);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            }
            
            shader_program2.set_used();
            // Draw triangle in top-right
            unsafe {
                gl::BindVertexArray(vao2);
                //gl::BindBuffer(gl::ARRAY_BUFFER, vbo2);
                gl::DrawArrays(gl::TRIANGLES, 0, 3);

                gl::BindVertexArray(0);
                //gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            }

            window.gl_swap_window();
        }
    }
}