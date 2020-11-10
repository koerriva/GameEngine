use crate::engine::logic::IGameLogic;
use crate::engine::window::Window;
use crate::engine::renderer::Renderer;
use crate::engine::graph::shader::ShaderProgram;
use crate::engine::font::{Font};
use nalgebra_glm::vec3;

pub struct DummyGame{
    renderer:Renderer,
    fonts:Vec<Font>
}

impl DummyGame{
    pub fn new()->DummyGame {
        let renderer= Renderer::new(0,0);
        let mut fonts:Vec<Font> = Vec::new();
        let font = Font::new("LiuJianMaoCao-Regular.ttf",72);
        fonts.push(font);
        let font = Font::new("NotoSansSC-Regular.otf",18);
        fonts.push(font);
        DummyGame{renderer,fonts}
    }
}

impl IGameLogic for DummyGame {
    fn init(&mut self) {
        self.renderer.init();
        // for font in &mut self.fonts {
        //     font.init()
        // }

        let font_shader = ShaderProgram::new("font");
        for font in &mut self.fonts {
            font.shader = Some(font_shader);
        }
    }

    fn input(&mut self,window:&Window) {

    }

    fn update(&mut self, window: &Window, interval: f32) {
    }

    fn render(&mut self, window: &Window) {
        self.renderer.set_view_size(window.frame_buffer_size.0,window.frame_buffer_size.1);
        self.renderer.clear_color(162.0/255.0,155.0/255.0,124.0/255.0);

        let font_maocao = &mut self.fonts[0];
        let font_color = vec3(80.0/255.0,97.0/255.0,109.0/255.0);

        let mut line = 72.0;
        self.renderer.render_text((500.0,line),&font_color,String::from("夏日绝句"),font_maocao);
        line += font_maocao.height as f32+18.0;
        self.renderer.render_text((670.0,line),&font_color,String::from("宋 李清照"),font_maocao);
        line += font_maocao.height as f32+36.0;
        self.renderer.render_text((460.0,line),&font_color,String::from("生当做人杰"),font_maocao);
        line += font_maocao.height as f32+18.0;
        self.renderer.render_text((460.0,line),&font_color,String::from("死亦为鬼雄"),font_maocao);
        line += font_maocao.height as f32+18.0;
        self.renderer.render_text((460.0,line),&font_color,String::from("至今思项羽"),font_maocao);
        line += font_maocao.height as f32+18.0;
        self.renderer.render_text((460.0,line),&font_color,String::from("不肯过江东"),font_maocao);

        let font_noto = &mut self.fonts[1];
        let font_color = vec3(179.0/255.0,0.0,0.0);
        self.renderer.render_text((5.0,690.0),&font_color,String::from("Powered By Rust\u{00A9}"),font_noto)
    }
}