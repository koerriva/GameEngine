#pragma once
#include "Engine/Graph/ShaderProgram.hpp"
#include "Engine/Graph/Mesh.hpp"

namespace Engine{
    using namespace Graph;
    using namespace glm;
    class Debug{
    private:
        ShaderProgram* shaderProgram;
        Mesh* mesh;
        Character c;
    public:
        Debug(vec2 pos){
            shaderProgram = new ShaderProgram("font");
            c = chars['c'];

            float x = pos.x+c.bearing.x*1.0f;
            float y = pos.y-(c.size.y-c.bearing.y)*1.0f;
            float w = c.size.x*1.f;
            float h = c.size.y*1.f;

            Logger::Info("char info ({},{}),({},{})",x,y,w,h);
            //4个顶点
            float v[12] ={
                    x,y+h,0.0,
                    x,y,0.0,
                    x+w,y,0.0,
                    x+w,y+w,0.0
            };
            float t[8] = {
                    0.0,0.0,//0
                    0.0,1.0,//1
                    1.0,1.0,//2
                    1.0,0.0,//3
            };
            vector<float> vertices(begin(v),end(v));
            vector<float> texCoords(begin(t),end(t));
            unsigned int idx[6]={
                    0,1,3,3,1,2
            };
            vector<unsigned int> indices(begin(idx),end(idx));
            vector<float> normals;

            mesh = new Mesh(vertices,indices,normals,texCoords);
        }

        void Draw(const string& text,vec3 color){
            mat4 P = ortho(0.f,800.f,0.f,600.f);
            glViewport(0,0,800,600);
            glClearColor(0.1f,0.1f,0.1f,1.0f);
            glClear(GL_COLOR_BUFFER_BIT);
            glPolygonMode(GL_FRONT_AND_BACK,GL_LINE);
//            glEnable(GL_BLEND);
//            glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
//            glEnable(GL_CULL_FACE);
//            glEnable(GL_BLEND);
//            glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

            shaderProgram->Bind();
            shaderProgram->SetMat4("P", reinterpret_cast<float *>(&P));
            shaderProgram->SetVec3("color", reinterpret_cast<float *>(&color));

            glActiveTexture(GL_TEXTURE0);
            glBindTexture(GL_TEXTURE_2D,c.texture);
            mesh->Draw();
            glBindTexture(GL_TEXTURE_2D,0);

            Engine::Graph::ShaderProgram::Unbind();
        };
    };
}