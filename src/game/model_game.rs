use crate::engine::logic::IGameLogic;
use crate::engine::window::Window;
use crate::engine::renderer::Renderer;
use crate::engine::graph::shader::ShaderProgram;
use crate::engine::font::{Font};
use crate::engine::graph::mesh::Mesh;
use gltf::json::accessor::Type::Vec2;
use crate::engine::camera::Camera;
use nalgebra_glm::{vec3, TMat4, TVec3};
use crate::engine::graph::model::Model;
use crate::engine::graph::material::Material;
use std::thread::sleep;

pub struct ModelGame{
    renderer:Renderer,
    fonts:Vec<Font>,
    camera:Camera,
    scene:Vec<Model>,
}

impl ModelGame{
    pub fn new()->ModelGame {
        let renderer= Renderer::new(0,0);
        let mut fonts:Vec<Font> = Vec::new();
        let font = Font::new("NotoSansSC-Regular.otf",18);
        fonts.push(font);

        let scene = Vec::new();
        let camera = Camera::new(4.0/3.0);
        ModelGame{renderer,fonts,camera,scene}
    }
}

impl IGameLogic for ModelGame {
    fn init(&mut self) {
        self.renderer.init();

        let font_shader = ShaderProgram::new("font");
        for font in &mut self.fonts {
            font.shader = Some(font_shader);
        }

        let data:[f32;44] = [
            -1.0,1.0,-1.0  ,0.0,0.0,1.0  ,0.0,0.0  ,1.0,0.0,0.0,//左上角
            1.0,1.0,-1.0   ,0.0,0.0,1.0  ,1.0,0.0  ,0.0,1.0,0.0,//右上角
            -1.0,-1.0,-1.0 ,0.0,0.0,1.0  ,0.0,1.0  ,0.5,0.5,0.5,//左下角
            1.0,-1.0,-1.0  ,0.0,0.0,1.0  ,1.0,1.0  ,0.0,0.0,1.0,//右下角
        ];

        let indices:[u16;6] = [0,2,3,0,3,1];

        let mesh = Mesh::from_data(&data,&indices);
        let mut meshes = Vec::new();
        meshes.push(mesh);

        let base_shader = ShaderProgram::new("base");
        let textures = Vec::new();

        let mut material = Material::new(textures,base_shader);

        let mut transform = TMat4::default();
        transform.fill_with_identity();
        let model = Model::new(meshes,material,transform);

        self.scene.push(model)
    }

    fn input(&mut self,window:&Window) {

    }

    fn update(&mut self, window: &Window, interval: f32) {
    }

    fn render(&mut self, window: &Window) {
        self.renderer.set_view_size(window.width as i32, window.height as i32);
        self.renderer.clear_color(162.0/255.0,155.0/255.0,124.0/255.0);

        for model in &self.scene {
            model.draw(&self.camera)
        }

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