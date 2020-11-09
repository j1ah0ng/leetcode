

int findMin(int* nums, int numsSize){
    int first = nums[0];
    for (int i = 1; i < numsSize; ++i) {
        if (nums[i] < first) return nums[i];
    }
    return first;
}