#pragma once
#include <vector>
#include <glad.h>

using namespace std;

namespace Engine::Graph {
    class Mesh
    {
    private:
        vector<float> vertices;
        vector<unsigned int> indices;
        vector<float> normals;
        vector<float> texCoords;

        unsigned int vao=0;
        vector<unsigned int> vbos;
        unsigned int ebo=0;

    public:
        Mesh(vector<float>& vertices,vector<unsigned>& indices,vector<float> &normals,vector<float> &texCoords);
        ~Mesh();

        void Draw() const;
        void Cleanup() const;

        static Mesh Sphere(float r,int sectors,int stacks);
    };

    Mesh::Mesh(vector<float> &vertices,vector<unsigned>& indices,vector<float> &normals,vector<float> &texCoords) {
        this->vertices = vertices;
        this->indices = indices;
        this->normals = normals;
        this->texCoords = texCoords;

        glGenVertexArrays(1,&vao);
        glBindVertexArray(vao);

        cout << "vertices :" << vertices.size() << endl;
        unsigned int vbo[3];

        //vertices
        glGenBuffers(3,vbo);
        glBindBuffer(GL_ARRAY_BUFFER,vbo[0]);
        glBufferData(GL_ARRAY_BUFFER,vertices.size()*sizeof(float),vertices.data(),GL_STATIC_DRAW);
        glVertexAttribPointer(0,3,GL_FLOAT,GL_FALSE,3*sizeof(float),nullptr);
        glEnableVertexAttribArray(0);

        //normals
        glBindBuffer(GL_ARRAY_BUFFER,vbo[1]);
        glBufferData(GL_ARRAY_BUFFER,normals.size()*sizeof(float),normals.data(),GL_STATIC_DRAW);
        glVertexAttribPointer(1,3,GL_FLOAT,GL_FALSE,3*sizeof(float),nullptr);
        glEnableVertexAttribArray(1);

        //texcoords
        glBindBuffer(GL_ARRAY_BUFFER,vbo[2]);
        glBufferData(GL_ARRAY_BUFFER,texCoords.size()*sizeof(float),texCoords.data(),GL_STATIC_DRAW);
        glVertexAttribPointer(2,2,GL_FLOAT,GL_FALSE,2*sizeof(float),nullptr);
        glEnableVertexAttribArray(2);

        glGenBuffers(1,&ebo);
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER,ebo);
        glBufferData(GL_ELEMENT_ARRAY_BUFFER,indices.size()*sizeof(unsigned),indices.data(),GL_STATIC_DRAW);

        glBindVertexArray(0);

        for (unsigned int & i : vbo) {
            vbos.push_back(i);
        }
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
        vector<float> vertices;
        vector<unsigned int> indices;
        vector<float> normals;
        vector<float> texCoords;

        float sectorStep = float(2.*PI) / float(sectors);//圆周等分
        float stackStep = PI / stacks; //半圆等分
        for (int i = 0; i <= stacks; ++i) {
            float stackAngle = PI/2 - float(i)*stackStep; //垂直角
            float y = r*sinf(stackAngle);
            float xz = r*cosf(stackAngle);

            for (int j = 0; j <= sectors; ++j) {
                float sectorAngle = float(j)*sectorStep; //水平角
                float x = xz*cosf(sectorAngle);
                float z = xz*sinf(sectorAngle);

                //顶点坐标
                vertices.push_back(x);
                vertices.push_back(y);
                vertices.push_back(z);

                //贴图坐标
                float s = float(j)/float(sectors);
                float t = float(i)/float(stacks);
                texCoords.push_back(s);
                texCoords.push_back(t);

                //法线
                float len = 1.0f/float(r);
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
        return Mesh(vertices,indices,normals,texCoords);
    }
}
