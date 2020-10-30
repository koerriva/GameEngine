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

        [[nodiscard]] wstring::const_iterator begin() const {
            return data.begin();
        }

        [[nodiscard]] wstring::const_iterator end() const {
            return data.end();
        }

        explicit Text(wstring data):data(std::move(data)){}
        explicit Text(const string& data){
            auto ws = wstring_convert<codecvt_utf8<wchar_t>>().from_bytes(data);
            this->data = move(ws);
        }
    };
}