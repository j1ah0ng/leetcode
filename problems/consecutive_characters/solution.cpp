class Solution {
public:
    int maxPower(string s) {
        
        int max_len = 1;
        int working_char = s[0];
        int working_len = 1;
        
        for (auto i = s.begin() + 1; i != s.end(); ++i) {
            
            if (*i == working_char) ++working_len;
            else {
                // check if we have a record
                if (working_len > max_len) max_len = working_len;
                working_len = 1;
                working_char = *i;
            }
        }
        
        return std::max(working_len, max_len);
    }
};