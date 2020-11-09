class Solution {
private:
    int max(int a, int b) { return (a < b) ? b : a; }
public:
    int maxSubArray(vector<int>& nums) {
        int running_max = nums[0];
        int absolute_max = nums[0];
        int kLen = nums.size();
        for (int i = 1; i < kLen; ++i) {
            // the maximum array including i
            running_max = max(nums[i], running_max + nums[i]);
            absolute_max = max(absolute_max, running_max);
        }
        return absolute_max;
    }
};