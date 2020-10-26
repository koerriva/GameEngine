#pragma once

#include "Graph/ShaderProgram.hpp"
#include "Graph/Mesh.hpp"
#include "Graph/Texture.hpp"

namespace Engine{
    using namespace Graph;
    class Renderer
    {
    private:
        /* data */
    public:
        Renderer(/* args */);
        ~Renderer();

        void Init();
        void Render(const Window* window,const vector<Mesh>& meshList,ShaderProgram* shaderProgram);
    };

    Renderer::Renderer(/* args */)
    = default;

    Renderer::~Renderer()
    {
        cout << "Drop Renderer" << endl;
    }

    void Renderer::Init(){
        glClearColor(0.f,0.f,0.f,1.0f);
        glClear(GL_DEPTH_BUFFER_BIT|GL_COLOR_BUFFER_BIT);
    }

    void Renderer::Render(const Window* window,const vector<Mesh>& meshList,ShaderProgram* shaderProgram){
        glViewport(0,0,window->GetWidth(),window->GetHeight());
        glClearColor(0.f,0.f,0.f,1.0f);
        glClear(GL_DEPTH_BUFFER_BIT|GL_COLOR_BUFFER_BIT);
        glEnable(GL_DEPTH_TEST);
        glEnable(GL_CULL_FACE);
        glCullFace(GL_BACK);
        if(window->GetKeyPressed(F1)){
            glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);
        }else{
            glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);
        }

        shaderProgram->Bind();
        auto time = (float)Window::GetTimeInSecond();
        shaderProgram->SetFloat("time",time);

        for (auto& mesh:meshList){
            mesh.Draw();
        }
        Engine::Graph::ShaderProgram::Unbind();
    }
}
