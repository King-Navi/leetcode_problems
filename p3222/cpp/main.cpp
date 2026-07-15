#include <iostream>
#include <string>

class Solution
{
public:
    std::string winningPlayer(int x, int y)
    {
        std::string alice = "Alice";
        std::string bob = "Bob";
        bool alice_win = false;
        while (true)
        {
            if (x - 1 < 0 || y - 4 < 0)
            {
                break;
            }
            x -= 1;
            y -= 4;
            alice_win = !alice_win;
        }

        if (alice_win)
        {
            return alice;
        }
        else
        {
            return bob;
        }
    }
};