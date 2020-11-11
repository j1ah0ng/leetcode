impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        
        // Similar algorithm to saliency as used in imgproc.
        // If the "tank" is not at least 3 wide, it can't hold any water.
        let kLen = height.len();
        if kLen < 3 { return 0; }
        
        let kMaxHeight = *height.iter().clone().max().expect("");
        let mut tank: Vec<i32> = vec![kMaxHeight; kLen];
        
        // Iterate from top to bottom; 
        // for each iteration, subtract the outer layers of water not bounded by 
        // a wall.
        for h in (1..=kMaxHeight).rev() {
            
            for idx in 0..kLen {
                if height[idx] >= h { break; }
                else {
                    tank[idx] = h - 1;
                }
            }
            for idx in (0..kLen).rev() {
                if height[idx] >= h { break; }
                else {
                    tank[idx] = h - 1;
                }
            }
        }
        
        // Accumulate all water
        let mut acc: i32 = 0;
        for i in 0..kLen {
            acc += tank[i] - height[i];
        }
        
        acc
    }
}