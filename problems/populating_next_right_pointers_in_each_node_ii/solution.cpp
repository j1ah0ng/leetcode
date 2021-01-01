/*
// Definition for a Node.
class Node {
public:
    int val;
    Node* left;
    Node* right;
    Node* next;

    Node() : val(0), left(NULL), right(NULL), next(NULL) {}

    Node(int _val) : val(_val), left(NULL), right(NULL), next(NULL) {}

    Node(int _val, Node* _left, Node* _right, Node* _next)
        : val(_val), left(_left), right(_right), next(_next) {}
};
*/

class Solution {
public:
    Node* connect(Node* root) {
        
        if (root == nullptr) return nullptr;

        auto layer = new vector<Node *>{root};
        
        while (!layer->empty()) {
            // Set nexts.
            for (int i = 0; i < layer->size() - 1; ++i) {
                (*layer)[i]->next = (*layer)[i + 1];
            }
            (*layer)[layer->size() - 1]->next = nullptr;
            
            // Enqueue next layer.
            auto new_layer = new vector<Node *>();
            for (auto node : *layer) {
                if (node->left != nullptr) new_layer->push_back(node->left);
                if (node->right != nullptr) new_layer->push_back(node->right);
            }
            delete layer;
            layer = new_layer;
        }
        delete layer;
        return root;
    }

};