use crate::engine::window::Window;

pub trait IGameLogic{
    fn init(&mut self);
    fn input(&mut self,window:&Window);
    fn update(&mut self,window:&Window,interval:f32);
    fn render(&mut self,window:&Window);
}