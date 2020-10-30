#pragma once
#include "Engine/Graph/ShaderProgram.hpp"
#include "Engine/Graph/Mesh.hpp"

namespace Engine{
    using namespace Graph;
    using namespace glm;
    using namespace Common;
    class Debug{
    private:
        ShaderProgram* shaderProgram;
        Character c;
        vector<float> vertices;
        GLuint VAO,VBO;
    public:
        Debug(){
            shaderProgram = new ShaderProgram("font");
            c = Font::GetChar('c');
            // Configure VAO/VBO for texture quads
            glGenVertexArrays(1, &VAO);
            glBindVertexArray(VAO);

            glGenBuffers(1, &VBO);
            glBindBuffer(GL_ARRAY_BUFFER, VBO);
            glBufferData(GL_ARRAY_BUFFER, 24 * sizeof(GLfloat), nullptr, GL_DYNAMIC_DRAW);
            glEnableVertexAttribArray(0);
            glVertexAttribPointer(0,4,GL_FLOAT,GL_FALSE,4*sizeof(float),nullptr);

            glBindBuffer(GL_ARRAY_BUFFER, 0);
            glBindVertexArray(0);
        }

        void Draw(vec2 pos,const Text& text,vec3 color){
            glEnable(GL_CULL_FACE);
            glEnable(GL_BLEND);
            glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

            shaderProgram->Bind();
            shaderProgram->SetVec3("color", reinterpret_cast<float *>(&color));
            mat4 P = ortho(0.f,800.f,600.f,0.f);
            shaderProgram->SetMat4("P",reinterpret_cast<float *>(&P));
            glActiveTexture(GL_TEXTURE0);
            glBindVertexArray(VAO);


            for (std::_String_const_iterator<std::_String_val<std::_Simple_types<wchar_t>>>::value_type it : text) {
                c = Font::GetChar(it);
                float x = pos.x+c.bearing.x*1.0f;
                float y = pos.y+Font::PIXEL_SIZE+float((c.size.y-c.bearing.y))*1.0f;
                auto w = float(c.size.x);
                auto h = float(c.size.y);

                //6个顶点
                vertices ={
                    x,y-h,0.0,0.0,
                    x,y,0.0,1.0,
                    x+w,y,1.0,1.0,

                    x,y-h,0.0,0.0,
                    x+w,y,1.0,1.0,
                    x+w,y-h,1.0,0.0,
                };

                pos.x += c.advance>>6;

                glBindTexture(GL_TEXTURE_2D,c.texture);
                glBindBuffer(GL_ARRAY_BUFFER,VBO);
                glBufferSubData(GL_ARRAY_BUFFER,0,vertices.size()*sizeof(float),vertices.data());
                glDrawArrays(GL_TRIANGLES,0,6);
            }
            glBindVertexArray(0);
            glBindTexture(GL_TEXTURE_2D,0);

            glDisable(GL_CULL_FACE);
            glDisable(GL_BLEND);
        };
    };
}