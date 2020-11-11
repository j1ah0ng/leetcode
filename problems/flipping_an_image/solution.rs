impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
                
        // Flip, then invert
        a.clone().into_iter()
            .map( |v: Vec<i32>| {
                v.into_iter().rev().map( |i: i32|
                    match i {
                        1 => 0,
                        0 => 1,
                        _ => i,
                    }
                ).collect::<Vec<i32>>()
            }).collect::<Vec<Vec<i32>>>()
            
    }
}