/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */

#include <unordered_set>

class Solution {
public:
    bool hasCycle(ListNode *head) {

        if (head == nullptr) return false;
        
        ListNode *second_head = head->next;
        while (second_head != head) {
            if (second_head == nullptr || second_head->next == nullptr) return false;
            second_head = second_head->next->next;
            head = head->next;
        }
        
        return true;
    }
};