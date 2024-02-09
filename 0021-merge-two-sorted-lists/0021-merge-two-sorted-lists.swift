/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     public var val: Int
 *     public var next: ListNode?
 *     public init() { self.val = 0; self.next = nil; }
 *     public init(_ val: Int) { self.val = val; self.next = nil; }
 *     public init(_ val: Int, _ next: ListNode?) { self.val = val; self.next = next; }
 * }
 */
class Solution {
    func mergeTwoLists(_ list1: ListNode?, _ list2: ListNode?) -> ListNode? {
        guard let l1 = list1 else { return list2}
        guard let l2 = list2 else { return list1}
        
        let dummyNode = ListNode(0)
        var mergedList: ListNode? = dummyNode
        var l1Curr:ListNode? = l1
        var l2Curr:ListNode? = l2

    while l1Curr != nil && l2Curr != nil {
        if l1Curr!.val < l2Curr!.val {
            mergedList?.next = l1Curr
            l1Curr = l1Curr?.next
        } else {
        mergedList?.next = l2Curr
            l2Curr = l2Curr?.next
        }
        mergedList = mergedList?.next
    }
    if l1Curr != nil {
        mergedList?.next = l1Curr
    }  
    if l2Curr != nil {
        mergedList?.next = l2Curr
    } 
    return dummyNode.next
    }
}