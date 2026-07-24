#include <string>

class Solution {
public:
    std::string getEncryptedString(std::string s, int k) {
        std::string result = "";
        for (int i=0; i<s.length();i++ ){
            result.push_back(s[(i+k)% s.length()]);
        }
        return result;
    }
};