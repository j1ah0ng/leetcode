class Solution {
public:
    vector<int> findMinHeightTrees(int n, vector<vector<int>>& edges) {

        // Trivial case
        if (n == 1) return vector<int>{0};
        if (n == 2) return vector<int>{0, 1};
        
        /* Approach:
         * Do a BFS, stripping away leaves one layer at a time until we are left with
         * one node (in the case of a perfectly balanced tree) or two (in the case of 
         * a slightly imbalanced tree)
         */
        
        // Map each node to the set of nodes immediately adjacent
        map<int, set<int>> path_map {};
        for (auto edge : edges) {
            path_map[edge[0]].insert(edge[1]);
            path_map[edge[1]].insert(edge[0]);
        }
        
        // Need dynalloc in order to change what leaves refers to
        set<int> *leaves = new set<int>();
        
        // Get the initial set of leaves. From here, we work inwards to avoid
        // nested looping
        for (auto i = path_map.begin(); i != path_map.end(); ++i) {
            if (i->second.size() == 1) leaves->insert(i->first);
        }
        
        // Keep track of how many nodes are "left"
        int left = path_map.size();
        
        // Strip layers of leaves until two nodes left
        do {
            
            // Keep track of how many we have, at the beginning
            left = left - leaves->size();
            
            // Track new set of leaves
            set<int> *new_leaves = new set<int>();
            
            // Mark and remove all nodes with one adjacent node as leaves
            for (auto leaf : *leaves) {
                
                // Traverse the only possible edge up from the leaf, and remove
                // that edge from the parent.
                int parent = *path_map[leaf].begin();
                path_map[parent].erase(leaf);
                
                // If it is now a leaf, add to the set of leaves
                if (path_map[parent].size() == 1)
                    new_leaves->insert(parent);
                
            }
            
            // Use new set of leaves
            delete leaves;
            leaves = new_leaves;
            
        } while (left > 2);
        
        // found them
        vector<int> remaining {};
        for (auto leaf : *leaves) {
            remaining.push_back(leaf);
        }
        delete leaves;
        return remaining;
    }
};