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
    ListNode* insertionSortList(ListNode* head) {
        
        if (head == nullptr) return nullptr;
        
        // store the tail position
        auto tail = head;
        
        // iterate over the unsorted list
        auto i = head->next;
        while (i != nullptr) {
            
            // note that tail always points to the node before i
            
            // find insertion position in the sorted list
            // special case for insertion before head
            if (i->val < head->val) {

                tail->next = tail->next->next;    // tail skip over i
                i->next = head;             // emplace i
                head = i;                   // set new head
                i = tail->next;             // increment i
            } 
            
            // normal case for insertion after head
            else {
            
                for (auto j = head; j != i; j = j->next) {

                    // always insert AFTER j 
                    if (j->next->val >= i->val) {

                        // check if insertion is at j == tail
                        if (j != tail) {
                            // perform the insertion
                            tail->next = tail->next->next;
                            auto tmp = j->next;
                            j->next = i;
                            i->next = tmp;
                            i = tail->next;
                        } else {
                            // relink tail
                            tail = tail->next;
                            i = i->next;
                        }
                        break;
                    }
                }
            }
        }
        
        return head;
    }
};