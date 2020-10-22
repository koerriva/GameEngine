#include <iostream>
#include "Engine/Window.h"
#include "Engine/GameEngine.hpp"
#include "Game/DummyGame.hpp"

using namespace std;

int main(){
    cout << "Welcome GameEngine 0.1" << endl;
    Window window;
    DummyGame game;
    GameEngine engine(&window,&game);

    engine.run();
    return 0;
}