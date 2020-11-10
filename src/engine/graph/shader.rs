use crate::engine::resource::ResourceLoader;
use crate::engine::device::opengl::{gen_shader, gen_program, gl_bind_program, gl_unbind_program};
use crate::engine::device::opengl::ShaderType::{Vertex, Fragment};
use std::ops::Add;
use std::ffi::CString;
use nalgebra_glm::{TVec3, TMat4};

#[derive(Copy, Clone)]
pub struct ShaderProgram {
    program:u32,
}

impl ShaderProgram {
    pub fn new(name:&str)->ShaderProgram{
        let vertex_file = String::from(name).add(".vert");
        let fragment_file = String::from(name).add(".frag");
        let vertex_code = ResourceLoader::load_shader(vertex_file.as_str());
        let fragment_code = ResourceLoader::load_shader(fragment_file.as_str());

        let vertex = gen_shader(vertex_code.unwrap(),Vertex);
        if vertex.is_err(){
            panic!(format!("编译顶点着色器失败{}",vertex.err().unwrap()))
        }
        let fragment = gen_shader(fragment_code.unwrap(),Fragment);
        if fragment.is_err(){
            panic!(format!("编译像素着色器失败{}",fragment.err().unwrap()))
        }

        let program = gen_program(vertex.unwrap(),fragment.unwrap());
        if program.is_err(){
            panic!(format!("生成着色程序失败{}",program.err().unwrap()))
        }
        ShaderProgram{program:program.unwrap()}
    }

    pub fn set_vec3(&self,name:&str,vec3:&TVec3<f32>){
        unsafe {
            let c_name = CString::from_vec_unchecked(name.as_bytes().to_vec());
            let location = gl::GetUniformLocation(self.program,c_name.as_ptr());
            gl::Uniform3fv(location,1,vec3.as_ptr())
        }
    }

    pub fn set_mat4(&self,name:&str,mat4:&TMat4<f32>){
        unsafe {
            let c_name = CString::from_vec_unchecked(name.as_bytes().to_vec());
            let location = gl::GetUniformLocation(self.program,c_name.as_ptr());
            gl::UniformMatrix4fv(location,1,gl::FALSE,mat4.as_ptr());
        }
    }

    pub fn bind(&self){
        unsafe {gl::UseProgram(self.program)}
    }

    pub fn unbind(&self){
        unsafe {gl::UseProgram(0)}
    }
}