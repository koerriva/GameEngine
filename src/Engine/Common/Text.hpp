#pragma once

#include <codecvt>
#include <string>
#include <utility>

using namespace std;

namespace Engine::Common{
    class Text{
    private:
        wstring data;;
    public:
        static Text from(const string& text){
            return Text(wstring_convert<codecvt_utf8<wchar_t>>().from_bytes(text));
        }

        static wstring s_2_w(const string& text){
            return wstring_convert<codecvt_utf8<wchar_t>>().from_bytes(text);
        }

        [[nodiscard]] wstring::const_iterator begin() const {
            return data.begin();
        }

        [[nodiscard]] wstring::const_iterator end() const {
            return data.end();
        }

        explicit Text(wstring data):data(std::move(data)){}
        explicit Text(const string& data){
            #ifdef __APPLE__
            auto ws = wstring_convert<codecvt_utf8<wchar_t>>().from_bytes(data);
            this->data = std::move(ws);
            #else
            auto ws = s2ws(data);
            this->data = std::move(ws);
            #endif
        }

        wstring& value() {
            return data;
        }

        static wstring s2ws(const string& s){
            size_t convertedChars=0;
            string curLocale=setlocale(LC_ALL,nullptr);   //curLocale="C"
            setlocale(LC_ALL,"chs");
            setlocale(LC_CTYPE, "zh_CN.UTF-8");

            const char* source=s.c_str();
            size_t charNum=sizeof(char)*s.size()+1;
            wchar_t* dest=new wchar_t[charNum];

            #ifndef __APPLE__
            mbstowcs_s(&convertedChars,dest,charNum,source,_TRUNCATE);
            #endif

            wstring result=dest;
//            wcout << result << endl;

            delete[] dest;
            setlocale(LC_ALL,curLocale.c_str());
            return result;
        }
    };
}