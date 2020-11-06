#pragma once
#include <random>
using namespace std;
namespace Engine::Utils{
    class Random{
    private:
        default_random_engine engine;

    public:
        Random() = default;
        ~Random() = default;

        float random(float min,float max){
            uniform_real_distribution<float> distribution(min,max);
            return distribution(engine);
        }
    };
}