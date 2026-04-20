def check_palindrome3(x):
    if x < 100 or x > 999:
        return "-"
    a = x // 100
    c = x % 10
    return "Yes" if a == c else "No"

try:
    with open("input.txt", "r", encoding="utf-8") as f:
        data = list(map(int, f.read().split()))
        n = data[0]
        nums = data[1:]
except:
    n = int(input("Введите количество чисел N: "))
    nums = list(map(int, input("Введите числа: ").split()))

results = [check_palindrome3(nums[i]) for i in range(n)]

print(" ".join(results))

with open("output.txt", "w", encoding="utf-8") as f:
    f.write(" ".join(results) + "\n")
