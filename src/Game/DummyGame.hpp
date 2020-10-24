#pragma once

namespace Game{
    using namespace Engine;
    class DummyGame:public IGameLogic
    {
    private:
        Renderer* renderer;

        float color=0.1f;
    public:
        DummyGame(Renderer* renderer);
        ~DummyGame();

        void Init();
        void Input(Window* window);
        void Update(float interval);
        void Render(Window* window);
    };

    DummyGame::DummyGame(Renderer* renderer){
        this->renderer = renderer;
    }

    DummyGame::~DummyGame(){}

    void DummyGame::Init(){
        renderer->Init();
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

