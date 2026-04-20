#include <iostream>
#include <fstream>
#include <vector>

using namespace std;

string checkPalindrome3(int x) {
    if (x < 100 || x > 999) return "-";

    int a = x / 100;
    int c = x % 10;

    return (a == c) ? "Yes" : "No";
}

int main() {
    ifstream fin("input.txt");
    ofstream fout("output.txt");

    int N;
    vector<int> nums;

    if (fin.is_open()) {
        fin >> N;
        nums.resize(N);
        for (int i = 0; i < N; i++) {
            fin >> nums[i];
        }
    } else {
        cout << "Введите количество чисел N: ";
        cin >> N;
        nums.resize(N);

        cout << "Введите числа: ";
        for (int i = 0; i < N; i++) {
            cin >> nums[i];
        }
    }

    for (int i = 0; i < N; i++) {
        string result = checkPalindrome3(nums[i]);
        cout << result;
        fout << result;

        if (i + 1 < N) {
            cout << ' ';
            fout << ' ';
        }
    }

    cout << endl;
    fout << endl;

    return 0;
}
