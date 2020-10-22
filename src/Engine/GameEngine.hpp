#pragma once
#include "IGameLogic.hpp"

class GameEngine
{
private:
    Window* window;
    IGameLogic* game;
public:
    GameEngine(Window* window,IGameLogic* game);
    ~GameEngine();
    void run();
};

GameEngine::GameEngine(Window* window,IGameLogic* game)
{
    this->game = game;
    this->window = window;
}

GameEngine::~GameEngine()
{
}

void GameEngine::run(){
    while (true)
    {
        /* code */
    }
}
