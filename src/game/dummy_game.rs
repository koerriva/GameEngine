use crate::engine::logic::IGameLogic;
use crate::engine::window::Window;
use crate::engine::renderer::Renderer;
use crate::engine::graph::shader::ShaderProgram;
use crate::engine::font::{Font};

pub struct DummyGame{
    renderer:Renderer,
    shaders:Vec<ShaderProgram>,
    fonts:Vec<Font>
}

impl DummyGame{
    pub fn new()->DummyGame {
        let renderer= Renderer::new();
        let shaders:Vec<ShaderProgram> = Vec::new();
        let mut fonts:Vec<Font> = Vec::new();
        let font = Font::new("LiuJianMaoCao-Regular.ttf",72);
        fonts.push(font);
        let font = Font::new("NotoSansSC-Regular.otf",18);
        fonts.push(font);
        DummyGame{renderer,shaders,fonts}
    }
}

impl IGameLogic for DummyGame {
    fn init(&mut self) {
        self.renderer.init();

        let terrain_shader = ShaderProgram::new("terrain");
        self.shaders.push(terrain_shader);
        let font_shader = ShaderProgram::new("font");
        self.shaders.push(font_shader);

        for font in &mut self.fonts {
            font.init()
        }
    }

    fn input(&self,window:&Window) {

    }

    fn update(&self, window: &Window, interval: f32) {
    }

    fn render(&self, window: &Window) {
        self.renderer.viewport(window.width, window.height);
        self.renderer.clear_color(162.0/255.0,155.0/255.0,124.0/255.0);

        let font_maocao = &self.fonts[0];
        let font_noto = &self.fonts[1];
        let font_shader = &self.shaders[1];
        let font_color:(f32,f32,f32) = (80.0/255.0,97.0/255.0,109.0/255.0);

        let mut line = 70.0;
        self.renderer.render((500.0,line),font_color,String::from("夏日绝句"),font_maocao,font_shader);
        line += font_maocao.height as f32+15.0;
        self.renderer.render((670.0,line),font_color,String::from("宋 李清照"),font_maocao,font_shader);
        line += font_maocao.height as f32+35.0;
        self.renderer.render((460.0,line),font_color,String::from("生当做人杰"),font_maocao,font_shader);
        line += font_maocao.height as f32+25.0;
        self.renderer.render((460.0,line),font_color,String::from("死亦为鬼雄"),font_maocao,font_shader);
        line += font_maocao.height as f32+25.0;
        self.renderer.render((460.0,line),font_color,String::from("至今思项羽"),font_maocao,font_shader);
        line += font_maocao.height as f32+25.0;
        self.renderer.render((460.0,line),font_color,String::from("不肯过江东"),font_maocao,font_shader);

        let font_color:(f32,f32,f32) = (179.0/255.0,0.0,0.0);
        self.renderer.render((5.0,690.0),font_color,String::from("Powered By Rust\u{00A9}"),font_noto,font_shader)
    }
}