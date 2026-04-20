#include <iostream>
#include <fstream>
#include <vector>

using namespace std;

int minRefuels(int N, int V, const vector<int>& R) {
    for (int dist : R) {
        if (dist > V) return 0;
    }

    int fuel = V;
    int refuels = 0;

    for (int i = 0; i < N - 1; i++) {
        if (fuel < R[i]) return 0;

        fuel -= R[i];

        if (i < N - 2 && fuel < R[i + 1]) {
            refuels++;
            fuel = V;
        }
    }

    return refuels;
}

int main() {
    ifstream fin("input.txt");
    ofstream fout("output.txt");

    int N, V;
    vector<int> R;

    if (fin.is_open()) {
        fin >> N >> V;
        R.resize(N - 1);
        for (int i = 0; i < N - 1; i++) {
            fin >> R[i];
        }
    } else {
        cout << "Введите количество планет N: ";
        cin >> N;
        cout << "Введите емкость бака V: ";
        cin >> V;

        R.resize(N - 1);
        cout << "Введите расстояния между соседними планетами: ";
        for (int i = 0; i < N - 1; i++) {
            cin >> R[i];
        }
    }

    int result = minRefuels(N, V, R);

    cout << "Результат: " << result << endl;
    fout << result << endl;

    return 0;
}
