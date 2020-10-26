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
        ShaderProgram* shaderProgram = nullptr;
        vector<Mesh> meshList;

        float color=0.1f;
    public:
        DummyGame(Renderer* renderer,Utils::ResourceLoader* resourceLoader);
        ~DummyGame();

        void Init() override;
        void Input(Window* window) override;
        void Update(float interval) override;
        void Render(Window* window) override;
        void Cleanup() override;
    };

    DummyGame::DummyGame(Renderer* renderer,Utils::ResourceLoader* resourceLoader){
        this->renderer = renderer;
        this->resourceLoader = resourceLoader;
    }

    DummyGame::~DummyGame(){
        cout << "Drop DummyGame" << endl;
    }

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
        //up-left
        vertices.push_back(-1.0f);
        vertices.push_back(1.0f);
        vertices.push_back(0.0f);
        //up-right
        vertices.push_back(1.0f);
        vertices.push_back(1.0f);
        vertices.push_back(0.0f);
        //bottom-right
        vertices.push_back(1.0f);
        vertices.push_back(-1.0f);
        vertices.push_back(0.0f);
        //bottom-left
        vertices.push_back(-1.0f);
        vertices.push_back(-1.0f);
        vertices.push_back(0.0f);
        vector<unsigned int> indices;
        indices.push_back(0);
        indices.push_back(2);
        indices.push_back(1);//逆时针为前,顺时针为后
        indices.push_back(0);
        indices.push_back(3);
        indices.push_back(2);
//        indices.push_back(3);

//        meshList.emplace_back(vertices,indices);
        meshList.push_back(Mesh::Sphere(36,18));
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
        renderer->Render(window,meshList,shaderProgram);
    }

    void DummyGame::Cleanup(){
        for(auto& mesh:meshList){
            mesh.Cleanup();
        }

        shaderProgram->Cleanup();
        delete shaderProgram;
    }
}

