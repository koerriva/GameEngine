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
        void Render();
        void Render(float color);
        void Render(Mesh* mesh,ShaderProgram* shaderProgram);
    };

    Renderer::Renderer(/* args */)
    {
    }

    Renderer::~Renderer()
    {
    }

    void Renderer::Init(){
        glViewport(0,0,640,480);
    }

    void Renderer::Render(){
        glClearColor(0.2f,0.3f,0.4f,1.0f);
        glClear(GL_DEPTH_BUFFER_BIT|GL_COLOR_BUFFER_BIT);
    }

    void Renderer::Render(float color){
        glClearColor(color,color,color,1.0f);
        glClear(GL_DEPTH_BUFFER_BIT|GL_COLOR_BUFFER_BIT);
    }

    void Renderer::Render(Mesh* mesh,ShaderProgram* shaderProgram){
        shaderProgram->Bind();
        mesh->Draw();
        shaderProgram->Unbind();
    }
}
