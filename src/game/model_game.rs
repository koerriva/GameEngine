use crate::engine::logic::IGameLogic;
use crate::engine::window::Window;
use crate::engine::renderer::Renderer;
use crate::engine::graph::shader::ShaderProgram;
use crate::engine::font::{Font};
use crate::engine::graph::mesh::Mesh;
use gltf::json::accessor::Type::Vec2;
use crate::engine::camera::Camera;
use nalgebra_glm::{vec3, TMat4, TVec3, sin, Mat4};
use crate::engine::graph::model::{Model, Scene};
use crate::engine::graph::material::Material;
use std::thread::sleep;
use nalgebra::clamp;
use glfw::Key::*;

pub struct ModelGame{
    renderer:Renderer,
    fonts:Vec<Font>,
    scene:Option<Scene>,
    camera_state:(f32,f32,f32,f32)
}

impl ModelGame{
    pub fn new()->ModelGame {
        let renderer= Renderer::new(0,0);
        let mut fonts:Vec<Font> = Vec::new();
        let font = Font::new("NotoSansSC-Regular.otf",18);
        fonts.push(font);

        ModelGame{renderer,fonts,scene:None,camera_state:(0.0,0.0,0.0,0.0)}
    }
}

impl IGameLogic for ModelGame {
    fn init(&mut self) {
        self.renderer.init();

        let font_shader = ShaderProgram::new("font");
        for font in &mut self.fonts {
            font.shader = Some(font_shader);
        }

        let scene = Scene::from_gltf("data/model/Scene.gltf");
        self.scene = Some(scene)
    }

    fn input(&mut self,window:&Window) {
        if window.is_key_pressed(W){
            self.camera_state.1 = 1.0
        }
        if window.is_key_pressed(S){
            self.camera_state.1 = -1.0
        }
        if window.is_key_pressed(A){
            self.camera_state.0 = -1.0
        }
        if window.is_key_pressed(D){
            self.camera_state.0 = 1.0
        }

        let (_,_,x0,y0) = window.mouse_offset;
        self.camera_state.2 = x0;
        self.camera_state.3 = y0
    }

    fn update(&mut self, window: &Window, interval: f32) {
        let scene = self.scene.as_mut().unwrap();
        for model in &mut scene.models {
            model.rotate(interval*10.0,&vec3(0.0,1.0,0.0));
        }

        let camera = &mut scene.camera;
        camera.move_forward(self.camera_state.1*interval*10.0);
        camera.move_right(self.camera_state.0*interval*10.0);

        camera.rotation(self.camera_state.2,self.camera_state.3);

        self.camera_state = (0.0,0.0,0.0,0.0);
    }

    fn render(&mut self, window: &Window) {
        self.renderer.set_view_size(window.width as i32, window.height as i32);
        self.renderer.clear_color(162.0/255.0,155.0/255.0,124.0/255.0);

        let scene = self.scene.as_mut().unwrap();
        let camera = &scene.camera;
        let models = &scene.models;
        self.renderer.render_model(camera,models);

        let font_noto = &mut self.fonts[0];
        let font_color:TVec3<f32> = vec3(179.0/255.0,0.0,0.0);
        self.renderer.render_text((5.0,690.0),&font_color,String::from("Powered By Rust\u{00A9}"),font_noto)
    }
}

impl Drop for ModelGame {
    fn drop(&mut self) {
        println!("Drop ModelGame")
    }
}