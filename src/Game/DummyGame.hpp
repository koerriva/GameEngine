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
        Camera* camera = nullptr;
        Debug* debug = nullptr;
        vector<Mesh> meshList;
        vector<Texture> textures;
        Timer* timer;
        Terrain* terrain;
        ShaderProgram* terrainShader;

        float updateRate = 0.f;
        float frameTime = 0.f;
        int frameCount = 0;
        int frameRate = 0;
        vec2 cameraState {0.f,0.f};
        vec2 cameraDirection{0.f,0.f};
        float cameraLen = 0;
        int LOD = 1;
    public:
        DummyGame();
        ~DummyGame();

        void Init() override;
        void Input(Window* window) override;
        void Update(float elapsedTime) override;
        void Render(Window* window,float elapsedTime) override;
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
        
//        this->shaderProgram = new Graph::ShaderProgram("base");
//        meshList.push_back(Mesh::Sphere(EARTH_RADIUS,72,36));
//
//        int len;
//        const unsigned char* buffer = ResourceLoader::LoadTexture("earthmap4k.jpg",&len);
//        auto tex = Texture(buffer,len);
//        textures.push_back(tex);
        this->terrain = new Terrain();
        this->terrain->Init();
        this->terrainShader = new ShaderProgram("terrain");

        this->camera = new Camera(vec3{0,25,10});
        this->camera->Rotate(0,70);
        cameraLen = length(camera->Position());

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

        cameraDirection.x = 0;
        cameraDirection.y = 0;
        if(window->GetMouseButtonPressed(M_RIGHT)){
            cameraDirection.x = static_cast<float>(window->GetMouseXOffset());
            cameraDirection.y = static_cast<float>(window->GetMouseYOffset());
        }
    }

    void DummyGame::Update(float interval){
        camera->MoveForward(cameraState.x*interval*2.f);
        camera->MoveRight(cameraState.y*interval*2.f);
        camera->Rotate(cameraDirection.x,cameraDirection.y);
        updateRate = interval;
        cameraLen = length(camera->Position());
        if(cameraLen<25){
            LOD = static_cast<int>(glm::clamp(25.f/cameraLen,1.f,5.f));
        }else{
            LOD = 1;
        }
        terrain->Update(LOD);
    }

    void DummyGame::Render(Window* window,float elapsedTime){
        frameTime += float(timer->GetElapsedTime());
        frameCount += 1;
        if(frameRate==0&&frameTime>0){
            frameRate = int(frameCount/frameTime);
        }

        glClear(GL_DEPTH_BUFFER_BIT|GL_COLOR_BUFFER_BIT);
        glViewport(0,0,window->GetFrameBufferWidth(),window->GetFrameBufferHeight());
        renderer->Render(window,camera,terrain,terrainShader);
        debug->Draw(vec2{5,5},Text("帧率:"+to_string(frameRate)+","+to_string(int(1/elapsedTime))),vec3{0.05f,.99f,0.05f});

        vec3 camPos = camera->Position();
        debug->Draw(vec2{5,25},Text("相机坐标:"+to_string(camPos.x)+","+to_string(camPos.y)+","+to_string(camPos.z)),vec3{0.05f,.99f,0.05f});
        vec3 camRot = camera->Rotation();
        debug->Draw(vec2{5,45},Text("相机角度:"+to_string(camRot.x)+","+to_string(camRot.y)+","+to_string(camRot.z)),vec3{0.05f,.99f,0.05f});

        debug->Draw(vec2{5,65},Text("视距:"+to_string(cameraLen)),vec3{0.05f,.99f,0.05f});

        debug->Draw(vec2{5,85},Text("LOD:"+to_string(LOD)),vec3{0.05f,.99f,0.05f});

        size_t chunks = terrain->GetChunkSize();
        debug->Draw(vec2{5,105},Text("块数:"+to_string(chunks)),vec3{0.05f,.99f,0.05f});

        if(frameTime>1.0){
            frameRate = int(frameCount/frameTime);
            frameTime=0;
            frameCount=0;
        }
    }

    void DummyGame::Cleanup(){
        for(auto& mesh:meshList){
            mesh.Cleanup();
        }

        for(auto& tex:textures){
            tex.Cleanup();
        }

        terrain->Cleanup();

        delete terrain;

        delete terrainShader;

        delete camera;

        delete renderer;

        delete timer;
    }
}