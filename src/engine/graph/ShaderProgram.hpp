#pragma once

namespace Engine::Graph{
    enum ShaderType{
        VERTEX_SHADER=1,GEOMETRY_SHADER,FRAGMENT_SHADER
    };

    class ShaderProgram{
    private:
        const char* vertexSource;
        const char* fragmentSource;
        unsigned int shaderProgram=0;

        unordered_map<string,int> uniforms;

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

        static unsigned int CreateProgram(unsigned vertShader,unsigned fragShader){
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

        void Upload(){
            unsigned vertexShader = CreateShader(GL_VERTEX_SHADER);
            unsigned fragmentShader = CreateShader(GL_FRAGMENT_SHADER);
            shaderProgram = CreateProgram(vertexShader,fragmentShader);
            glDeleteShader(vertexShader);
            glDeleteShader(fragmentShader);
        };
    public:
        ShaderProgram(const char* name){
            this->vertexSource = ResourceLoader::LoadShader(name,VERTEX_SHADER);
            this->fragmentSource = ResourceLoader::LoadShader(name,FRAGMENT_SHADER);
            Upload();
        }
        ~ShaderProgram() {
            cout << "Drop ShaderProgram" << endl;
        };

        void Bind() const{
            glUseProgram(shaderProgram);
        };
        static void Unbind(){
            glUseProgram(0);
        };
        void Cleanup() const{
            cout << "Clean Program " << shaderProgram << endl;
            glDeleteProgram(shaderProgram);
        }

        void SetFloat(string name,float value){
            int location = 0;
            if(uniforms.count(name)==0){
                cout << "Find Uniform : " << name << endl;
                location = glGetUniformLocation(shaderProgram,name.c_str());
                cout << "Uniform[" << name << "] Location=" << location << endl;
                uniforms[name]=location;
            }else{
                location = uniforms[name];
            }
            glUniform1f(location,value);
        }

        void SetMat4(string name,float* value){
            int location = 0;
            if(uniforms.count(name)==0){
                cout << "Find Uniform : " << name << endl;
                location = glGetUniformLocation(shaderProgram,name.c_str());
                cout << "Uniform[" << name << "] Location=" << location << endl;
                uniforms[name]=location;
            }else{
                location = uniforms[name];
            }
            glUniformMatrix4fv(location,1,GL_FALSE,value);
        }

        void SetVec3(string name,float* value){
            int location = 0;
            if(uniforms.count(name)==0){
                cout << "Find Uniform : " << name << endl;
                location = glGetUniformLocation(shaderProgram,name.c_str());
                cout << "Uniform[" << name << "] Location=" << location << endl;
                uniforms[name]=location;
            }else{
                location = uniforms[name];
            }
            glUniform3fv(location,1,value);
        }
    };
}