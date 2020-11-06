use crate::engine::window::Window;
use crate::engine::logic::IGameLogic;

pub struct GameEngine{
    window:Window,
    game: Box<dyn IGameLogic>
}

impl GameEngine {
    pub fn new(width:u32, height:u32, game: Box<dyn IGameLogic>) -> Box<GameEngine> {
        let window = Window::new(width,height);
        Box::new(GameEngine{window,game})
    }

    pub fn run(&self)->(){
        loop {

        }
    }
}