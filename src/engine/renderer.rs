use crate::engine::device::opengl::{viewport, clear, clear_color, pixel_unpack};
use crate::engine::font::{Font, Character};
use crate::engine::graph::shader::ShaderProgram;

pub struct Renderer{

}

impl Renderer {
    pub fn new()->Renderer{
        Renderer{}
    }

    pub fn viewport(&self,width:u32,height:u32){
        viewport(0, 0, width as i32, height as i32);
    }

    pub fn clear_color(&self,r:f32,g:f32,b:f32){
        clear_color(r,g,b);
        clear();
    }

    pub fn render(&self,text:String,font:&Font,shader:&ShaderProgram){
        for char in text.chars() {

        }
    }

    pub fn upload_char(&self,char:&Character){
        pixel_unpack(1)
    }
}