#pragma once

namespace Engine{
    using namespace glm;
    class Camera{
    private:
        vec3 position{0.f,0.f,0.f};
        vec3 front{0.f,0.f,-1.f};
        vec3 up{0.f,1.f,0.f};
        vec3 right{1.f,0.f,0.f};
    public:
        Camera(vec3 pos){
            this->position = pos;
        };
        ~Camera(){
            cout << "Drop Camera" << endl;
        }
        [[nodiscard]] mat4 GetViewMatrix() const{
            return lookAt(position,position+front,up);
        }

        void MoveForward(float factor){
            position += factor*front;
        }

        void MoveRight(float factor){
            position += factor*normalize(cross(front,up));
        }
    };
}