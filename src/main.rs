#[warn(unused_imports)]
use gl::types::*;
use crate::engine::window::Window;
use crate::engine::core::GameEngine;
use crate::engine::logic::IGameLogic;
use crate::game::dummy_game::DummyGame;
use crate::engine::resource::ResourceLoader;
use crate::engine::font::Font;
use std::fmt::Error;
use crate::engine::graph::mesh::{Mesh,VertexAttr};
use crate::game::model_game::ModelGame;
use gltf::Semantic;
use std::process::id;

mod engine;
mod game;

fn main(){
    ResourceLoader::init();
    let game = Box::new(ModelGame::new());
    let mut engine = GameEngine::new(1280,720,game);
    engine.run();
}

#[test]
fn test_font(){
    let font = Font::new("NotoSansSC-Thin.otf",18);
    let text = String::from("生当作人杰，死亦为鬼雄。至今思项羽，不肯过江东。");
    for char in text.chars() {
        let c = font.read(char as usize);
        println!("char {},{:?}",char,c);
    }
}

#[test]
fn test_resource_load(){
    let shader_code = ResourceLoader::load_shader("terrain.vert");
    println!("shader code : {:?}",shader_code);
}
