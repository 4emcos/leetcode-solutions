package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func main() {

	result := mergeTwoLists(
		&ListNode{Val: 1, Next: &ListNode{Val: 2, Next: &ListNode{Val: 4}}},
		&ListNode{Val: 1, Next: &ListNode{Val: 3, Next: &ListNode{Val: 4}}})

	fmt.Println(result)
}

func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	var result *ListNode
	var current *ListNode

	for list1 != nil && list2 != nil {
		if list1.Val < list2.Val {
			if result == nil {
				result = &ListNode{Val: list1.Val}
				current = result
			} else {
				current.Next = &ListNode{Val: list1.Val}
				current = current.Next
			}
			list1 = list1.Next
		} else {
			if result == nil {
				result = &ListNode{Val: list2.Val}
				current = result
			} else {
				current.Next = &ListNode{Val: list2.Val}
				current = current.Next
			}
			list2 = list2.Next
		}
	}

	for list1 != nil {
		if result == nil {
			result = &ListNode{Val: list1.Val}
			current = result
		} else {
			current.Next = &ListNode{Val: list1.Val}
			current = current.Next
		}
		list1 = list1.Next
	}

	for list2 != nil {
		if result == nil {
			result = &ListNode{Val: list2.Val}
			current = result
		} else {
			current.Next = &ListNode{Val: list2.Val}
			current = current.Next
		}
		list2 = list2.Next
	}

	return result

}
