class Solution {
public:
    int findMin(vector<int>& nums) {
        int first = nums[0];
        auto end = nums.end();
        for (auto i = nums.begin() + 1; i != end; ++i) {
            if (*i < first) return *i;
        }
        return first;
    }
};