int max(int a, int b) {
    return (a > b) ? a : b;
}

int maxSubArray(int* nums, int numsSize){
    int max_inc = nums[0];
    int abs_max = nums[0];
    
    for (int i = 1; i < numsSize; ++i) {
        max_inc = max(nums[i], nums[i] + max_inc);
        abs_max = max(abs_max, max_inc);
    }
    
    return abs_max;
}