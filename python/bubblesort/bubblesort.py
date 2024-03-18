import array


def bubblesort(numbs):
    l = len(numbs)
    for i in range(l):
        for r in range(0, l - i - 1):
            left = numbs[r]
            right = numbs[r + 1]
            if left > right:
                numbs[r] = right
                numbs[r + 1] = left
    return numbs


def reverse_bubblesort(numbs):
    l = len(numbs)
    for i in range(l):
        for r in range(0, l - i - 1):
            left = numbs[r]
            right = numbs[r + 1]
            if left < right:
                numbs[r] = right
                numbs[r + 1] = left
    return numbs


nums = array.array("i", [100, 4, 8, 3, 5, 7, 29, 5, 33, 1, 55, 42, 2, 6, 9, 19, 32, 31])
print(bubblesort(nums))

print("")
print(reverse_bubblesort(nums))
