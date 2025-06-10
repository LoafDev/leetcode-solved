#include <iostream>
#include <stack>

class Solution
{
public:
    bool isValid(std::string s)
    {
        std::stack<char> st;
        char a[256];

        a['('] = ')';
        a['['] = ']';
        a['{'] = '}';

        for (char &c : s)
        {
            if (c == '(' || c == '[' || c == '{')
            {
                st.push(c);
            }
            else
            {
                if (st.empty() || a[st.top()] != c)
                {
                    return false;
                }
                st.pop();
            }
        }

        if (!st.empty())
        {
            return false;
        }

        return true;
    }
};

int main()
{
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);

    Solution sol;

    if (sol.isValid("()[]{}"))
    {
        std::cout << "true\n";
    }
    else
    {
        std::cout << "no\n";
    }

    return 0;
}