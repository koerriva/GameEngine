#pragma once

namespace Engine{
    using namespace Utils;
    using namespace glm;

    const char* fontpath = "data/font/NotoSansSC-Regular.otf";
    FT_Library ft;
    FT_Face fc;
    struct Character{
        GLuint texture;
        ivec2 size;
        ivec2 bearing;
        GLuint advance;
    };
    std::unordered_map<GLchar,Character> chars;


    class Font{
    private:
    public:
        static void Init(){
            if(FT_Init_FreeType(&ft)){
                Logger::Error("Can't Init FreeType Library");
            }
            if(FT_New_Face(ft,fontpath,0,&fc)){
                Logger::Error("Can't Load Font [{}]",fontpath);
            }
            FT_Set_Pixel_Sizes(fc,0,24);

            glPixelStorei(GL_UNPACK_ALIGNMENT,1);//禁用字节对齐
            for (int c = 0; c < 128; ++c) {
//                Logger::Info("Load Char {}",c);
                if(FT_Load_Char(fc,c,FT_LOAD_RENDER)){
                    Logger::Error("Can't Load char {}",c);
                }
//                Logger::Info("Create Font Texture...");
                GLuint texture;
                glGenTextures(1,&texture);
                glBindTexture(GL_TEXTURE_2D,texture);
//                Logger::Info("Upload Font Texture Data...");
                glTexImage2D(GL_TEXTURE_2D,0,GL_RED
                             ,fc->glyph->bitmap.width,fc->glyph->bitmap.rows
                             ,0,GL_RED,GL_UNSIGNED_BYTE
                             ,fc->glyph->bitmap.buffer);
                // 设置纹理选项
//                Logger::Info("Setting Font Texture...");
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
                glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
                Character character{texture
                                    ,ivec2{fc->glyph->bitmap.width,fc->glyph->bitmap.rows}
                                    ,ivec2{fc->glyph->bitmap_left,fc->glyph->bitmap_top}
                                    ,static_cast<GLuint>(fc->glyph->advance.x)};
                chars[c] = character;
            }

            FT_Done_Face(fc);
            FT_Done_FreeType(ft);
        }
    };
}