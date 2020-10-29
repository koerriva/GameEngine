#pragma once

namespace Engine{
    using namespace Utils;
    using namespace glm;

    FT_Library ft;
    FT_Face fc;
    struct Character{
        GLuint texture;
        ivec2 size;
        ivec2 bearing;
        GLuint advance;
    };

    class Font{
    private:
        const char* fontpath;
        std::unordered_map<wchar_t,Character> chars;
        explicit Font(const char* fontpath){
            this->fontpath = fontpath;
        }
        ~Font()= default;
        void _init(){
            if(FT_Init_FreeType(&ft)){
                Logger::Error("Can't Init FreeType Library");
            }
            if(FT_New_Face(ft,fontpath,0,&fc)){
                Logger::Error("Can't Load Font [{}]",fontpath);
            }
            FT_Set_Pixel_Sizes(fc,0,24);

            glPixelStorei(GL_UNPACK_ALIGNMENT,1);//禁用字节对齐
            for (wchar_t c = 0; c < 128; ++c) {
                if(FT_Load_Char(fc,c,FT_LOAD_RENDER)){
                    Logger::Error("Can't Load char");
                    continue;
                }
                _GenTexture(c);
            }
            for (wchar_t c = 0x4e00; c < 0x9fa5; ++c) {
                if(FT_Load_Char(fc,c,FT_LOAD_RENDER)){
                    Logger::Error("Can't Load char");
                    continue;
                }
                _GenTexture(c);
            }
            glBindTexture(GL_TEXTURE_2D,0);

            FT_Done_Face(fc);
            FT_Done_FreeType(ft);
        }

        void _GenTexture(wchar_t c){
            GLuint texture;
            glGenTextures(1,&texture);
            glBindTexture(GL_TEXTURE_2D,texture);
            glTexImage2D(GL_TEXTURE_2D,0,GL_RED
                    ,fc->glyph->bitmap.width,fc->glyph->bitmap.rows
                    ,0,GL_RED,GL_UNSIGNED_BYTE
                    ,fc->glyph->bitmap.buffer);
            // 设置纹理选项
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
        void _cleanup(){
            chars.clear();
        }
        static Font* _instance;
    public:
        static void Init(){
            _instance->_init();
        }
        static Character GetChar(wchar_t c){
            return _instance->chars[c];
        }
        static void Cleanup(){
            _instance->_cleanup();
        }
    };

    Font* Font::_instance = new Font("data/font/NotoSansSC-Regular.otf");
}