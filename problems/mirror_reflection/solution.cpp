class Solution {
public:
    int mirrorReflection(int p, int q) {
        int gcd = p;
        int b = q;
        while (b != 0) {
            int tmp = gcd % b;
            gcd = b;
            b = tmp;
        }
        
        int lcm = p * q / gcd;

        if ((lcm / p) % 2 == 0) {
            return 0;
        } else {
            if ((lcm / q) % 2 == 0) {
                return 2;
            } else return 1;
        }
    }
};