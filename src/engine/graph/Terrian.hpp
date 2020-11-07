#pragma once

using namespace std;

namespace Engine::Graph {
    struct Patch{
        bool loaded = false;
        float points[12]{};
        float vertex_count = 4;
        unsigned int indices[6] = {0,3,2,0,2,1};
        unsigned int vao{},vbo{},ebo{};
    };
    struct QuadTreeNode{
        unsigned long id=0;
        //N,E,S,W
        QuadTreeNode* neighbors[4]{};
        size_t neighbors_count=0;
        QuadTreeNode* children[4]{};
        size_t children_count=0;
        QuadTreeNode* parent{};
        //深度
        size_t depth{};
        //渲染元
        Patch patch;
        //中心点
        vec3 center{};
        //边长
        float bound{};
    };

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
        patch.loaded = true;
    }

    void DrawPatch(Patch& patch){
        if(patch.loaded){
            glBindVertexArray(patch.vao);
            glDrawElements(GL_TRIANGLES,6,GL_UNSIGNED_INT, nullptr);
            glBindVertexArray(0);
        }
    }

    class Terrain{
    private:
        unsigned long MAX_CHUNK_COUNT = 65535;
        unsigned long next_id = 0;
        QuadTreeNode* chunks = static_cast<QuadTreeNode *>(malloc(sizeof(QuadTreeNode) * MAX_CHUNK_COUNT));

        QuadTreeNode* CreateNewChunk(size_t depth, vec3 center, float bound, QuadTreeNode* parent){
//          Logger::Info("Create QuadTreeNode ({},{},{}),{:.4f}",center.x,center.y,center.z,bound);
            QuadTreeNode node;
            node.depth = depth;
            node.center = center;
            node.bound = bound;
            node.parent = parent;
//          Logger::Info("Set Node[{}] Bound {:.4f}",node.id,bound);
            vec3 top_left = center+vec3(-bound/2,0.f,-bound/2);
            vec3 top_right = center+vec3(bound/2,0.f,-bound/2);
            vec3 bot_left = center+vec3(-bound/2,0.f,bound/2);
            vec3 bot_right = center+vec3(bound/2,0.f,bound/2);
            vector<vec3> buffer;
            buffer.push_back(top_left);
            buffer.push_back(top_right);
            buffer.push_back(bot_right);
            buffer.push_back(bot_left);
            auto *ptr = reinterpret_cast<float *>(buffer.data());
            for (int i = 0; i < 12; ++i) {
                node.patch.points[i] = *(ptr + i);
            }
            UploadPatch(node.patch);

            node.id = next_id++;
            chunks[node.id]=node;
            return chunks+node.id;
        }

        void SplitChunk(QuadTreeNode* parent){
            float childBound = parent->bound/2;
            vec3 NW_OFFSET(-childBound/2,0.f,-childBound/2);
            vec3 NE_OFFSET(childBound/2,0.f,-childBound/2);
            vec3 SW_OFFSET(-childBound/2,0.f,childBound/2);
            vec3 SE_OFFSET(childBound/2,0.f,childBound/2);
            parent->children[0] = CreateNewChunk(parent->depth+1,parent->center+NW_OFFSET,childBound,parent);
            parent->children[1] = CreateNewChunk(parent->depth+1,parent->center+NE_OFFSET,childBound,parent);
            parent->children[2] = CreateNewChunk(parent->depth+1,parent->center+SW_OFFSET,childBound,parent);
            parent->children[3] = CreateNewChunk(parent->depth+1,parent->center+SE_OFFSET,childBound,parent);
            parent->children_count=4;
        }

        void SplitChunk(QuadTreeNode* parent,int depth){
            if(parent->depth<depth){
                if(parent->children_count==0){
                    SplitChunk(parent);
                }else{
                    for (int i = 0; i < parent->children_count; ++i) {
                        SplitChunk(parent->children[i],depth);
                    }
                }
            }
        }

        QuadTreeNode* target_chunk;
        int target_depth=1;
    public:
        explicit Terrain() {
            cout << "QuadTreeNode Size:" << sizeof(QuadTreeNode) << endl;
            vec3 origin(0.f,0.f,0.f);
            float bound = 10.f;
            vec3 up_offset(0,0,-bound),down_offset(0,0,bound),left_offset(-bound,0,0),right_offset(bound,0,0);

            int init_depth=1;
            auto root = CreateNewChunk(init_depth,origin,bound, nullptr);
            root->neighbors[0] = CreateNewChunk(init_depth,origin+up_offset,bound,nullptr);
            root->neighbors[1] = CreateNewChunk(init_depth,origin+down_offset,bound,nullptr);
            root->neighbors[2] = CreateNewChunk(init_depth,origin+left_offset,bound,nullptr);
            root->neighbors[3] = CreateNewChunk(init_depth,origin+right_offset,bound,nullptr);
            root->neighbors_count = 4;

            target_chunk = root;
            target_depth = init_depth;

            Logger::Info("Root Node[{}] Depth[{}] {:.4f}",root->id,root->depth,root->bound);
        }

        virtual ~Terrain() {
            cout << "Drop Terrain" << endl;
        }

        void Init(){
        }

        void Update(int depth){
            target_depth = depth;
            SplitChunk(target_chunk,target_depth);
//            const size_t size = next_id;
//            for (size_t i = 0; i < size; ++i) {
//                auto& chunk = chunks[i];
//                if(chunk.depth<target_depth&&chunk.children_count==0){
//                    SplitChunk(&chunk);
//                }
//            }
        }

        void Draw(){
            const size_t size = next_id;
            for (size_t i = 0; i < size; ++i) {
                auto& chunk = chunks[i];
                if(chunk.depth<=target_depth){
                    DrawPatch(chunk.patch);
                }
            }
        }

        void Cleanup(){
            delete chunks;
        }

        [[nodiscard]] size_t GetChunkSize() const {
            return next_id;
        }
    };
}