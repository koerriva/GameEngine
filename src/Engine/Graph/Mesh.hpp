#pragma once
#include <vector>
#include <glad.h>

using namespace std;

namespace Engine::Graph {
    class Mesh
    {
    private:
        const vector<float>* vertices;
        const vector<float>* texCoords;
        const vector<int>* indices;
        unsigned int vao;
        vector<unsigned int> vbos;
        unsigned int ebo;

    public:
        Mesh(const vector<float>* vertices,const vector<float>* texCoords,const vector<int>* indices);
        ~Mesh();

        void Draw();
    };

    Mesh::Mesh(const vector<float>* vertices,const vector<float>* texCoords,const vector<int>* indices)
    {
        this->vertices = vertices;
        this->texCoords = texCoords;
        this->indices = indices;

        glGenVertexArrays(1,&vao);
        glBindVertexArray(vao);

        cout << "vertices :" << vertices->size() << endl;
        unsigned int vbo;
        glGenBuffers(1,&vbo);
        glBindBuffer(GL_ARRAY_BUFFER,vbo);
        glBufferData(GL_ARRAY_BUFFER,vertices->size()*sizeof(float),this->vertices->data(),GL_STATIC_DRAW);
        glVertexAttribPointer(0,3,GL_FLOAT,GL_FALSE,3*sizeof(float),nullptr);
        glEnableVertexAttribArray(0);
        glBindVertexArray(0);
    }

    Mesh::~Mesh(){}

    void Mesh::Draw() {
        glBindVertexArray(vao);
        glDrawArrays(GL_TRIANGLES,0,3);
        glBindVertexArray(0);
    }
}
