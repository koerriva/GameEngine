#pragma once

namespace Game{
    using namespace Engine;
    using namespace Engine::Utils;
    using namespace Engine::Graph;
    using namespace glm;

    class DummyGame:public IGameLogic
    {
    private:
        Renderer* renderer;
        ShaderProgram* shaderProgram = nullptr;
        Camera* camera = nullptr;
        vector<Mesh> meshList;
        vector<Texture> textures;
        Debug* debug;

        vec2 cameraState {0.f,0.f};
    public:
        DummyGame(Renderer* renderer);
        ~DummyGame();

        void Init() override;
        void Input(Window* window) override;
        void Update(float interval) override;
        void Render(Window* window) override;
        void Cleanup() override;
    };

    DummyGame::DummyGame(Renderer* renderer){
        this->renderer = renderer;
    }

    DummyGame::~DummyGame(){
        cout << "Drop DummyGame" << endl;
    }

    void DummyGame::Init(){
        Logger::Info("DummyGame Init...");
        renderer->Init();
        
        this->shaderProgram = new Graph::ShaderProgram("base");

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
        meshList.push_back(Mesh::Sphere(EARTH_RADIUS,72,36));

        int len;
        const unsigned char* buffer = ResourceLoader::LoadTexture("earthmap1k.jpg",&len);
        
        auto tex = Texture(buffer,len);
        textures.push_back(tex);

        this->camera = new Camera(vec3{0,0,0});

        debug = new Debug();
    }

    void DummyGame::Input(Window* window){
        if(window->GetKeyPressed(KeyCode::ESC)){
            window->Close();
        }

        cameraState.x = 0.f;
        cameraState.y = 0.f;
        if(window->GetKeyPressed(KeyCode::W)){
            cameraState.x = 1.f;
        }
        if(window->GetKeyPressed(KeyCode::S)){
            cameraState.x = -1.f;
        }
        if(window->GetKeyPressed(KeyCode::D)){
            cameraState.y = 1.f;
        }
        if(window->GetKeyPressed(KeyCode::A)){
            cameraState.y = -1.f;
        }

        if(window->GetKeyPressed(F1)){
            renderer->SetMode();
        }
    }

    void DummyGame::Update(float interval){
        camera->MoveForward(cameraState.x*interval*1000);
        camera->MoveRight(cameraState.y*interval*1000);
    }

    void DummyGame::Render(Window* window){
        glClear(GL_DEPTH_BUFFER_BIT|GL_COLOR_BUFFER_BIT);
//        debug->Draw(vec2{25,25},"c",vec3{0.9,.1,.1});
        renderer->Render(window,camera,meshList,textures,shaderProgram);
        debug->Draw(vec2{25,25},L"abc你好世界",vec3{0.9,.1,.1});

    }

    void DummyGame::Cleanup(){
        for(auto& mesh:meshList){
            mesh.Cleanup();
        }

        for(auto& tex:textures){
            tex.Cleanup();
        }

        shaderProgram->Cleanup();
        delete shaderProgram;

        delete camera;
    }
}

