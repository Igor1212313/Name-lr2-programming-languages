def is_prime(n):
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    i = 3
    while i * i <= n:
        if n % i == 0:
            return False
        i += 2
    return True

def nearest_prime_distance(n):
    if is_prime(n):
        return 0
    d = 1
    while True:
        if is_prime(n - d) or is_prime(n + d):
            return d
        d += 1

try:
    with open("input.txt", "r", encoding="utf-8") as f:
        data = f.read().strip()
        N = int(data)
except:
    N = int(input("Введите N: "))

result = nearest_prime_distance(N)

print("Результат:", result)

with open("output.txt", "w", encoding="utf-8") as f:
    f.write(str(result) + "\n")
