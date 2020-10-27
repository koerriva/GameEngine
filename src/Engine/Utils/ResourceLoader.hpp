#pragma once

#define FILE_SPLITER "\\"
#ifdef __APPLE__
    #define FILE_SPLITER "/"
#endif

#include <filesystem>
#include <fstream>

using namespace std;
using namespace std::filesystem;

namespace Engine::Utils{
    struct FileInfo {
        long long size=0;
        string ext;
    };
    class ResourceLoader{
    private:
        const char* root = "data";
        unordered_map<string,FileInfo> info;
        unordered_map<string,vector<unsigned char>> data;
        void file_collector(const directory_entry& entry){
            if(entry.is_directory()){
                Logger::Info("Dir {}",entry.path().string());
                directory_iterator list(entry.path());
                for (auto& dir:list) {
                    file_collector(dir);
                }
            }else{
                ifstream file(entry.path(),ios::ate|ios::binary);
                if(file.is_open()){
                    auto filename = entry.path().string();
                    auto ext = filename.substr(filename.find_last_of('.')+1);

                    streampos size = file.tellg();
                    auto& buffer = data[filename];
                    if(size>0){
                        buffer.resize(size);
                        file.seekg(0,ios::beg);
                        char* p = (char*)(buffer.data());
                        file.read(p,size);
                    }
                    file.close();
                    info[filename].ext = ext;
                    info[filename].size = buffer.size();
                    Logger::Info("File {},{}",filename,buffer.size());
                }
            }
        }
    public:
        ResourceLoader() = default;
        ~ResourceLoader(){
            cout << "Drop ResourceLoader" << endl;
        };

        void Init(){
            directory_iterator list(root);
            for (auto& entry:list){
                file_collector(entry);
            }
        };

        const char* LoadShader(const char* name,int type){
            string file_ext;
            if(type==1){
                file_ext = ".vert";
            }else if(type==3){
                file_ext = ".frag";
            }else if(type==2){
                file_ext = ".geom";
            }
            
            string filepath = string(root).append(FILE_SPLITER).append("shader").append(FILE_SPLITER).append(name).append(file_ext);
            path dir(filepath);
            Logger::Info("Find Shader {}",dir.string());

            if(data.count(filepath)>0){
                auto& buffer = data[filepath];
                if(buffer.size()==info[filepath].size){
                    Logger::Info("Add Terminated Char {}",dir.string());
                    buffer.push_back(0);
                }
                return (const char*)(buffer.data());
            }else{
                Logger::Error("Can't Find Shader {}",dir.string());
                return nullptr;
            }
        }

        const unsigned char* LoadTexture(const char* name,int* len){
            string filepath = string(root).append(FILE_SPLITER).append("textures").append(FILE_SPLITER).append(name);
            path dir(filepath);
            Logger::Info("Find Texture {}",dir.string());

            if(data.count(filepath)>0){
                *len = data[filepath].size();
                return data[filepath].data();
            }else{
                Logger::Error("Can't Find Texture {}",dir.string());
                return nullptr;
            }
        }

        void Cleanup(){
            data.clear();
        }
    };
}