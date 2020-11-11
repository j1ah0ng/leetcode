int trap(int* height, int heightSize){
    
    if (heightSize < 3) return 0;
    
    int max_height = 0;
    int ground_vol = 0;
    for (int i = 0; i < heightSize; ++i) {
        ground_vol += height[i];
        if (height[i] > max_height) max_height = height[i];
    }
    
    int acc = heightSize * max_height - ground_vol;
    for (int h = max_height; h >= 0; --h) {
        for (int i = 0; i < heightSize; ++i) {
            if (height[i] >= h) break;
            acc -= 1;
        }
        for (int i = heightSize - 1; i >= 0; --i) {
            if (height[i] >= h) break;
            acc -= 1;
        }
    }
    
    return acc;
}