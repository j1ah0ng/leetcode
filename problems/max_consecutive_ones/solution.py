class Solution:
    def findMaxConsecutiveOnes(self, nums: List[int]) -> int:
        
        # Iterate, keeping track of the max
        abs_max = 0
        run = 0
        
        for num in nums:
            if num == 1:
                run += 1
            else:
                abs_max = max(abs_max, run)
                run = 0
                
        return max(abs_max, run)