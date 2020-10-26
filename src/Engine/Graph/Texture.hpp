#pragma once

using namespace std;

namespace Engine::Graph{
    class Texture{
    private:
    public:
        Texture() = default;
        ~Texture(){
            cout << "Drop Texture" << endl;
        }
    };
}