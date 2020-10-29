#pragma once
#include <chrono>

#include "IGameLogic.hpp"
#include "Renderer.hpp"

namespace Engine {
    using namespace std::chrono;

    //duration_cast<nanoseconds>(high_resolution_clock::now().time_since_epoch());
    //1ms = 1000000ns
    struct GameTime{
        nanoseconds start;
        nanoseconds now;
        nanoseconds steps;
    };
    class GameEngine
    {
    private:
        const float TARGET_FPS = 120;
        const float TARGET_UPS = 60;
        GameTime time{nanoseconds(0),nanoseconds(0)};

        Window* window;
        IGameLogic* game;
    public:
        GameEngine(Window* window,IGameLogic* game);
        ~GameEngine();

        void Run();

    protected:
        void Init();
        void Input();
        void Update(float interval);
        void Render();
        void Sync(nanoseconds current) const;
        void Cleanup();
    };

    GameEngine::GameEngine(Window* window,IGameLogic* game)
    {
        this->game = game;
        this->window = window;
    }

    GameEngine::~GameEngine()
    {
        cout << "Drop GameEngine" << endl;
    }

    void GameEngine::Init(){
        game->Init();
    }

    void GameEngine::Run(){
        Init();

        time.start = duration_cast<nanoseconds>(high_resolution_clock::now().time_since_epoch());
        time.now = duration_cast<nanoseconds>(high_resolution_clock::now().time_since_epoch());
        while (!window->Closed())
        {
            auto loopStartTime = duration_cast<nanoseconds>(high_resolution_clock::now().time_since_epoch());
            auto elapsed = loopStartTime-time.now;
            time.now = loopStartTime;
            time.steps += elapsed;

            Input();
            long nanoPerUpdate = long(1.f/TARGET_UPS*1000.f*1000000.f);
            while (time.steps>=nanoseconds(nanoPerUpdate)){
                Update(1.f/TARGET_UPS);
                time.steps -=nanoseconds(nanoPerUpdate);
            }

//            Update(1.f/TARGET_UPS);

            Render();

            Sync(loopStartTime);
        }
        Cleanup();
    }

    void GameEngine::Sync(nanoseconds current) const {
        long nanoPerRender = long(1.f/TARGET_UPS*1000.f*1000000.f);
        auto loopSlot = nanoseconds(nanoPerRender);
        nanoseconds endTime = current+loopSlot;
        while(duration_cast<nanoseconds>(high_resolution_clock::now().time_since_epoch())<endTime){
            this_thread::sleep_for(microseconds(1));
        }
    }

    void GameEngine::Input(){
        game->Input(window);
    }

    void GameEngine::Update(float interval){
        window->Update();
        game->Update(interval);
    }

    void GameEngine::Render(){
        game->Render(window);
    }

    void GameEngine::Cleanup(){
        game->Cleanup();

        //窗口必须最后清理，防止OpenGL Context关闭。
        window->Cleanup();
    }
}