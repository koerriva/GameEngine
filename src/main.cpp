#include <iostream>
#include <glad.h>
#include <unordered_map>
#include <vector>

#define PI 3.14159265358979323846
#define EARTH_RADIUS 6300.f

#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtx/common.hpp>
#include <glm/gtx/compatibility.hpp>

#define STB_IMAGE_IMPLEMENTATION
#include <stb/stb_image.h>
#define STB_IMAGE_WRITE_IMPLEMENTATION
#include <stb/stb_image_write.h>

#include <ft2build.h>
#include FT_FREETYPE_H

#include "Engine/Common/Text.hpp"

#include "Engine/Common/Random.hpp"
#include "Engine/Utils/Logger.hpp"
#include "Engine/Utils/ResourceLoader.hpp"

#include "Engine/Font.hpp"
#include "Engine/Camera.hpp"
#include "Engine/Window.hpp"
#include "Engine/GameEngine.hpp"
#include "Engine/Debug.hpp"
#include "Game/DummyGame.hpp"

using namespace std;
using namespace Engine;
using namespace Engine::Utils;
using namespace Game;

#include <noise.h>
using namespace noise;

// N卡使用独显运行
extern "C" __declspec(dllexport) unsigned long NvOptimusEnablement = 0x00000001;

// A显卡使用独显运行
//extern "C" __declspec(dllexport) int AmdPowerXpressRequestHighPerformance = 0x00000001;

int main(){
//    auto frameStart = duration_cast<nanoseconds>(steady_clock::now().time_since_epoch()).count();
//    Logger::Info("frameStart {}",frameStart);
//    auto frameEnd = frameStart+1000000000;
//    Logger::Info("frameEnd {}",frameEnd);
//    long long now = frameStart;
//    while (now <  frameEnd){
//        this_thread::sleep_for(milliseconds(100));
//        now = duration_cast<nanoseconds>(steady_clock::now().time_since_epoch()).count();
//        Logger::Info("now {}",now);
//    }
//    vector<int> arr={1,2,3,4};
//    for(auto it=arr.begin();it!=arr.end();it++){
//        cout << *it << endl;
//        if(*it==2){
//            arr.push_back(6);
//        }
//    }


    Logger::Info("我的游戏引擎 0.1");
    DummyGame game;
    GameEngine engine("我的游戏引擎 0.1",800,600, false,&game);
    engine.Run();

//    int w=4000,h=2000,comp=3;
//    auto* data = static_cast<unsigned char *>(malloc(w * h * comp));
//
//    module::Perlin perlin;
//    perlin.SetSeed(1234);
//    perlin.SetOctaveCount(7);
//    perlin.SetLacunarity(2.0);
//    perlin.SetPersistence(0.6);
//    perlin.SetFrequency(1);
//    perlin.SetNoiseQuality(NoiseQuality::QUALITY_BEST);
//
//    model::Sphere sphere;
//    sphere.SetModule(perlin);
//
//    int pos=0;
//    double latFactor = 180.0/h;
//    double lonFactor = 360.0/w;
//    double lat,lon;
//    float min=0,max=0;
//    for (int i = 0; i < h; ++i) {
//        int dh = h/2 - i;
//        for (int j = 0; j <w; ++j) {
//            int dw = j - w/2;
//            lat = dh*latFactor;
//            lon = dw*lonFactor;
//            float s = sphere.GetValue(lat,lon);
//            if(s<min){
//                min=s;
//            }
//            if(s>max){
//                max=s;
//            }
//            double t = (s+1.2293236)/2.5414443;
//
//            ivec3 color;
//            if(s>1.2){
//                //雪山
//                color = vec3(255,255,255);
//            } else if(s<=1.2&&s>1.0){
//                //雪山
//                color = vec3(255,245,245);
//            } else if(s<=1.0&&s>0.7500){
//                //岩石
//                color = vec3(90,77,65);
//            } else if(s<=.7500&&s>.3750){
//                //泥地
//                color = vec3(155,118,83);
//            } else if(s<=.3750&&s>.1250){
//                //草地
//                color = vec3(32,160,0);
//            } else if(s<=.1250&&s>.0625){
//                //沙滩
//                color = vec3(199,158,1);
//            } else if(s<=.0625&&s>.0){
//                color = vec3(0,128,255);
//            } else if(s<=0&&s>-.2500){
//                //浅海
//                color = vec3(0,0,255);
//            } else if(s<=-2.5&&s>-1.0){
//                //深海
//                color = vec3(25,25,112);
//            } else{
//                //深海
//                color = vec3(10,10,52);
//            }
//            data[pos++] = color.r;
//            data[pos++] = color.g;
//            data[pos++] = color.b;
//        }
//    }
//    Logger::Info("{}-{}={}",min,max,min-max);
//    stbi_write_jpg("data/textures/earthmap4k.jpg",w,h,comp,data,100);
}