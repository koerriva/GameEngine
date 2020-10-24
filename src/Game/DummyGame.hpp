#pragma once

namespace Game{
    using namespace Engine;
    class DummyGame:public IGameLogic
    {
    private:
        Renderer* renderer;
        Utils::ResourceLoader* resourceLoader;

        float color=0.1f;
    public:
        DummyGame(Renderer* renderer,Utils::ResourceLoader* resourceLoader);
        ~DummyGame();

        void Init();
        void Input(Window* window);
        void Update(float interval);
        void Render(Window* window);
    };

    DummyGame::DummyGame(Renderer* renderer,Utils::ResourceLoader* resourceLoader){
        this->renderer = renderer;
        this->resourceLoader = resourceLoader;
    }

    DummyGame::~DummyGame(){}

    void DummyGame::Init(){
        Utils::Logger::Info("DummyGame Init...");
        renderer->Init();
        resourceLoader->LoadVertexShader("base");
    }

    void DummyGame::Input(Window* window){
        if(window->GetKeyPressed(KeyCode::ESC)){
            window->Close();
        }
        if(window->GetKeyPressed(KeyCode::F12)){
            if(color<1.0)color+=0.01f;
        }
    }

    void DummyGame::Update(float interval){

    }

    void DummyGame::Render(Window* window){
        renderer->Render(color);
    }
}

