#pragma once

using namespace std;

namespace Engine::Graph{
    class Texture{
    private:
        const unsigned char* buffer = nullptr;
        int len,width,height,comp;
        unsigned int texture;
    public:
        Texture(const unsigned char* buffer,int len){
            this->buffer = buffer;
            this->len = len;
            unsigned char* data = stbi_load_from_memory(buffer,len,&width,&height,&comp,0);
            Utils::Logger::Info("Image Info width={}, height={}, channels={}",width,height,comp);
            glGenTextures(1,&texture);
            //为当前绑定的纹理对象设置环绕、过滤方式
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
            //生成纹理
            glBindTexture(GL_TEXTURE_2D,texture);
            glTexImage2D(GL_TEXTURE_2D,0,GL_RGB,width,height,0,GL_RGB,GL_UNSIGNED_BYTE,data);
            glGenerateMipmap(GL_TEXTURE_2D);
            stbi_image_free(data);
        };
        ~Texture(){
            cout << "Drop Texture" << endl;
        }

        void Bind() const {
            glBindTexture(GL_TEXTURE_2D,texture);
        }

        void Cleanup(){
            glDeleteTextures(1,&texture);
        }
    };
}