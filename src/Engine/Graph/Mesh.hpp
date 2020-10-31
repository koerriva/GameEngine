#pragma once
#include <vector>
#include <glad.h>
#include <noise.h>

using namespace std;
using namespace noise;

namespace Engine::Graph {
    enum TerrainFaceType{
        SNOW=0,ROCK,DIRT,GRASS,SAND,SHORE,SHALLOW_WATER,DEEP_WATER
    };
    tuple<float,vec3> TerrainFaceColor[]={
            make_tuple(1.0000f,vec3(1.0f,1.0f,1.0f)),//snow
            make_tuple(0.7500f,vec3(128.f/255.99f,128.f/255.99f,128.f/255.99f)),//rock
            make_tuple(0.3750f,vec3(224.f/255.99f,224.f/255.99f,0.f)),//dirt
            make_tuple(0.1250f,vec3(32.f/255.99f,160.f/255.99f,0.f)),//grass
            make_tuple(0.0625f,vec3(240.f/255.99f,240.f/255.99f,64.f)),//sand
            make_tuple(0.0000f,vec3(0.f,128.f/255.99f,1.f)),//shore
            make_tuple(-0.2500f,vec3(0.f,0.f,1.f)),//shallow water
            make_tuple(-1.000f,vec3(0.f,0.f,128.f/255.f)),//deep water
    };
    class Mesh
    {
    private:
        vector<float> vertices;//顶点顺序，逆时针为前，顺时针为后
        vector<unsigned int> indices;
        vector<float> normals;
        vector<float> texCoords;
        vector<float> colors;

        unsigned int vao=0;
        vector<unsigned int> vbos;
        unsigned int ebo=0;

    public:
        Mesh(vector<float>& vertices,vector<unsigned>& indices,vector<float> &normals,vector<float> &texCoords,vector<float>& colors);
        ~Mesh();

        void Draw() const;
        void Cleanup() const;

        static Mesh Sphere(float r,int sectors,int stacks);
    };

    Mesh::Mesh(vector<float> &vertices,vector<unsigned>& indices,vector<float> &normals,vector<float> &texCoords,vector<float>& colors) {
        this->vertices = vertices;
        this->indices = indices;
        this->normals = normals;
        this->texCoords = texCoords;

        glGenVertexArrays(1,&vao);
        glBindVertexArray(vao);

        cout << "vertices :" << vertices.size() << endl;
        unsigned int vbo;

        //vertices
        glGenBuffers(1,&vbo);
        glBindBuffer(GL_ARRAY_BUFFER,vbo);
        glBufferData(GL_ARRAY_BUFFER,vertices.size()*sizeof(float),vertices.data(),GL_STATIC_DRAW);
        glEnableVertexAttribArray(0);
        glVertexAttribPointer(0,3,GL_FLOAT,GL_FALSE,3*sizeof(float),nullptr);
        vbos.push_back(vbo);

        //normals
        glGenBuffers(1,&vbo);
        glBindBuffer(GL_ARRAY_BUFFER,vbo);
        glBufferData(GL_ARRAY_BUFFER,normals.size()*sizeof(float),normals.data(),GL_STATIC_DRAW);
        glEnableVertexAttribArray(1);
        glVertexAttribPointer(1,3,GL_FLOAT,GL_FALSE,3*sizeof(float),nullptr);
        vbos.push_back(vbo);

        //texcoords
        glGenBuffers(1,&vbo);
        glBindBuffer(GL_ARRAY_BUFFER,vbo);
        glBufferData(GL_ARRAY_BUFFER,texCoords.size()*sizeof(float),texCoords.data(),GL_STATIC_DRAW);
        glEnableVertexAttribArray(2);
        glVertexAttribPointer(2,2,GL_FLOAT,GL_FALSE,2*sizeof(float),nullptr);
        vbos.push_back(vbo);

        if(!colors.empty()){
            //colors
            glGenBuffers(1,&vbo);
            glBindBuffer(GL_ARRAY_BUFFER,vbo);
            glBufferData(GL_ARRAY_BUFFER,colors.size()*sizeof(float),colors.data(),GL_STATIC_DRAW);
            glEnableVertexAttribArray(3);
            glVertexAttribPointer(3,3,GL_FLOAT,GL_FALSE,3*sizeof(float),nullptr);
            vbos.push_back(vbo);
        }

        glGenBuffers(1,&ebo);
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER,ebo);
        glBufferData(GL_ELEMENT_ARRAY_BUFFER,indices.size()*sizeof(unsigned),indices.data(),GL_STATIC_DRAW);

        glBindVertexArray(0);
    }

    Mesh::~Mesh(){
        cout << "Drop Mesh" << endl;
    }

    void Mesh::Draw() const {
        glBindVertexArray(vao);
//        glDrawArrays(GL_TRIANGLES,0,3);
        glDrawElements(GL_TRIANGLES,indices.size(),GL_UNSIGNED_INT,nullptr);
        glBindVertexArray(0);
    }

    void Mesh::Cleanup() const {
        cout << "Clean Mesh " << vao << endl;
        glDeleteVertexArrays(1,&vao);
        glDeleteBuffers(vbos.size(),vbos.data());
    }

    Mesh Mesh::Sphere(float r,int sectors,int stacks) {
        Utils::Random rand;
        module::Perlin perlin_noise;
        perlin_noise.SetFrequency(2);
        perlin_noise.SetSeed(1234);
        perlin_noise.SetPersistence(0.0);
        perlin_noise.SetLacunarity(3.5);
        perlin_noise.SetOctaveCount(2);
        perlin_noise.SetNoiseQuality(NoiseQuality::QUALITY_BEST);

        vector<float> vertices;
        vector<float> colors;
        vector<unsigned int> indices;
        vector<float> normals;
        vector<float> texCoords;

        float sectorStep = float(2.*PI) / float(sectors);//圆周等分
        auto stackStep = float(PI / stacks); //半圆等分
        float len = 1.0f/float(r);
        for (int i = 0; i <= stacks; ++i) {
            auto stackAngle = float(PI/2 - float(i)*stackStep); //垂直角
            float y = r*sinf(stackAngle);
            float xz = r*cosf(stackAngle);

            for (int j = 0; j <= sectors; ++j) {
                float sectorAngle = float(j)*sectorStep; //水平角
                float x = xz*cosf(sectorAngle);
                float z = xz*sinf(sectorAngle);

                //顶点坐标
                float offset = perlin_noise.GetValue(x,y,z);
                vertices.push_back(x+x*len*offset*100);
                vertices.push_back(y+y*len*offset*100);
                vertices.push_back(z+z*len*offset*100);

                vec3 color;
                for (int k = 0; k < 8; ++k) {
                    if(k<7){
                        auto [maxH,maxC] = TerrainFaceColor[k];
                        auto [minH,minC] = TerrainFaceColor[k+1];
                        if(offset>minH&&offset<=maxH){
                            color = lerp(minC,maxC,offset);
                            break;
                        }
                    }else{
                        auto [maxH,maxC] = TerrainFaceColor[k];
                        color = maxC;
                    }
                }
                colors.push_back(color.r);
                colors.push_back(color.g);
                colors.push_back(color.b);

                //贴图坐标
                float s = float(j)/float(sectors);
                float t = float(i)/float(stacks);
                texCoords.push_back(s);
                texCoords.push_back(t);

                //法线
                normals.push_back(x*len);
                normals.push_back(y*len);
                normals.push_back(z*len);

            }
        }

        for (int v = 0; v < stacks; ++v) {
            int v0 = v*(sectors+1);//起点
            int v1 = v0+sectors+1;//终点
            for (int h = 0; h < sectors; ++h,++v0,++v1) {
                if(v!=0){
                    indices.push_back(v0);
                    indices.push_back(v1);
                    indices.push_back(v0+1);
                }
                if(v!=(stacks-1)){
                    indices.push_back(v0+1);
                    indices.push_back(v1);
                    indices.push_back(v1+1);
                }
            }
        }
        return Mesh(vertices,indices,normals,texCoords,colors);
    }
}
