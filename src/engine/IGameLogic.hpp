#pragma once

namespace Engine {
    class IGameLogic
    {
    private:
    public:
        virtual void Init() = 0;
        virtual void Input(Window* window) = 0;
        virtual void Update(float elapsedTime) = 0;
        virtual void Render(Window* window,float elapsedTime) = 0;
        virtual void Cleanup() = 0;
    };
}
