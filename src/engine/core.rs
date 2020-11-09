use crate::engine::window::Window;
use crate::engine::logic::IGameLogic;

pub struct GameEngine{
    game: Box<dyn IGameLogic>,
    window:Window,
}

impl GameEngine {
    pub fn new(width:u32, height:u32, game: Box<dyn IGameLogic>) -> Box<GameEngine> {
        let window = Window::new(width,height,"Tech Demo");
        Box::new(GameEngine{window,game})
    }

    pub fn run(&mut self) ->(){
        self.init();

        while !self.window.closed {
            self.input();
            self.update();
            self.render();
        }
    }

    fn init(&mut self){
        self.game.init();
    }

    fn input(&mut self){
        self.game.input(&self.window);
    }

    fn update(&mut self){
        self.window.update();
        self.game.update(&self.window,1.0/60.0);
    }

    fn render(&mut self){
        self.game.render(&self.window);
    }
}

impl Drop for GameEngine {
    fn drop(&mut self) {
        println!("Drop GameEngine");
    }
}