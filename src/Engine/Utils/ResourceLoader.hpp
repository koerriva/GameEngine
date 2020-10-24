#pragma once

#include <filesystem>
#include <fstream>

using namespace std;
using namespace std::filesystem;

namespace Engine::Utils{
    class ResourceLoader{
    private:
        const char* root = "data";
        vector<path> files;
        unordered_map<string,vector<char>> data;
        void file_collector(const directory_entry& entry){
            if(entry.is_directory()){
                Logger::Info("Dir {}",entry.path().string());
                directory_iterator list(entry.path());
                for (auto& dir:list) {
                    file_collector(dir);
                }
            }else{
                files.push_back(entry.path());
                ifstream file(entry.path(),ios::ate);
                if(file.is_open()){
                    streampos size = file.tellg();
                    cout << "File " << entry.path().filename() << ", Size " << size << endl;
                    auto& buffer = data[entry.path().string()];
                    if(size>0){
                        buffer.resize(size);
                        file.seekg(0,ios::beg);
                        file.read(buffer.data(),size);
                    }else{
                        file.seekg(0,ios::beg);
                        cout <<"read text file" << endl;
                        int count = 0;
                        char c;
                        while(file.get(c)){
                            cout << "char " << c << endl;
                        }
                    }
                    file.close();
//                    Logger::Info("File {},{}",entry.path().string(),buffer.size());
                }
            }
        }
    public:
        ResourceLoader() = default;
        ~ResourceLoader() = default;

        void Init(){
            directory_iterator list(root);
            for (auto& entry:list){
                file_collector(entry);
            }
        };

        const char* LoadVertexShader(const char* name){
            string filepath = string(root).append("\\").append("shader").append("\\").append(name).append(".vert");
            path dir(filepath);
            Logger::Info("Find Shader {}",dir.string());

            if(data.count(filepath)>0){
                return data[filepath].data();
            }else{
                Logger::Error("Can't Find Shader {}",dir.string());
                return nullptr;
            }
        }

        void Cleanup(){
            data.clear();
        }
    };
}