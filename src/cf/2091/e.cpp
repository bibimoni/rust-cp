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
#define int int64_t

signed main() {
  ios::sync_with_stdio(false); 
  cin.tie(0);
  const int N = 1e7;
  vector<int> ok(N + 1, 1);
  vector<int> ans(N + 1, 0);
  for (int i = 2; i <= N; i++) {
    if (ok[i]) {
      for (int j = i; j <= N; j += i) {
        ok[j] = false;
        ans[j] += 1;
      }
    }
  }
  for (int i = 1; i <= N; i++) {
    ans[i] += ans[i - 1];
  }
  int tt;
  cin >> tt;
  while (tt--) {
    int n;
    cin >> n;
    cout << ans[n] << '\n';
  }
}

