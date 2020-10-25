#pragma once

#include "Graph/ShaderProgram.hpp"
#include "Graph/Mesh.hpp"

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
        void Render(const Window* window,const Mesh* mesh,const ShaderProgram* shaderProgram);
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

    void Renderer::Render(const Window* window,const Mesh* mesh,const ShaderProgram* shaderProgram){
        glViewport(0,0,window->GetWidth(),window->GetHeight());
        glClearColor(0.f,0.f,0.f,1.0f);
        glClear(GL_DEPTH_BUFFER_BIT|GL_COLOR_BUFFER_BIT);

        shaderProgram->Bind();
        mesh->Draw();
//        Engine::Graph::ShaderProgram::Unbind();
    }

    void Renderer::Render(Mesh* mesh,ShaderProgram* shaderProgram){
        shaderProgram->Bind();
        mesh->Draw();
        shaderProgram->Unbind();
    }
}
