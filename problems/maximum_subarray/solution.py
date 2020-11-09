class Solution(object):
    def maxSubArray(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        
        max_inc = nums[0]
        abs_max = nums[0]
        for i in range(1, len(nums)):
            max_inc = max(nums[i], nums[i] + max_inc)
            abs_max = max(max_inc, abs_max)
            
        return abs_max
        