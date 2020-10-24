#pragma once

namespace Engine::Graph{
    class ShaderProgram{
    private:
        const char* vertexSource;
        const char* fragmentSource;
        unsigned int shaderProgram;

        unsigned int CreateShader(GLuint type){
            unsigned int shader = glCreateShader(type);
            if (type==GL_VERTEX_SHADER){
                glShaderSource(shader,1,&vertexSource, nullptr);

            }else if(type==GL_FRAGMENT_SHADER){
                glShaderSource(shader,1,&fragmentSource, nullptr);
            }else{
                Utils::Logger::Error("Unsupported Shader type {}",type);
                exit(-1001);
            }
            glCompileShader(shader);
            int success;
            char info[512];
            glGetShaderiv(shader,GL_COMPILE_STATUS,&success);
            if(!success){
                glGetShaderInfoLog(shader,512, nullptr,info);
                Utils::Logger::Error(info);
                exit(-1002);
            }
            return shader;
        }

        unsigned int CreateProgram(unsigned vertShader,unsigned fragShader){
            unsigned int program = glCreateProgram();
            glAttachShader(program,vertShader);
            glAttachShader(program,fragShader);
            glLinkProgram(program);
            int success;
            char info[512];
            glGetProgramiv(program,GL_LINK_STATUS, &success);
            if(!success){
                glGetProgramInfoLog(program,512, nullptr,info);
                Utils::Logger::Error(info);
                exit(-1002);
            }
            return program;
        }
    public:
        ShaderProgram(const char* vertexSource,const char* fragmentSource){
            this->vertexSource = vertexSource;
            this->fragmentSource = fragmentSource;
        }
        ~ShaderProgram() {};

        void Init(){
            unsigned vertexShader = CreateShader(GL_VERTEX_SHADER);
            unsigned fragmentShader = CreateShader(GL_FRAGMENT_SHADER);
            shaderProgram = CreateProgram(vertexShader,fragmentShader);
            glDeleteShader(shaderProgram);
            glDeleteShader(fragmentShader);
        };
        void Bind(){
            glUseProgram(shaderProgram);
        };
        void Unbind(){
            glUseProgram(0);
        };
        void Cleanup(){
            glDeleteProgram(shaderProgram);
        }
    };
}