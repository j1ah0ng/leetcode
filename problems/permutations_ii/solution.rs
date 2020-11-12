impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        
        let mut sorted = nums.clone();
        sorted.sort();
        
        Solution::permute_unique_impl(sorted)
    }
    
    pub fn permute_unique_impl(nums: Vec<i32>) -> Vec<Vec<i32>> {
        
        // Recursive function. If length is less than two, simply return
        if nums.len() < 2 {
            vec!(nums)
        }
        
        // Otherwise, for all unique elements, prepend all permutations
        // of the list \ one instance of the unique element with that element   
        else {
            
            let mut uniq = nums.clone();
            uniq.dedup();
            let mut multiset_permutations: Vec<Vec<i32>> = Vec::new();
            
            for unique in uniq.into_iter() {
                let mut remain = nums.clone();
                remain.remove(nums
                    .binary_search(&unique)
                    .unwrap());
                
                for mut permutation in Solution::permute_unique_impl(remain).into_iter() {
                    permutation.insert(0, unique);
                    multiset_permutations.push(permutation);
                }
            }
            
            multiset_permutations
        }
    }
}