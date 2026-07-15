#include <string>
#include <algorithm> // Para usar std::swap

class Solution {
public:
    std::string getSmallestString(std::string s) {
        int len = s.length();
        for (int i = 0; i < len - 1; i++) {
            int actual = s[i] - '0';
            int siguiente = s[i+1] - '0';
            
            if (actual % 2 == siguiente % 2) {
                if (actual > siguiente) {
                    std::swap(s[i], s[i+1]); 
                    return s;
                }
            }
        }
        
        return s;
    };
};