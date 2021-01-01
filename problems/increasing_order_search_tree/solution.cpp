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
class Solution {
public:
    TreeNode* increasingBST(TreeNode* root) {
        TreeNode b = TreeNode(0);
        TreeNode * p = &b;
        traverse(root, p);
        return b.right;
    }
    
    void traverse(TreeNode * root, TreeNode *& current) {
        if (root == nullptr) return;
        
        traverse(root->left, current);
        root->left = nullptr;
        current->right = root;
        current = root;
        traverse(root->right, current);
    }
};