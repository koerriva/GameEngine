use crate::engine::logic::IGameLogic;
use crate::engine::window::Window;
use crate::engine::renderer::Renderer;
use crate::engine::graph::shader::ShaderProgram;
use crate::engine::font::{Font};
use crate::engine::graph::mesh::Mesh;
use gltf::json::accessor::Type::Vec2;
use crate::engine::camera::Camera;

pub struct ModelGame{
    renderer:Renderer,
    shaders:Vec<ShaderProgram>,
    fonts:Vec<Font>,
    meshes:Vec<Mesh>,
    camera:Camera,
}

impl ModelGame{
    pub fn new()->ModelGame {
        let renderer= Renderer::new();
        let shaders:Vec<ShaderProgram> = Vec::new();
        let mut fonts:Vec<Font> = Vec::new();
        let font = Font::new("NotoSansSC-Regular.otf",18);
        fonts.push(font);

        let meshes = Vec::new();
        let camera = Camera::new(4.0/3.0);
        ModelGame{renderer,shaders,fonts,meshes,camera}
    }
}

impl IGameLogic for ModelGame {
    fn init(&mut self) {
        self.renderer.init();

        let base_shader = ShaderProgram::new("base");
        self.shaders.push(base_shader);
        let font_shader = ShaderProgram::new("font");
        self.shaders.push(font_shader);

        let data:[f32;44] = [
            -1.0,1.0,-1.0  ,0.0,0.0,1.0  ,0.0,0.0  ,1.0,0.0,0.0,//左上角
            1.0,1.0,-1.0   ,0.0,0.0,1.0  ,1.0,0.0  ,0.0,1.0,0.0,//右上角
            -1.0,-1.0,-1.0 ,0.0,0.0,1.0  ,0.0,1.0  ,0.5,0.5,0.5,//左下角
            1.0,-1.0,-1.0  ,0.0,0.0,1.0  ,1.0,1.0  ,0.0,0.0,1.0,//右下角
        ];

        let indices:[u16;6] = [0,2,3,0,3,1];

        let mesh = Mesh::from_data(&data,&indices);

        self.meshes.push(mesh)
    }

    fn input(&mut self,window:&Window) {

    }

    fn update(&mut self, window: &Window, interval: f32) {
    }

    fn render(&mut self, window: &Window) {
        self.renderer.clear_color(162.0/255.0,155.0/255.0,124.0/255.0);

        let base_shader = &self.shaders[0];
        self.renderer.render_mesh(&self.camera,&self.meshes,base_shader);

        let font_shader = &self.shaders[1];
        let font_noto = &mut self.fonts[0];
        let font_color:(f32,f32,f32) = (179.0/255.0,0.0,0.0);
        self.renderer.render_text((5.0,690.0),font_color,String::from("Powered By Rust\u{00A9}"),font_noto,font_shader)
    }
}

impl Drop for ModelGame {
    fn drop(&mut self) {
        println!("Drop ModelGame")
    }
}