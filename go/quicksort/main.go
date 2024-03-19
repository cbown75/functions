package main

import "fmt"

func part(slc []int, low int, high int) int {
  pinum := slc[high]
  l := low
  for i := low; i < high; i++ {
    if slc[i] < pinum {
      sl := slc[l]
      si := slc[i]
      slc[i] = sl
      slc[l] = si
      l = l + 1
    }
  }
  nsl := slc[l]
  nh := slc[high]
  slc[l] = nh
  slc[high] = nsl

  return l
}

func quicksort(slc []int, low int, high int) {
  if low < high {
    pi := part(slc, low, high)

    quicksort(slc, low, pi - 1)

    quicksort(slc, pi + 1, high)
  }
}

func main() {
  data := []int{100, 4, 8, 3, 5, 7, 29, 5, 33, 1, 55, 42, 101, 202, 205, 76, 2, 6, 9, 19, 32, 31}

  fmt.Println(data)
  quicksort(data, 0, len(data)-1)
  fmt.Println(data)
}


