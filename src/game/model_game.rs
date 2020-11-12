use crate::engine::logic::IGameLogic;
use crate::engine::window::Window;
use crate::engine::renderer::Renderer;
use crate::engine::graph::shader::ShaderProgram;
use crate::engine::font::{Font};
use crate::engine::graph::mesh::Mesh;
use gltf::json::accessor::Type::Vec2;
use crate::engine::camera::Camera;
use nalgebra_glm::{vec3, TMat4, TVec3, sin, Mat4, vec1};
use crate::engine::graph::model::{Model, Scene};
use crate::engine::graph::material::Material;
use std::thread::sleep;
use nalgebra::clamp;
use glfw::Key::*;
use crate::engine::graph::texture::Texture;
use glfw::MouseButton::Button2;
use noise::{Worley, NoiseFn, Perlin, Fbm, MultiFractal, Seedable};
use crate::engine::recoder::ScreenRecoder;

pub struct ModelGame{
    renderer:Renderer,
    fonts:Vec<Font>,
    scene:Option<Scene>,
    camera_state:(f32,f32,f32,f32),
    recorder:ScreenRecoder
}

impl ModelGame{
    pub fn new()->ModelGame {
        let renderer= Renderer::new(0,0);
        let mut fonts:Vec<Font> = Vec::new();
        let font = Font::new("NotoSansSC-Regular.otf",18);
        fonts.push(font);

        ModelGame{renderer,fonts,scene:None
            ,camera_state:(0.0,0.0,0.0,0.0),recorder:ScreenRecoder::new()}
    }
}

impl IGameLogic for ModelGame {
    fn init(&mut self) {
        self.renderer.init();
        self.recorder.run();

        let font_shader = ShaderProgram::new("font");
        for font in &mut self.fonts {
            font.shader = Some(font_shader);
        }

        let size = 256;
        let mut noise = Fbm::default();
        noise = noise.set_seed(1291);
        noise = noise.set_frequency(0.01);
        noise = noise.set_lacunarity(1.6);
        noise = noise.set_persistence(0.57);
        noise = noise.set_octaves(8);
        let mut heightmap = vec![0.0; size*size];
        for y in 0..size {
            for x in 0..size {
                let val = noise.get([x as f64,y as f64]);
                // println!("value {}",val);
                heightmap[y*size+x] = val;
            }
        }
        let mesh = Mesh::from_heightmap(size as i32, size as i32, &heightmap);
        let mut data = vec![66,76,80];
        let texture = Texture::new(1,1,3,&data);
        let base_shader = ShaderProgram::new("terrain");
        let material = Material::new(vec![texture],base_shader);
        let terrain = Model::new(vec![mesh],material);

        let mut scene = Scene::empty();
        scene.models.push(terrain);

        // let mut scene = Scene::from_gltf("data/model/Scene.gltf");
        self.scene = Some(scene);

        let camera = &mut self.scene.as_mut().unwrap().camera;
        camera.set_position(2.87,2.42,-5.81);
        camera.set_rotation(111.75,-11.0)
    }

    fn input(&mut self,window:&Window) {
        self.camera_state = (0.0,0.0,0.0,0.0);

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

        if window.is_mouse_click(Button2){
            self.camera_state.2 = window.mouse_offset.2;
            self.camera_state.3 = window.mouse_offset.3;
        }

        if window.is_key_pressed(F1){
            self.renderer.set_wireframe_mode()
        }

        if window.is_key_pressed(F2){
            self.renderer.set_hide_hud();
        }

        if window.is_key_pressed(F3) {
            let (w,h) = window.frame_buffer_size;
            let buffer = self.renderer.read_framebuffer(w,h);
            self.recorder.record(w,h,buffer);
        }
    }

    fn update(&mut self, window: &Window, interval: f32) {
        let scene = self.scene.as_mut().unwrap();
        // let factor = window.glfw.get_time().sin() as f32;
        // for model in &mut scene.models {
        //     model.rotate(interval*factor*5.0,&vec3(1.0,0.0,0.0));
        // }

        let camera = &mut scene.camera;
        camera.move_forward(self.camera_state.1*interval);
        camera.move_right(self.camera_state.0*interval);
        camera.rotate(self.camera_state.2,self.camera_state.3);
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

        let pos = (&camera.position.x,&camera.position.y,&camera.position.z);
        self.renderer.render_text((5.0,5.0),&vec3(0.1,0.9,0.2),format!("相机位置{:?}",pos).as_str(),font_noto);
        let rot = (&camera.yaw,&camera.pitch);
        self.renderer.render_text((5.0,25.0),&vec3(0.1,0.9,0.2),format!("相机旋转{:?}",rot).as_str(),font_noto);
        self.renderer.render_text((5.0,690.0),&font_color,"Powered By Rust\u{00A9}",font_noto)
    }
}

impl Drop for ModelGame {
    fn drop(&mut self) {
        println!("Drop ModelGame")
    }
}