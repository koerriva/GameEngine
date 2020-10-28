#include <iostream>
#include <glad.h>
#include <unordered_map>
#include <vector>

#define PI 3.14159265358979323846
#define EARTH_RADIUS 6300.f

#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>

#define STB_IMAGE_IMPLEMENTATION
#include <stb/stb_image.h>
#include <ft2build.h>
#include FT_FREETYPE_H

#include "Engine/Utils/Random.hpp"
#include "Engine/Utils/Logger.hpp"
#include "Engine/Utils/ResourceLoader.hpp"

#include "Engine/Font.hpp"
#include "Engine/Debug.hpp"
#include "Engine/Camera.hpp"
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
    window.Init();
    Font::Init();
    ResourceLoader::Init();

    Renderer renderer;
    DummyGame game(&renderer);
    GameEngine engine(&window,&game);
    engine.Run();

    ResourceLoader::Cleanup();
    return 0;
}