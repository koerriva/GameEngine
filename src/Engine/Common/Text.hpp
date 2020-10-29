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
        static Text from(string text){
            return Text(wstring_convert<codecvt_utf8<wchar_t>>().from_bytes(text));
        }
//
//        static string to(wstring& text){
//            return wstring_convert<codecvt_utf8<wchar_t>>().to_bytes(text);
//        }
        wstring::const_iterator begin() const {
            return data.begin();
        }
        wstring::const_iterator end() const {
            return data.end();
        }

//        static Text from(const string& str)
//        {
//            setlocale(LC_ALL, "chs");
//            const char* p_source = str.c_str();
//            size_t new_size = str.size() + 1;
//            auto *p_destination = new wchar_t[new_size];
//            wmemset(p_destination, 0, new_size);
////            mbstowcs(p_destination, p_source, new_size);
//            size_t out_size;
//            mbstowcs_s(&out_size,p_destination,new_size,p_source,new_size-1);
//            Text t((wstring(p_destination)));
//            delete[]p_destination;
//            setlocale(LC_ALL, "C");
//            return t;
//        }
        explicit Text(wstring data):data(std::move(data)){}
        explicit Text(const string& data){
            auto ws = wstring_convert<codecvt_utf8<wchar_t>>().from_bytes(data);
            this->data = move(ws);
        }
    };
}