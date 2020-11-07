use crate::engine::resource::ResourceLoader;
use crate::engine::device::opengl::{gen_shader, gen_program};
use crate::engine::device::opengl::ShaderType::{Vertex, Fragment};
use std::ops::Add;

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
}