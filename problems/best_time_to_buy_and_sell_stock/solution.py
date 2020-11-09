class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        
        min_idx = 0
        max_pft = 0
        
        for i, p, in enumerate(prices):
            if p < prices[min_idx]:
                min_idx = i
            else:
                if p - prices[min_idx] > max_pft:
                    max_pft = p - prices[min_idx]
                    
        return max_pft
        