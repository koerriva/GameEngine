#pragma once

#include <spdlog/spdlog.h>

namespace Engine::Utils{
    using namespace spdlog;
    class Logger
    {
    private:
    public:
        Logger()= default;;
        ~Logger()= default;;

        template<typename T>
        static void Info(const T& msg){
            info(msg);
        };

        template<typename T,typename... Args>
        static void Info(const T& fmt,const Args&... args){
            info(fmt,args...);
        }

        template<typename T>
        static void Error(const T& msg){
            error(msg);
        };

        template<typename T,typename... Args>
        static void Error(const T& fmt,const Args&... args){
            error(fmt,args...);
        }
    };
    
}