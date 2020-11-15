class Solution {
public:
    void duplicateZeros(vector<int>& arr) {
        
        size_t len = arr.size();
        for (auto iter = arr.begin(); iter != arr.end(); ++iter) {
            if (*iter == 0) iter = arr.insert(iter + 1, 0);
        }
        arr = vector(arr.begin(), arr.begin() + len);
    }
};