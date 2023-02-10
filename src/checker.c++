#include "bits/stdc++.h"
using namespace std;

string INPUT = "data/input.txt";
string BRUTEFORCE_OUTPUT = "data/bruteforce_output.txt";

void bruteforce(int num) {
    bool flag = true;

    for(int i = 2; i < num; i++) {
        if (num % i == 0) flag = false; 
    }

    flag ? cout << "True" : cout << "False"; 
    cout<< "\n";
}

int main() {
    freopen(INPUT.c_str(), "r", stdin);
    freopen(BRUTEFORCE_OUTPUT.c_str(), "w", stdout);

    int n;
    cin >> n;
    bruteforce(n);
}
