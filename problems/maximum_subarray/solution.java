class Solution {
    public int maxSubArray(int[] nums) {
        int maxInc = nums[0];
        int absMax = nums[0];
        for (int i = 1; i < nums.length; ++i) {
            maxInc = max(nums[i], nums[i] + maxInc);
            absMax = max(maxInc, absMax);
        }
        return absMax;
    }
    
    private int max(int a, int b) {
        return (a > b) ? a : b;
    }
}