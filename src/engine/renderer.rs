use crate::engine::font::{Font, Character};
use crate::engine::graph::shader::ShaderProgram;
use crate::engine::device::opengl::*;
use nalgebra::Vector3;
use nalgebra_glm::{vec3, ortho, TMat4, perspective, look_at, scale, TVec, TVec3};
use std::mem::size_of;
use std::os::raw::c_void;
use std::borrow::Borrow;
use crate::engine::graph::mesh::Mesh;
use crate::engine::camera::Camera;
use crate::engine::graph::model::Model;

pub trait Drawable{
    fn draw(&self);
}

pub struct Renderer{
    gui_context:GUIContext,
    view_size:(i32,i32)
}

pub struct GUIContext{
    font_vao:Option<u32>,
    font_vbo:Option<u32>,
}

impl Renderer {
    pub fn new(width:i32,height:i32)->Renderer{
        let gui_context = GUIContext{font_vao:None,font_vbo:None};
        Renderer{gui_context,view_size:(width,height)}
    }

    pub fn init(&mut self,){
        let (vao,vbo) = gen_font_mesh();
        self.gui_context.font_vao = Some(vao);
        self.gui_context.font_vbo = Some(vbo)
    }

    pub fn set_view_size(&mut self,width:i32,height:i32){
        self.view_size = (width,height)
    }

    pub fn clear_color(&self,r:f32,g:f32,b:f32){
        gl_clear_color(r,g,b);
        gl_clear();
    }

    pub fn render_model(&mut self,camera:&Camera,models:&Vec<Model>){
        gl_enable_depth_test();

        for model in models {
            model.draw(camera)
        }
    }

    pub fn render_mesh(&mut self,camera:&Camera,meshes:&Vec<Mesh>,shader:&ShaderProgram){
        gl_enable_depth_test();

        shader.bind();
        let p:TMat4<f32> = camera.projection_matrix();
        shader.set_mat4("P",&p);
        let v:TMat4<f32> = camera.view_matrix();
        shader.set_mat4("V",&v);
        let mut m:TMat4<f32> = TMat4::default();
        m.fill_with_identity();
        shader.set_mat4("M",&m);

        for mesh in meshes {
            mesh.draw()
        }

        shader.unbind()
    }

    pub fn render_text(&mut self,pos:(f32,f32),color:&TVec3<f32>,text:&str,font:&mut Font){
        unsafe {
            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }
        let shader = &font.shader.unwrap();
        shader.bind();
        shader.set_vec3("color",color);
        let width = self.view_size.0;
        let height = self.view_size.1;
        let p = ortho(0.0, width as f32, height as f32, 0.0, 0.0, 1.0);
        shader.set_mat4("P",&p);

        let gui_context =&self.gui_context;

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindVertexArray(gui_context.font_vao.unwrap());
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
                    gl::BindBuffer(gl::ARRAY_BUFFER,gui_context.font_vbo.unwrap());
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