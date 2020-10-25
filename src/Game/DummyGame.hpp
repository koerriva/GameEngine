#pragma once

namespace Game{
    using namespace Engine;
    using namespace Engine::Utils;
    using namespace Engine::Graph;

    class DummyGame:public IGameLogic
    {
    private:
        Renderer* renderer;
        ResourceLoader* resourceLoader;
        ShaderProgram* shaderProgram;
        Mesh* mesh;

        float color=0.1f;
    public:
        DummyGame(Renderer* renderer,Utils::ResourceLoader* resourceLoader);
        ~DummyGame();

        void Init();
        void Input(Window* window);
        void Update(float interval);
        void Render(Window* window);
        void Cleanup();
    };

    DummyGame::DummyGame(Renderer* renderer,Utils::ResourceLoader* resourceLoader){
        this->renderer = renderer;
        this->resourceLoader = resourceLoader;
    }

    DummyGame::~DummyGame(){}

    void DummyGame::Init(){
        Logger::Info("DummyGame Init...");
        renderer->Init();
        const char* vertexSource = resourceLoader->LoadShader("base",Graph::VERTEX_SHADER);
        const char* fragSource = resourceLoader->LoadShader("base",Graph::FRAGMENT_SHADER);
        Logger::Info(vertexSource);
        Logger::Info(fragSource);
        
        this->shaderProgram = new Graph::ShaderProgram(vertexSource,fragSource);
        shaderProgram->Init();
        vector<float> vertices;
        //up
        vertices.push_back(0.0f);
        vertices.push_back(1.0f);
        vertices.push_back(0.0f);
        //right
        vertices.push_back(1.0f);
        vertices.push_back(-1.0f);
        vertices.push_back(0.0f);
        //left
        vertices.push_back(-1.0f);
        vertices.push_back(-1.0f);
        vertices.push_back(0.0f);
        this->mesh = new Mesh(&vertices,nullptr,nullptr);
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
        renderer->Render(mesh,shaderProgram);
    }

    void DummyGame::Cleanup(){
        shaderProgram->Cleanup();
        delete shaderProgram;
    }
}

