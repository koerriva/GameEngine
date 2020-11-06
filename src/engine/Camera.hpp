#pragma once

namespace Engine{
    using namespace glm;
    class Camera{
    private:
        vec3 position{0.f,0.f,0.f};
        vec3 front{0.f,0.f,-1.f};
        vec3 up{0.f,1.f,0.f};
        vec3 right{1.f,0.f,0.f};
        float pitch=0;
        float yaw = -90;
    public:
        explicit Camera(vec3 pos){
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

        void Rotate(float xoffset,float yoffset){
            yaw -= xoffset;
            pitch -= yoffset;
            pitch = glm::clamp(pitch,-89.0f,89.f);
            glm::vec3 _front;
            _front.x = cos(glm::radians(pitch)) * cos(glm::radians(yaw));
            _front.y = sin(glm::radians(pitch));
            _front.z = cos(glm::radians(pitch)) * sin(glm::radians(yaw));
            this->front = glm::normalize(_front);
        }

        vec3 Position(){
            return this->position;
        }

        vec3 Rotation(){
            return vec3{yaw,pitch,0};
        }
    };
}