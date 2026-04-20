#include <iostream>
#include <fstream>
#include <cmath>

using namespace std;

bool isPrime(int n) {
    if (n < 2) return false;
    if (n == 2) return true;
    if (n % 2 == 0) return false;

    for (int i = 3; i * i <= n; i += 2) {
        if (n % i == 0) return false;
    }
    return true;
}

int nearestPrimeDistance(int n) {
    if (isPrime(n)) return 0;

    int d = 1;
    while (true) {
        if (isPrime(n - d) || isPrime(n + d)) {
            return d;
        }
        d++;
    }
}

int main() {
    ifstream fin("input.txt");
    ofstream fout("output.txt");

    int N;
    if (fin.is_open()) {
        fin >> N;
    } else {
        cout << "Введите N: ";
        cin >> N;
    }

    int result = nearestPrimeDistance(N);

    cout << "Результат: " << result << endl;
    fout << result << endl;

    return 0;
}
