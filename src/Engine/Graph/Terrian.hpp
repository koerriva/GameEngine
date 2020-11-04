#pragma once

using namespace std;

namespace Engine::Graph {
    struct Patch{
        float points[12]{};
        float vertex_count = 4;
        unsigned int indices[6] = {0,3,2,0,2,1};
        unsigned int vao{},vbo{},ebo{};
    };
    struct QuadTreeNode{
        unsigned long id=0;
        //N,E,S,W
        shared_ptr<QuadTreeNode> neighbors[4];
        size_t neighbors_count=0;
        shared_ptr<QuadTreeNode> children[4];
        size_t children_count=0;
        shared_ptr<QuadTreeNode> parent;
        //深度
        size_t depth{};
        //渲染元
        Patch patch;
        //中心点
        vec3 center{};
        //边长
        float bound{};
    };

    QuadTreeNode InitQuadTreeNode(size_t depth,vec3 center,float bound,const shared_ptr<QuadTreeNode>& parent){
//        Logger::Info("Init QuadTreeNode ({},{},{}),{:.4f}",center.x,center.y,center.z,bound);
        QuadTreeNode treeNode;
        vec3 top_left = center+vec3(-bound/2,0.f,-bound/2);
        vec3 top_right = center+vec3(bound/2,0.f,-bound/2);
        vec3 bot_left = center+vec3(-bound/2,0.f,bound/2);
        vec3 bot_right = center+vec3(bound/2,0.f,bound/2);
        treeNode.depth = depth;
        treeNode.center = center;
        treeNode.bound = bound;
//        Logger::Info("Set Node[{}] Bound {:.4f}",treeNode.id,bound);
        vector<vec3> buffer;
        buffer.push_back(top_left);
        buffer.push_back(top_right);
        buffer.push_back(bot_right);
        buffer.push_back(bot_left);
        auto *ptr = reinterpret_cast<float *>(buffer.data());
        for (int i = 0; i < 12; ++i) {
            treeNode.patch.points[i] = *(ptr+i);
        }
        if(parent!= nullptr){
            treeNode.parent = parent;
        }
        return treeNode;
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

    void DrawPatch(Patch& patch){
        glBindVertexArray(patch.vao);
        glDrawElements(GL_TRIANGLES,6,GL_UNSIGNED_INT, nullptr);
        glBindVertexArray(0);
    }

    class Terrain{
    private:
        vector<QuadTreeNode> chunks;
//        unordered_map<unsigned long,QuadTreeNode> chunks;

        shared_ptr<QuadTreeNode> CreateNewChunk(size_t depth,vec3 center,float bound,const shared_ptr<QuadTreeNode>& parent){
            auto node = InitQuadTreeNode(depth,center,bound,parent);
            node.id = chunks.size();
            chunks.push_back(node);
             return make_shared<QuadTreeNode>(chunks[node.id]);
        }

        void SplitChunk(const shared_ptr<QuadTreeNode>& parent,int maxDepth){
            if(parent->depth>maxDepth)return;
            float childBound = parent->bound/2;
//            Logger::Error("Node[{}] Depth[{}] Bound[{:.8f}, {:.8f}]",parent->id,parent->depth,parent->bound,childBound);
            vec3 NW_OFFSET(-childBound/2,0.f,-childBound/2);
            vec3 NE_OFFSET(childBound/2,0.f,-childBound/2);
            vec3 SW_OFFSET(-childBound/2,0.f,childBound/2);
            vec3 SE_OFFSET(childBound/2,0.f,childBound/2);
            parent->children[0] = CreateNewChunk(parent->depth+1,parent->center+NW_OFFSET,childBound,parent);
            parent->children[1] = CreateNewChunk(parent->depth+1,parent->center+NE_OFFSET,childBound,parent);
            parent->children[2] = CreateNewChunk(parent->depth+1,parent->center+SW_OFFSET,childBound,parent);
            parent->children[3] = CreateNewChunk(parent->depth+1,parent->center+SE_OFFSET,childBound,parent);
            parent->children_count=4;
            for(const auto& node:parent->children){
                SplitChunk(node,maxDepth);
            }
        }

        void SplitChunk(const shared_ptr<QuadTreeNode>& parent){
            float childBound = parent->bound/2;
//            Logger::Error("Node[{}] Depth[{}] Bound[{:.8f}, {:.8f}]",parent->id,parent->depth,parent->bound,childBound);
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

        shared_ptr<QuadTreeNode> target_chunk;
        int target_depth=1;
    public:
        explicit Terrain() {
            vec3 origin(0.f,0.f,0.f);
            float bound = 10.f;
            vec3 up_offset(0,0,-bound),down_offset(0,0,bound),left_offset(-bound,0,0),right_offset(bound,0,0);

            auto root = CreateNewChunk(1,origin,bound, nullptr);
            root->neighbors[0] = CreateNewChunk(1,origin+up_offset,bound,nullptr);
            root->neighbors[1] = CreateNewChunk(1,origin+down_offset,bound,nullptr);
            root->neighbors[2] = CreateNewChunk(1,origin+left_offset,bound,nullptr);
            root->neighbors[3] = CreateNewChunk(1,origin+right_offset,bound,nullptr);
            root->neighbors_count = 4;

            Logger::Info("Root Node[{}] Depth[{}] {:.4f}",root->id,root->depth,root->bound);
            SplitChunk(root,8);

            for(auto & chunk : chunks){
                UploadPatch(chunk.patch);
            }
        }

        virtual ~Terrain() {
            cout << "Drop Terrain" << endl;
        }

        void Init(){

        }

        void Update(int depth){
            if(depth>target_depth){
                for(auto& chunk:chunks){
                    if(chunk.depth==target_depth){
//                        SplitChunk(chunk)
                    }
                }
            }
        }

        void Draw(){
            for(auto& chunk:chunks){
                if(chunk.depth<=target_depth){
                    DrawPatch(chunk.patch);
                }
            }
        }

        void Cleanup(){
        }
    };
}