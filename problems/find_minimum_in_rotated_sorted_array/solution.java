class Solution {
    public int findMin(int[] nums) {
        int first = nums[0];
        for (int i = 1; i < nums.length; ++i) {
            if (nums[i] < first) return nums[i];
        }
        return first;
    }
}