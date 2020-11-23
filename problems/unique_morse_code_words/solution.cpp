#include <iostream>
#include <unordered_set>
#include <string>
#include <vector>

using std::string;
using std::vector;
using std::unordered_set;
using std::cout;
using std::endl;

static const string kLetterToMorse[] =  {".-","-...","-.-.","-..",".","..-.","--.",
    "....","..",".---","-.-",".-..","--","-.","---",
    ".--.","--.-",".-.","...","-","..-","...-",
    ".--","-..-","-.--","--.."};

class Solution {
    public:
        int uniqueMorseRepresentations(vector<string>& words) {

            auto set = new unordered_set<string>();
            for (string word : words) {
                string tmp = string();
                for (auto itr = word.begin(); itr != word.end(); ++itr) {
                    tmp.append(kLetterToMorse[*itr - 'a']);
                }
                set->insert(string(tmp));
            }

            return set->size();
        }
};
