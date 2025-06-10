#include<iostream>
#include<string>
#include<vector>
using namespace std; 

class Solution {
public:
    bool quick_check(char a, char b) {
        if (a == '(' && b == ')') { return true; }
        else if (a == '{' && b == '}') { return true; }
        else if (a == '[' && b == ']') { return true; }
        else { return false; }
    }
    
    bool isValid(string s) {
        int n = s.size();
        vector<int> v;


        for (auto i:s) {
            if (v.empty()) { v.push_back(i); } 
            else {
                if (Solution::quick_check(v.back(), i)) { v.pop_back(); }
                else { v.push_back(i); }
            }
        }

        return v.empty();
    }
};
