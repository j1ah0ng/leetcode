/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
#include <algorithm>
#include <cmath>

using std::abs;

class Solution {
public:
    int maxAncestorDiff(TreeNode* root, int min = INT_MAX, int max = INT_MIN) {
        if (root == nullptr) return max - min;
        min = std::min(min, root->val);
        max = std::max(max, root->val);
        return std::max(maxAncestorDiff(root->left, min, max), maxAncestorDiff(root->right, min, max));
    }
};