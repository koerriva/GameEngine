use crate::engine::window::Window;

pub trait IGameLogic{
    fn input(&self);
    fn update(&self,window:&Window,interval:f32);
    fn render(&self,window:&Window);
}