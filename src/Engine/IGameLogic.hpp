#pragma once

#include "Window.h"

class IGameLogic
{
private:
public:
    virtual void init() = 0;
    virtual void input(Window* window) = 0;
    virtual void update(float interval) = 0;
    virtual void render(Window* window) = 0;
};
