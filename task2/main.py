def min_refuels(n, v, r):
    for dist in r:
        if dist > v:
            return 0

    fuel = v
    refuels = 0

    for i in range(n - 1):
        if fuel < r[i]:
            return 0

        fuel -= r[i]

        if i < n - 2 and fuel < r[i + 1]:
            refuels += 1
            fuel = v

    return refuels


try:
    with open("input.txt", "r", encoding="utf-8") as f:
        data = list(map(int, f.read().split()))
        n = data[0]
        v = data[1]
        r = data[2:]
except:
    n = int(input("Введите количество планет N: "))
    v = int(input("Введите емкость бака V: "))
    r = list(map(int, input("Введите расстояния между соседними планетами: ").split()))

result = min_refuels(n, v, r)

print("Результат:", result)

with open("output.txt", "w", encoding="utf-8") as f:
    f.write(str(result) + "\n")
