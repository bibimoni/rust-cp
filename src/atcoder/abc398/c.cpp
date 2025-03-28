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
  int n;
  cin >> n;
  map<int, int> mp;
  vector<int> a(n);
  for (int i = 0; i < n; i++) {
    int x;
    cin >> x;
    a[i] = x;
    mp[x] += 1;
  }
  int ans = -1;
  for (int i = 0; i < n; i++) {
    if (mp[a[i]] == 1) {
      ans = max(ans, i + 1);
    }
  }
  cout << ans << '\n';
}

