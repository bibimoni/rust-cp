/**
 * Author: distiled
 */
#include<bits/stdc++.h>
using namespace std;

#ifdef DEBUG
#include </Users/distiled/codeStuff/templates/debug.h>
#else
#define dbg(x...)
#endif
#define int long long

template <typename T>
class Array {
public:
    Array(int size, T initValue);
    ~Array();
    void print();
private:
    int size;
    T * p;
};

// TODO
template<typename T>
Array<T>::Array(int size, T initValue) : size(size) {
    p = new T[size];
    p[0] = initValue;
}
template<typename T>
Array<T>::~Array() {
    delete[] p;
    p = nullptr;
}
template<typename T>
void Array<T>::print() {
  for(int i = 0; i < size; i++) {
    cout << p[i] << " \n"[i == size - 1];
  }
}

signed main() {
  ios::sync_with_stdio(false); 
  cin.tie(0);
  Array<int> a1(5, 0);
  a1.print();  
}

