#include <vector>
class Solution {
public:
    int numberOfAlternatingGroups(std::vector<int>& colors) {
        int len = colors.size();
        int leff_idx = 0;
        int middle_idx = 1;
        int right_idx = 2;
        int iter = 0;
        int result = 0;
        while (iter < len )
        {
            if (colors[right_idx % len] == colors[leff_idx % len]
            && colors[middle_idx % len] != colors[leff_idx% len]
            && colors[middle_idx % len] != colors[right_idx % len])
            {
                result ++;
            }
            
            leff_idx++;
            right_idx++;
            middle_idx++;
            iter++;
        }
        
        return result;
    }
};