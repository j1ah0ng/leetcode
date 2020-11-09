/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
#ifndef NULL
#define NULL 0
#endif
#ifndef nullptr
#define nullptr NULL
#endif

bool hasCycle(struct ListNode *head) {
    if (head == nullptr) return false;

    struct ListNode *second_head = head->next;
    while (second_head != head) {
        if (second_head == nullptr || second_head->next == nullptr) return false;
        second_head = second_head->next->next;
        head = head->next;
    }

    return true;

}