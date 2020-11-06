class Solution {
public:
    int minCostToMoveChips(vector<int>& position) {
        int num_in_odd_pos = 0;
        int num_in_even_pos = 0;
        for (int i = 0; i < position.size(); ++i) {
            if (position[i] % 2 == 0) ++num_in_even_pos;
            else ++num_in_odd_pos;
        }
        return min(num_in_odd_pos, num_in_even_pos);
    }
};