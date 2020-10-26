#include <iostream>
#include <glad.h>
#include <unordered_map>
#include <vector>

#define PI 3.14159265358979323846

#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>

#define STB_IMAGE_IMPLEMENTATION
#include <stb/stb_image.h>

#include "Engine/Utils/Logger.hpp"
#include "Engine/Utils/ResourceLoader.hpp"
#include "Engine/Window.hpp"
#include "Engine/GameEngine.hpp"
#include "Game/DummyGame.hpp"

using namespace std;
using namespace Engine;
using namespace Engine::Utils;
using namespace Game;

int main(){
    Logger::Info("GameEngine Version 0.1");
    ResourceLoader resourceLoader;
    resourceLoader.Init();

    Window window("我的游戏引擎",800,600,true);
    Renderer renderer;
    DummyGame game(&renderer,&resourceLoader);
    GameEngine engine(&window,&game,&resourceLoader);
    engine.Run();

    resourceLoader.Cleanup();
    return 0;
}