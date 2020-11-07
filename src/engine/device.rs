use crate::engine::window::Window;
use gl::types::*;

pub mod opengl{
    #[derive(Eq,PartialEq,Copy,Clone)]
    pub enum ShaderType{
        Vertex,
        Fragment
    }

    use std::os::raw::c_void;
    use crate::engine::device::opengl::ShaderType::Vertex;
    use std::ffi::CString;

    pub fn viewport(x:i32, y:i32, width:i32, height:i32){
        unsafe {
            gl::Viewport(x,y,width,height)
        }
    }

    pub fn clear_color(r:f32,g:f32,b:f32){
        unsafe {
            gl::ClearColor(r,g,b,1.0)
        }
    }

    pub fn clear(){
        unsafe {
            gl::Clear(gl::DEPTH_BUFFER_BIT|gl::COLOR_BUFFER_BIT)
        }
    }

    //禁用字节对齐
    pub fn pixel_unpack(size:usize){
        unsafe {gl::PixelStorei(gl::UNPACK_ALIGNMENT,size as i32)}
    }

    pub fn gen_font_texture(w:i32,h:i32,data:&[u8])->u32{
        let mut texture:u32 = 0;
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT,1);
            gl::GenTextures(1,&mut texture);
            gl::BindTexture(gl::TEXTURE_2D,texture);
            gl::TexImage2D(gl::TEXTURE_2D,0,gl::RED as i32,w,h,0,gl::RED,gl::UNSIGNED_BYTE,data.as_ptr() as *const c_void);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::BindTexture(gl::TEXTURE_2D,0);
        }
        texture
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