impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        
        // Allocate the final array
        let kLen = nums.len();
        let mut output: Vec<i32> = nums.clone();
        
        // Iterate over the output array. Store left-side products
        // in the output array.
        let mut acc: i32 = 1;
        for i in 0..kLen {
            output[i] = acc;
            acc *= nums[i];
        }
        
        // Iterate once again. Now combine right-side products.
        acc = 1;
        for i in (0..kLen).rev() {
            output[i] *= acc;
            acc *= nums[i];
        }
        
        output
    }
}