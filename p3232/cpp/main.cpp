class Solution
{
public:
    bool canAliceWin(vector<int> &nums)
    {
        int single=0, ddouble = 0;
        for (int i = 0; i < nums.size(); i++)
        {
            if (nums[i] <= 9)
            {
                single +=nums[i];
            }else{
                ddouble +=nums[i];
            }
            
        }
        if (single == ddouble)
        {
            return false;
        }

        return true;
    }
};