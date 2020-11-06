use crate::engine::renderer::MeshRenderer;
use crate::engine::logic::IGameLogic;
use crate::engine::window::Window;

pub struct DummyGame{
    renderer:MeshRenderer
}

impl DummyGame{
    pub fn new()->DummyGame {
        let renderer= MeshRenderer::new();
        DummyGame{renderer}
    }
}

impl IGameLogic for DummyGame {
    fn input(&self) {
        unimplemented!()
    }

    fn update(&self, window: &Window, interval: f32) {
        unimplemented!()
    }

    fn render(&self, window: &Window) {
        unimplemented!()
    }
}