import array


def part(arr, low, high):
    pinum = arr[high]
    l = low
    for i in range(low, high):
        if arr[i] < pinum:
            arr[i], arr[l] = arr[l], arr[i]
            l = l + 1
    arr[l], arr[high] = arr[high], arr[l]

    return l


def quicksort(arr, low, high):
    if low < high:
        pi = part(arr, low, high)

        quicksort(arr, low, pi - 1)

        quicksort(arr, pi + 1, high)

    return arr


data = array.array(
    "i", [100, 4, 8, 101, 102, 105, 3, 5, 7, 29, 5, 33, 1, 55, 42, 2, 6, 9, 19, 32, 31]
)

print(data)
print(quicksort(data, 0, len(data) - 1))
