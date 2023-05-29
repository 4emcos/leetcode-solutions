package main

import "fmt"

func main() {

	result := twoSum([]int{2, 7, 11, 15}, 9)

	fmt.Println(result)

}

func twoSum(nums []int, target int) []int {
	numsMap := make(map[int]int)
	for i, v := range nums {
		numsMap[v] = i
	}
	for i, v := range nums {
		if j, ok := numsMap[target-v]; ok && i != j {
			return []int{i, j}
		}
	}
	return []int{}
}
