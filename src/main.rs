use gl::types::*;
use crate::engine::window::Window;
use crate::engine::core::GameEngine;
use crate::engine::logic::IGameLogic;
use crate::game::dummy_game::DummyGame;
use crate::engine::resource::ResourceLoader;
use crate::engine::font::Font;

mod engine;
mod game;

fn main() {
    ResourceLoader::init();
    // let shader_code = ResourceLoader::load_shader("terrain.vert");
    // println!("shader code : {:?}",shader_code);
    // let font = Font::new("NotoSansSC-Thin.otf");
    // let text = String::from("生当作人杰，死亦为鬼雄。至今思项羽，不肯过江东。");
    // for char in text.chars() {
    //     let c = font.read(char as usize);
    //     println!("char {},{:?}",char,c);
    // }
    let mut game = Box::new(DummyGame::new());
    let mut engine = GameEngine::new(1280,720,game);
    engine.run();
}
