use crate::engine::window::Window;

pub trait IGameLogic{
    fn init(&mut self);
    fn input(&self,window:&Window);
    fn update(&self,window:&Window,interval:f32);
    fn render(&self,window:&Window);
}