

void merge(int* nums1, int nums1Size, int m, int* nums2, int nums2Size, int n){
    
    if (nums2Size == 0) return;

    // Begin by moving all elements in nums1 to the tail.
    for (int i = nums1Size - 1; i >= n; --i) {
        nums1[i] = nums1[i - n];
        nums1[i - n] = 0;
    }
    
    // Merge.
    int i = 0;
    int j = 0;
    int ins = 0;
    while (i < m && j < n) {
        if (nums1[i + n] < nums2[j]) {
            // Insert from nums1.
            nums1[ins++] = nums1[(i++) + n];
        } else {
            // Insert from nums2.
            nums1[ins++] = nums2[j++];
        }
    }
    
    // Only need to fix remainder of list if nums2 hasn't run out.
    if (j < n) {
        while (ins < nums1Size) {
            nums1[ins++] = nums2[j++];
        }
    }
}