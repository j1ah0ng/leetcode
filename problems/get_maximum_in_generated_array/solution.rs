impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        
        // just generate the array
        let mut v: Vec<i32> = Vec::new();

        for k in 0..n+1 {
            let i = k as usize;
            if i < 2 { v.push(k); }
            else {
                if i % 2 == 0 {
                    v.push(v[i/2]);
                } else {
                    v.push(v[(i-1)/2] + v[(i-1)/2 + 1]);
                }
            }
        }
        
        v.into_iter().max().expect("")
        
    }
}