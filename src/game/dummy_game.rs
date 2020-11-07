use crate::engine::logic::IGameLogic;
use crate::engine::window::Window;
use crate::engine::renderer::Renderer;
use crate::engine::graph::shader::ShaderProgram;

pub struct DummyGame{
    renderer:Renderer,
    shaders:Vec<ShaderProgram>
}

impl DummyGame{
    pub fn new()->DummyGame {
        let renderer= Renderer::new();
        let shaders:Vec<ShaderProgram> = Vec::new();
        DummyGame{renderer,shaders}
    }
}

impl IGameLogic for DummyGame {
    fn init(&mut self) {
        let terrain_shader = ShaderProgram::new("terrain");
        self.shaders.push(terrain_shader);
    }

    fn input(&self,window:&Window) {

    }

    fn update(&self, window: &Window, interval: f32) {
    }

    fn render(&self, window: &Window) {
        self.renderer.viewport(window.width, window.height);
        self.renderer.clear_color(0.1,0.2,0.3);
    }
}