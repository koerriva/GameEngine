#pragma once

#include "Graph/ShaderProgram.hpp"
#include "Graph/Mesh.hpp"
#include "Graph/Texture.hpp"

namespace Engine{
    using namespace Graph;
    class Renderer
    {
    private:
        bool WIREFRAME_MODE = false;
        bool SHADER_MODE = true;
    public:
        Renderer(/* args */);
        ~Renderer();

        void Init();
        void SetWireframeMode();
        void SetShaderMode();
        void Render(const Window* window,const Camera* camera,const vector<Mesh>& meshList,const vector<Texture>& textures,ShaderProgram* shaderProgram);
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

    void Renderer::SetWireframeMode(){
        WIREFRAME_MODE = true;
        SHADER_MODE = false;
    }

    void Renderer::SetShaderMode() {
        WIREFRAME_MODE = false;
        SHADER_MODE = true;
    }

    void Renderer::Render(const Window* window,const Camera* camera,const vector<Mesh>& meshList,const vector<Texture>& textures,ShaderProgram* shaderProgram){
        glEnable(GL_DEPTH_TEST);

//        glEnable(GL_CULL_FACE);
//        glCullFace(GL_BACK);
        if(WIREFRAME_MODE){
            glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);
        }
        if(SHADER_MODE){
            glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);
        }

        glActiveTexture(GL_TEXTURE0);
        textures[0].Bind();

        shaderProgram->Bind();
        auto time = (float)Window::GetTimeInSecond();
        shaderProgram->SetFloat("time",time);

        float aspect = window->GetAspect();
        glm::mat4 P,V(1.0f),M(1.0f);
        P = glm::perspective(glm::radians(60.f),aspect,.1f,99999.f);
        V = camera->GetViewMatrix();
        M = glm::rotate(M,time,glm::vec3(0,1,0));
        M = glm::scale(M,glm::vec3(1.0f));

        shaderProgram->SetMat4("P", reinterpret_cast<float *>(&P));
        shaderProgram->SetMat4("V", reinterpret_cast<float *>(&V));
        shaderProgram->SetMat4("M", reinterpret_cast<float *>(&M));

        for (size_t i = 0; i < meshList.size(); i++)
        {
            auto& mesh = meshList[i];
            mesh.Draw();
        }
        
        Engine::Graph::ShaderProgram::Unbind();

        glDisable(GL_DEPTH_TEST);
    }
}
