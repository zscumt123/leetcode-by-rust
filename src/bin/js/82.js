/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @return {ListNode}
 */
var deleteDuplicates = function(head) {
  if(!head) {
    return head
  }
  let hair = new ListNode(-1, head);

  let pointer = hair

  while (pointer.next && pointer.next.next) {
    if(pointer.next.val === pointer.next.next.val) {
      const x = pointer.next.val;
      while(pointer.next && pointer.next.val === x) {
        pointer.next = pointer.next.next;
      }
    } else {
      pointer = pointer.next;
    }
  }

  return hair.next

};
