use std::cmp::Ordering;
use std::collections::HashSet;
use std::vec::Vec;

impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        
        // Make sure points are distinct
        let mut set = HashSet::new();
        set.insert(p1[0]);
        set.insert(p2[0]);
        set.insert(p3[0]);
        set.insert(p4[0]);
        if set.len() < 2 { return false; }
        set = HashSet::new();
        set.insert(p1[1]);
        set.insert(p2[1]);
        set.insert(p3[1]);
        set.insert(p4[1]);
        if set.len() < 2 { return false; }

        // Sort all points by increasing order of x-axis.
        let mut all_points: Vec<Vec<i32>> = vec![p1, p2, p3, p4];
        all_points.sort_by(|a, b| { let i = a[0].cmp(&b[0]);
            match i {
                Ordering::Equal => a[1].cmp(&b[1]),
                _ => i,
            }
        });
        
        // Choosing the first two points necessarily gives us one side, as does the next.
        // Then, we can compare absolute value of slopes and lengths; then the length 
        // between the remaining sides. 
        let dy_1 = (all_points[1][1] - all_points[0][1]).abs();
        let dx_1 = (all_points[1][0] - all_points[0][0]).abs();
        let dy_2 = (all_points[3][1] - all_points[2][1]).abs();
        let dx_2 = (all_points[3][0] - all_points[2][0]).abs();
        let dy_3 = (all_points[2][1] - all_points[0][1]).abs();
        let dx_3 = (all_points[2][0] - all_points[0][0]).abs();
        
        // Unequal length or slope.
        if dy_1 != dy_2 || dx_1 != dx_2 { return false; }
        if (dy_1*dy_1 + dx_1*dx_1) 
            != (dy_3*dy_3 + dx_3*dx_3) { return false; }
        
        // Orthogonality. Check dot between (0,1) and (0,2)
        if (all_points[2][0] - all_points[0][0]) 
            * (all_points[1][0] - all_points[0][0]) +
            (all_points[2][1] - all_points[0][1])
            * (all_points[1][1] - all_points[0][1]) != 0 { return false; }


        return true;
    }
}
