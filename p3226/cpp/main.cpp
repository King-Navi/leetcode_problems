#include <iostream>
#include <bit>

class Solution {
public:
    int minChanges(int n, int k) {
        if ((n & k) !=k)
        {
            return -1;
        }
        int nuevo_value = (n ^k);
        return std::popcount(static_cast<unsigned int>(nuevo_value));
    }
};