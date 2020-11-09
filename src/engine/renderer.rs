use crate::engine::font::{Font, Character};
use crate::engine::graph::shader::ShaderProgram;
use crate::engine::device::opengl::*;
use nalgebra::Vector3;
use nalgebra_glm::{vec3, ortho, TMat4, perspective, look_at, scale};
use std::mem::size_of;
use std::os::raw::c_void;
use std::borrow::Borrow;
use crate::engine::graph::mesh::Mesh;
use crate::engine::camera::Camera;
use crate::engine::graph::model::Model;

pub struct Renderer{
    font_vao:Option<u32>,
    font_vbo:Option<u32>,
}

impl Renderer {
    pub fn new()->Renderer{
        Renderer{font_vao:None,font_vbo:None}
    }

    pub fn init(&mut self){
        let (vao,vbo) = gen_font_mesh();
        self.font_vao = Some(vao);
        self.font_vbo = Some(vbo)
    }

    pub fn viewport(&self,width:u32,height:u32){
        gl_viewport(0, 0, width as i32, height as i32);
    }

    pub fn clear_color(&self,r:f32,g:f32,b:f32){
        gl_clear_color(r,g,b);
        gl_clear();
    }

    pub fn render_model(&mut self,camera:&Camera,models:&Vec<Model>){
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }

        for model in models {
            model.draw(camera)
        }
    }

    pub fn render_mesh(&mut self,camera:&Camera,meshes:&Vec<Mesh>,shader:&ShaderProgram){
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }

        shader.bind();
        let p:TMat4<f32> = camera.projection_matrix();
        shader.set_mat4("P",p);
        let v:TMat4<f32> = camera.view_matrix();
        shader.set_mat4("V",v);
        let mut m:TMat4<f32> = TMat4::default();
        m.fill_with_identity();
        shader.set_mat4("M",m);

        for mesh in meshes {
            mesh.draw()
        }

        shader.unbind()
    }

    pub fn render_text(&mut self,pos:(f32,f32),color:(f32,f32,f32),text:String,font:&mut Font,shader:&ShaderProgram){
        unsafe {
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }

        shader.bind();
        shader.set_vec3("color",vec3(color.0,color.1,color.2));
        let p = ortho(0.0, 1280.0, 720.0, 0.0,0.0,1.0);
        shader.set_mat4("P",p);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindVertexArray(self.font_vao.unwrap());
        }

        let mut pos_x = pos.0;
        let pos_y = pos.1;
        for char in text.chars() {
            let font_height = font.height;
            let r = font.read_mut(char as usize);
            if r.is_some() {
                let c = r.unwrap();
                let x = pos_x + c.bearing.0 as f32;
                let y = pos_y + (font_height as i32 +c.size.1-c.bearing.1) as f32;
                let w = c.size.0 as f32;
                let h = c.size.1 as f32;

                //6个顶点
                let vertices:[f32;24] = [
                    x,y-h,0.0,0.0,
                    x,y,0.0,1.0,
                    x+w,y,1.0,1.0,

                    x,y-h,0.0,0.0,
                    x+w,y,1.0,1.0,
                    x+w,y-h,1.0,0.0,
                ];

                pos_x += (c.advance.0>>6) as f32;

                unsafe {
                    let texture = c.texture.as_ref().unwrap();
                    gl::BindTexture(gl::TEXTURE_2D,texture.id);
                    gl::BindBuffer(gl::ARRAY_BUFFER,self.font_vbo.unwrap());
                    let buffer_size = vertices.len()*size_of::<f32>();
                    gl::BufferSubData(gl::ARRAY_BUFFER,0,buffer_size as isize,vertices.as_ptr() as *const c_void);
                    gl::DrawArrays(gl::TRIANGLES,0,6);
                }
            }
        }
        unsafe {
            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D,0);

            gl::Disable(gl::CULL_FACE);
            gl::Disable(gl::BLEND);
        }
    }
}