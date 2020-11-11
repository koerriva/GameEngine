use crate::engine::window::Window;
use gl::types::*;

pub mod opengl{
    #[derive(Eq,PartialEq,Copy,Clone)]
    pub enum ShaderType{
        Vertex,
        Fragment
    }
    #[derive(Eq,PartialEq,Copy,Clone)]
    pub enum DrawType{
        Static,Dynamic
    }

    use std::os::raw::c_void;
    use crate::engine::device::opengl::ShaderType::Vertex;
    use std::ffi::CString;
    use std::mem::size_of;
    use crate::engine::device::opengl::DrawType::Static;

    pub fn gl_viewport(x:i32, y:i32, width:i32, height:i32){
        unsafe {
            gl::Viewport(x,y,width,height)
        }
    }

    pub fn gl_clear_color(r:f32,g:f32,b:f32){
        unsafe {
            gl::ClearColor(r,g,b,1.0)
        }
    }

    pub fn gl_clear(){
        unsafe {
            gl::Clear(gl::DEPTH_BUFFER_BIT|gl::COLOR_BUFFER_BIT)
        }
    }

    pub fn gl_enable_depth_test(){
        unsafe {gl::Enable(gl::DEPTH_TEST)}
    }

    pub fn gl_disable_depth_test(){
        unsafe {gl::Disable(gl::DEPTH_TEST)}
    }

    pub fn gl_wireframe_mode(){
        unsafe {gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE)}
    }

    pub fn gl_shader_mode(){
        unsafe {gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL)}
    }

    pub fn gl_gen_vao()->u32{
        let mut vao=0;
        unsafe {gl::GenVertexArrays(1,&mut vao)}
        vao
    }
    pub fn gl_bind_vao(vao:u32){
        unsafe {gl::BindVertexArray(vao)}
    }
    pub fn gl_unbind_vao(){
        unsafe {gl::BindVertexArray(0)}
    }
    pub fn gl_gen_vbo()->u32{
        let mut vbo=0;
        unsafe {gl::GenBuffers(1,&mut vbo)}
        vbo
    }
    pub fn gl_bind_vbo(vbo:u32){
        unsafe {gl::BindBuffer(gl::ARRAY_BUFFER,vbo)}
    }
    pub fn gl_unbind_vbo(){
        unsafe {gl::BindBuffer(gl::ARRAY_BUFFER,0)}
    }
    pub fn gl_upload_vbo(data:&[f32],draw_type:DrawType){
        let buffer_size:isize = (data.len() * size_of::<f32>()) as isize;
        match draw_type {
            Static => unsafe {gl::BufferData(gl::ARRAY_BUFFER,buffer_size, data.as_ptr() as *const c_void,gl::STATIC_DRAW)},
            _ => unsafe {gl::BufferData(gl::ARRAY_BUFFER, buffer_size, std::ptr::null(), gl::DYNAMIC_DRAW)}
        }
    }

    pub fn gl_bind_program(program:u32){
        unsafe {gl::UseProgram(program)}
    }

    pub fn gl_unbind_program(){
        unsafe {gl::UseProgram(0)}
    }

    pub fn gen_font_mesh()->(u32,u32){
        let mut vao = 0;
        let mut vbo =0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            let buffer_size = 24 * size_of::<f32>();
            gl::BufferData(gl::ARRAY_BUFFER, buffer_size as isize, std::ptr::null(), gl::DYNAMIC_DRAW);
            gl::EnableVertexAttribArray(0);
            let pointer_size = 4 * size_of::<f32>();
            gl::VertexAttribPointer(0,4,gl::FLOAT,gl::FALSE,pointer_size as i32,std::ptr::null());

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        (vao,vbo)
    }

    pub fn gen_shader(code:&str,shader_type:ShaderType)->Result<u32,String>{
        let kind = if shader_type==Vertex{
            gl::VERTEX_SHADER
        }else {
            gl::FRAGMENT_SHADER
        };
        let mut id:u32=0;

        unsafe {
            id = gl::CreateShader(kind);
            let c_source = CString::from_vec_unchecked(code.as_bytes().to_vec());
            gl::ShaderSource(id, 1, &c_source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);

            let mut success:i32 = 1;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            if success==0 {
                let mut len:i32 = 0;
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
                // allocate buffer of correct size
                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
                // fill it with len spaces
                buffer.extend([b' '].iter().cycle().take(len as usize));
                // convert buffer to CString
                let error: CString = CString::from_vec_unchecked(buffer);
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
                return Err(error.to_string_lossy().into_owned())
            }
        }

        Ok(id)


    }

    pub fn gen_program(vertex:u32,fragment:u32)->Result<u32,String>{
        let mut id:u32=0;
        unsafe {
            id = gl::CreateProgram();
            gl::AttachShader(id,vertex);
            gl::AttachShader(id,fragment);
            gl::LinkProgram(id);

            let mut success:i32 = 1;
            gl::GetProgramiv(id,gl::LINK_STATUS,&mut success);

            if success==0 {
                let mut len:i32 = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
                // allocate buffer of correct size

                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
                // fill it with len spaces
                buffer.extend([b' '].iter().cycle().take(len as usize));
                // convert buffer to CString
                let error: CString = CString::from_vec_unchecked(buffer);
                gl::GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
                return Err(error.to_string_lossy().into_owned())
            }

            gl::DetachShader(id,vertex);
            gl::DetachShader(id,vertex);
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
        }
        Ok(id)
    }
}