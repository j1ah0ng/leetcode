import static java.lang.Math.ceil;
import static java.lang.Math.floor;
import static java.lang.Math.log10;

class Solution {
    public int findNumbers(int[] nums) {
        
        int acc = 0;
        for (int n : nums) {
            double logOfN = log10(n);
            
            if (logOfN == floor(logOfN)) {  // Rounding down is the same
                if ((ceil(logOfN) + 1) % 2 == 0) acc += 1;
            } else {
                if (ceil(logOfN) % 2 == 0) acc += 1;
            }
        }
        
        return acc;
    }
}