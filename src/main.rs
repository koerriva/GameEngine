use gl::types::*;
use crate::engine::window::Window;
use crate::engine::core::GameEngine;
use crate::engine::logic::IGameLogic;
use crate::game::dummy_game::DummyGame;

mod engine;
mod game;

fn main() {
    let mut game = Box::new(DummyGame::new());
    let mut engine = GameEngine::new(1280,720,game);
    engine.run();
}
