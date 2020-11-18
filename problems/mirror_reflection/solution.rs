impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        
        let mut gcd = p;
        let mut b = q;
        while b != 0 {
            let tmp = gcd % b;
            gcd = b;
            b = tmp;
        }
        
        let lcm = p * q / gcd;
        
        match (lcm / p) % 2 {
            0 => 0,
            _ => match (lcm / q) % 2 {
                0 => 2,
                _ => 1,
            },
        }
    }
}