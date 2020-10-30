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
        Debug* debug = nullptr;
        vector<Mesh> meshList;
        vector<Texture> textures;
        Timer* timer;

        float updateRate = 0.f;
        float frameRate = 0.f;
        vec2 cameraState {0.f,0.f};
    public:
        DummyGame();
        ~DummyGame();

        void Init() override;
        void Input(Window* window) override;
        void Update(float interval) override;
        void Render(Window* window) override;
        void Cleanup() override;
    };

    DummyGame::DummyGame(){
        this->renderer = new Renderer();
        this->timer = new Timer();
    }

    DummyGame::~DummyGame(){
        cout << "Drop DummyGame" << endl;
    }

    void DummyGame::Init(){
        Logger::Info("DummyGame Init...");
        renderer->Init();
        
        this->shaderProgram = new Graph::ShaderProgram("base");
        meshList.push_back(Mesh::Sphere(EARTH_RADIUS,72,36));

        int len;
        const unsigned char* buffer = ResourceLoader::LoadTexture("earthmap1k.jpg",&len);
        
        auto tex = Texture(buffer,len);
        textures.push_back(tex);

        this->camera = new Camera(vec3{0,0,0});

        debug = new Debug();

        timer->Init();
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
            renderer->SetShaderMode();
        }
        if(window->GetKeyPressed(F2)){
            renderer->SetWireframeMode();
        }
    }

    void DummyGame::Update(float interval){
        camera->MoveForward(cameraState.x*interval*1000);
        camera->MoveRight(cameraState.y*interval*1000);
        updateRate = interval;
    }

    void DummyGame::Render(Window* window){
        frameRate = float(timer->GetElapsedTime());

        glClear(GL_DEPTH_BUFFER_BIT|GL_COLOR_BUFFER_BIT);
        glViewport(0,0,window->GetFrameBufferWidth(),window->GetFrameBufferHeight());
        renderer->Render(window,camera,meshList,textures,shaderProgram);
        debug->Draw(vec2{5,5},Text("帧率:"+to_string(int(1/frameRate))),vec3{0.05f,.99f,0.05f});
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

        delete renderer;

        delete timer;
    }
}

