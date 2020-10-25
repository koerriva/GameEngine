#pragma once

#include <string>
#include <GLFW/glfw3.h>

using namespace std;

namespace Engine{
    class Window
    {
    private:
        int width = 640;
        int height = 480;
        bool vsync = true;
        string title;
        bool closed = false;

        GLFWwindow* glfwWindow{};
    public:
        Window(string title,int width,int height,bool vsync);
        ~Window();

        void Init();
        void Update();

        [[nodiscard]] bool Closed() const {
            return closed;
        }

        void Close(){
            glfwSetWindowShouldClose(glfwWindow,true);
        }

        void Cleanup();

        //Input
        bool GetKeyPressed(int key);
    };

    Window::Window(string title,int width,int height,bool vsync)
    {
        this->height=height;
        this->width=width;
        this->vsync=vsync;
        this->title = std::move(title);
    }

    Window::~Window() = default;

    void Window::Init(){
        if(glfwInit()){
            glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
            glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
            glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
            glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);

            glfwWindow = glfwCreateWindow(800, 600, title.c_str(), nullptr, nullptr);
            if(!glfwWindow){
                glfwTerminate();
                std::cerr << "Create GLFW Window Fail!"<<std::endl;
                exit(-1);
            }

            //set window to center
            GLFWmonitor* monitor = glfwGetPrimaryMonitor();
            const GLFWvidmode* mode = glfwGetVideoMode(monitor);
            if(mode){
                int mW=mode->width,mH=mode->height;
                Utils::Logger::Info("Monitor Size {},{}",mW,mH);
                glfwSetWindowPos(glfwWindow,(mW-width)/2,(mH-height)/2);
            }

            glfwMakeContextCurrent(glfwWindow);

            if(!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress)){
                std::cerr << "Load OpenGL Fail!" << std::endl;
                exit(-1);
            }

            Utils::Logger::Info("OpenGL Version {}.{}",GLVersion.major,GLVersion.minor);
            if(vsync){
                glfwSwapInterval(1);
            }
        }
        
    }

    void Window::Update(){
        closed = glfwWindowShouldClose(glfwWindow)==1;
        glfwSwapBuffers(glfwWindow);
        glfwPollEvents();
    }

    void Window::Cleanup(){
        glfwTerminate();
    }

    bool Window::GetKeyPressed(int key){
        return glfwGetKey(glfwWindow,key)==GLFW_PRESS;
    }

    enum KeyCode {
        ESC = GLFW_KEY_ESCAPE,ENTER,TAB,
        RIGHT=GLFW_KEY_RIGHT,LEFT,DOWN,UP,
        F1 = GLFW_KEY_F1,
        F2,F3,F4,F5,F6,F7,F8,F9,F10,F11,F12,
        Num0 = GLFW_KEY_0,Num1,Num2,Num3,Num4,Num5,Num6,Num7,Num8,Num9,
        A=GLFW_KEY_A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z
    };
}