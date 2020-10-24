#include <iostream>
#include <glad.h>
#include "Engine/Utils/Logger.hpp"
#include "Engine/Window.hpp"
#include "Engine/GameEngine.hpp"
#include "Game/DummyGame.hpp"

using namespace std;
using namespace Engine;
using namespace Engine::Utils;
using namespace Game;

int main(){
    Logger::Info("GameEngine Version 0.1");
    Window window("我的游戏引擎",800,600,true);
    Renderer renderer;
    DummyGame game(&renderer);
    GameEngine engine(&window,&game);

    engine.Run();
    return 0;
}