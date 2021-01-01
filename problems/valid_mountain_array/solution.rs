impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        
        if arr.len() < 3 { return false }
        
        // We must have a nonzero run of increasing numbers
        // followed by a nonzero run of decreasing numbers.
        let mut inc_len: i32 = 0;
        let mut dec: bool    = false;
        
        for i in (0..arr.len() - 1) {
            if !dec {
                // Initial increasing run.
                if arr[i] > arr[i + 1] { 
                    match inc_len > 0 {
                        true => dec = true,
                        false => return false
                    };
                }
                else if arr[i] == arr[i + 1] { return false }
                else { inc_len += 1;}
            } else {
                // Decreasing run.
                if arr[i] <= arr[i + 1] { return false }
            }
        }
        
        return dec
    }
}