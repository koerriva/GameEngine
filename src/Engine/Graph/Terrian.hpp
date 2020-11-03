#pragma once

using namespace std;

namespace Engine::Graph {
    struct Patch{
        float points[12]{};
        float vertex_count = 4;
        unsigned int indices[6] = {0,3,2,0,2,1};
        unsigned int vao,vbo,ebo;
    };
    struct QuadTreeNode{
        //N,E,S,W
        QuadTreeNode* neighbors[4]{};
        size_t neighbors_count=0;
        QuadTreeNode* children[4]{};
        size_t children_count=0;
        QuadTreeNode* parent{};
        //深度
        size_t depth{};
        //
        Patch patch;
    };

    void InitQuadTreeNode(QuadTreeNode& treeNode,size_t depth,vec3 topleft,vec3 topright,vec3 botleft,vec3 botright){
        treeNode.depth=depth;
        vector<vec3> buffer;
        buffer.push_back(topleft);
        buffer.push_back(topright);
        buffer.push_back(botright);
        buffer.push_back(botleft);
        auto *ptr = reinterpret_cast<float *>(buffer.data());
        for (int i = 0; i < 12; ++i) {
            treeNode.patch.points[i] = *(ptr+i);
        }
    }

    void UploadPatch(Patch& patch){
        unsigned int vao,vbo,ebo;
        glGenVertexArrays(1,&vao);
        glBindVertexArray(vao);

        glGenBuffers(1,&vbo);
        glBindBuffer(GL_ARRAY_BUFFER,vbo);
        glBufferData(GL_ARRAY_BUFFER,sizeof(patch.points),patch.points,GL_STATIC_DRAW);
        glVertexAttribPointer(0,3,GL_FLOAT,GL_FALSE,3*sizeof(float), nullptr);
        glEnableVertexAttribArray(0);

        glGenBuffers(1,&ebo);
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER,ebo);
        glBufferData(GL_ELEMENT_ARRAY_BUFFER,sizeof(patch.indices),patch.indices,GL_STATIC_DRAW);

        glBindVertexArray(0);

        patch.vao = vao;
        patch.vbo = vbo;
        patch.ebo = ebo;
    }

    void UploadNode(QuadTreeNode& node){
        UploadPatch(node.patch);
        for (int i = 0; i < node.neighbors_count; ++i) {
            UploadPatch(node.neighbors[i]->patch);
        }
    }

    void DrawPatch(Patch& patch){
        glBindVertexArray(patch.vao);
        glDrawElements(GL_TRIANGLES,6,GL_UNSIGNED_INT, nullptr);
        glBindVertexArray(0);
    }

    void DrawNode(QuadTreeNode& node){
        DrawPatch(node.patch);
        for (int i = 0; i < node.neighbors_count; ++i) {
            DrawPatch(node.neighbors[i]->patch);
        }
    }

    class Terrain{
    private:
        QuadTreeNode up;
        QuadTreeNode down;
        QuadTreeNode left;
        QuadTreeNode right;
        QuadTreeNode center;
    public:
        explicit Terrain() {
            vec3 origin(0.f,0.f,0.f);
            float bound = 10.f;
            vec3 center_top_left = vec3(-bound/2,0.f,bound/2);
            vec3 center_top_right = vec3(bound/2,0.f,bound/2);
            vec3 center_bot_left = vec3(-bound/2,0.f,-bound/2);
            vec3 center_bot_right = vec3(bound/2,0.f,-bound/2);

            vec3 up_offset(0,0,bound),down_offset(0,0,-bound),left_offset(-bound,0,0),right_offset(bound,0,0);

            InitQuadTreeNode(up,1
                             ,center_top_left+up_offset,center_top_right+up_offset
                             ,center_bot_left+up_offset,center_bot_right+up_offset);
            InitQuadTreeNode(down,1
                    ,center_top_left+down_offset,center_top_right+down_offset
                    ,center_bot_left+down_offset,center_bot_right+down_offset);
            InitQuadTreeNode(left,1
                             ,center_top_left+left_offset,center_top_right+left_offset
                             ,center_bot_left+left_offset,center_bot_right+left_offset);
            InitQuadTreeNode(right,1
                             ,center_top_left+right_offset,center_top_right+right_offset
                             ,center_bot_left+right_offset,center_bot_right+right_offset);

            InitQuadTreeNode(center,1,center_top_left,center_top_right ,center_bot_left ,center_bot_right);

            center.neighbors[0] = &up;
            center.neighbors[1] = &right;
            center.neighbors[2] = &down;
            center.neighbors[3] = &left;
            center.neighbors_count=4;

            UploadNode(center);
        }

        virtual ~Terrain() {
            cout << "Drop Terrain" << endl;
        }

        void Draw(){
            DrawNode(center);
        }

        void Cleanup(){
        }
    };
}