#pragma once

#include "IGameLogic.hpp"
#include "Renderer.hpp"

namespace Engine {
    class GameEngine
    {
    private:
        Window* window;
        IGameLogic* game;
        Utils::ResourceLoader* resourceLoader;
    public:
        GameEngine(Window* window,IGameLogic* game,Utils::ResourceLoader* resourceLoader);
        ~GameEngine();

        void Run();

    protected:
        void Init();
        void Input();
        void Update();
        void Render();
        void Cleanup();
    };

    GameEngine::GameEngine(Window* window,IGameLogic* game,Utils::ResourceLoader* resourceLoader)
    {
        this->game = game;
        this->window = window;
        this->resourceLoader = resourceLoader;
    }

    GameEngine::~GameEngine()
    {
        cout << "Drop GameEngine" << endl;
    }

    void GameEngine::Init(){
        window->Init();
        game->Init();
    }

    void GameEngine::Run(){
        Init();
        while (!window->Closed())
        {
            Input();
            
            Update();

            Render();
        }
        Cleanup();
    }

    void GameEngine::Input(){
        game->Input(window);
    }

    void GameEngine::Update(){
        window->Update();
        game->Update(1/60.0f);
    }

    void GameEngine::Render(){
        game->Render(window);
    }

    void GameEngine::Cleanup(){
        game->Cleanup();

        //窗口必须最后清理，防止OpenGL Context关闭。
        window->Cleanup();
        game->Cleanup();
    }
}