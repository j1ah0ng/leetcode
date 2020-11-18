#include <vector>
#include <algorithm>

using std::vector;
using std::sort;
using std::max;

class Solution {
public:
    vector<vector<int>> merge(vector<vector<int>>& intervals) {
        
        auto sorted = vector<vector<int>>(intervals);
        sort(sorted.begin(), sorted.end(),
             [&](vector<int> & a, vector<int> & b) {
                 return a[0] > b[0];
        });
        
        vector<vector<int>> merged = vector<vector<int>>();
        auto first = sorted.back();
        sorted.pop_back();
        int last_a = first[0];
        int last_b = first[1];

        while (sorted.size() >= 1) {
            auto next = sorted.back();
            sorted.pop_back();
            int a = next[0];
            int b = next[1];

            if (a <= last_b) {
                last_b = max(last_b, b);
            } else {
                merged.push_back(vector<int>({last_a, last_b}));
                last_a = a;
                last_b = b;
            }
        }
        
        if (merged.size() == 0 || merged.back()[0] != last_a) {
            merged.push_back(vector<int>({last_a, last_b}));
        }
        
        return merged;
    }
};