/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* mergeTwoLists(ListNode* l1, ListNode* l2) {
        
        if (l1 == nullptr) return l2;
        if (l2 == nullptr) return l1;
        
        ListNode* p_a = l1;
        ListNode* p_b = l2;
        ListNode* head = nullptr;
        
        if (p_a->val <  p_b->val) {
            head = p_a;
            p_a = p_a->next;
        } else {
            head = p_b;
            p_b = p_b->next;
        }
        
        ListNode* prev = head;
            
        while (p_a != nullptr && p_b != nullptr) {
            ListNode* next = nullptr;
                
            if (p_a->val <  p_b->val) {
                next = p_a;
                p_a = p_a->next;
            } else {
                next = p_b;
                p_b = p_b->next;
            }
            
            prev->next = next;
            prev = next;
        }
        
        // One or both of the lists are exhausted.
        if (p_a != nullptr) prev->next = p_a;
        if (p_b != nullptr) prev->next = p_b;
        
        return head;
    }
};