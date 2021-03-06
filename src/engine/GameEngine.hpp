#pragma once
#include <chrono>
#include <thread>

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

    class Timer {
    private:
        double lastLoopTime;
    public:
        ~Timer(){
            cout << "Drop Timer" << endl;
        }
        void Init(){
            lastLoopTime = GetTime();
        }

        double GetElapsedTime(){
            double time = GetTime();
            auto elapsedTime = time - lastLoopTime;
            lastLoopTime = time;
            return elapsedTime;
        }

        [[nodiscard]] double GetLastLoopTime() const{
            return lastLoopTime;
        }

        static double GetTime(){
            return duration_cast<nanoseconds>(steady_clock::now().time_since_epoch()).count()/1000000000.0;
        }
    };

    class GameEngine
    {
    private:
        const float TARGET_FPS = 60;
        const float TARGET_UPS = 30;

        Window* window;
        IGameLogic* game;
        Timer* timer;
    public:
        GameEngine(const char* title,int width,int height,bool vsync,IGameLogic* game);
        ~GameEngine();

        void Run();

    protected:
        void Init();
        void Input();
        void Update(float elapsedTime);
        void Render(float elapsedTime);
        void Sync();
        void Cleanup();
    };

    GameEngine::GameEngine(const char *title, int width, int height, bool vsync, IGameLogic *game) {
        this->game = game;
        this->window = new Window(title,width,height,vsync);
        this->timer = new Timer();
    }

    GameEngine::~GameEngine()
    {
        cout << "Drop GameEngine" << endl;
    }

    void GameEngine::Init(){
        timer->Init();
        //窗口先初始化
        window->Init();
        //加载资源
        ResourceLoader::Init();
        //加载字体
        Font::Init();
        //加载游戏
        game->Init();
    }

    void GameEngine::Run(){
        Init();
        float elapsedTime;
        float acc = 0.f;
        float interval = 1.f/TARGET_UPS;
        while (!window->Closed())
        {
            elapsedTime = float(timer->GetElapsedTime());
            acc += elapsedTime;
            Input();
            if(!window->VSynced()){
                while (acc >= interval){
                    Update(interval);
                    acc -= interval;
                }
            }else{
                Update(elapsedTime);
            }

            Render(elapsedTime);
            if(!window->VSynced()){
                Sync();
            }
        }
        Cleanup();
    }

    void GameEngine::Sync() {
        auto loopSlot = 1.f/TARGET_FPS;
        auto endTime = timer->GetLastLoopTime()+loopSlot;
        auto now = Engine::Timer::GetTime();
        while(now<endTime){
            this_thread::sleep_for(milliseconds(1));
            now = Engine::Timer::GetTime();
        }
    }

    void GameEngine::Input(){
        game->Input(window);
    }

    void GameEngine::Update(float elapsedTime){
        window->Update();
        game->Update(elapsedTime);
    }

    void GameEngine::Render(float elapsedTime){
        game->Render(window,elapsedTime);
    }

    void GameEngine::Cleanup(){
        game->Cleanup();

        ResourceLoader::Cleanup();
        Font::Cleanup();

        //窗口必须最后清理，防止OpenGL Context关闭。
        window->Cleanup();
    }
}