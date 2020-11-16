

int removeElement(int* nums, int numsSize, int val){

    int end = numsSize;
    for (int i = 0; i < end; ++i) {
        if (nums[i] == val) {
            nums[i--] = nums[(end--) - 1];
        }
    }
    
    return end;
}