#pragma once
#include "Engine/Graph/ShaderProgram.hpp"
#include "Engine/Graph/Mesh.hpp"

namespace Engine{
    using namespace Graph;
    using namespace glm;
    class Debug{
    private:
    public:
        Debug(){
        }

        void Draw(vec2 pos,const string& text,vec3 color){
            mat4 P = ortho(0.0f,800.f,600.f,0.f);
            ShaderProgram shaderProgram("font");
            shaderProgram.Bind();
            shaderProgram.SetVec3("color", reinterpret_cast<float *>(&color));

            Character c = chars['c'];
            float x = pos.x+c.bearing.x*1.0f;
            float y = pos.y-(c.size.y-c.bearing.y)*1.0f;
            float w = c.size.x*1.f;
            float h = c.size.y*1.f;
            //4个顶点
            vector<float> vertices;
            vector<float> texCoords;
            for (int i = 0; i < 4; ++i) {
                vertices.push_back(x+i%2*w);
                if(i>1){
                    vertices.push_back(y+h);
                }else{
                    vertices.push_back(y);
                }
                vertices.push_back(0.f);

                texCoords.push_back(0.f+i%2*1.f);
                if(i>1){
                    texCoords.push_back(1.f);
                }else{
                    texCoords.push_back(0.f);
                }
            }
            vector<unsigned int> indices;
            indices.push_back(0);
            indices.push_back(2);
            indices.push_back(1);
            indices.push_back(1);
            indices.push_back(2);
            indices.push_back(3);
            vector<float> normals;

            Mesh mesh(vertices,indices,normals,texCoords);

            shaderProgram.Unbind();
            shaderProgram.Cleanup();
        };
    };
}