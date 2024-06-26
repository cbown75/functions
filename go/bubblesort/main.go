package main

import "fmt"

func bubblesort(numbs []int) {
  for i := range numbs {
    for r := 0; r < (len(numbs) - i - 1); r++ {
      left := numbs[r]
      right := numbs[r+1]
      if left > right {
        numbs[r] = right
        numbs[r+1] = left
      }
    }
  }
}

func main() {
  data := []int{100, 4, 8, 3, 5, 7, 29, 5, 33, 1, 55, 42, 101, 202, 205, 76, 2, 6, 9, 19, 32, 31}

  fmt.Println(data)
  bubblesort(data)
  fmt.Println(data)
}
