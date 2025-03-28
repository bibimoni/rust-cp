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


// segtree with one-based indexing
// supports range update
//
// build:
// SegTree<T, F, U> seg(<cap>, <default>, <trash>)
// SegTree<int, 
//    [&] (int v_1, int v_2) { return min(v_1, v_2); },
//    [&] (int v_1, int v_2) { return v_1 + v_2; }>
//  st_mn(n, 0, int(1e9));
//
//  vector<int> a(n + 1);
//  seg.build(a); // vector `a` must be one-indexed
// 
// update:
//    seg.update(l, r, inc); // apply a[l -> r] = U(a[l -> r], inc)
//
// query:
//    seg.query(l, r)

#if __cplusplus >= 201703L

template <class T, auto F, auto U> struct SegTree {
  static_assert(is_convertible_v<decltype(F), function<T(T, T)>>, "F must be T(T, T)");
  static_assert(is_convertible_v<decltype(U), function<T(T, T)>>, "U must be T(T, T)");

#else 

template <class T, T (*F)(T, T), T (*U)(T, T)> struct SegTree {

#endif 

  vector<T> st, lazy;
  int n;
  T trash_val;
  T default_value;
  SegTree() : SegTree(0) {}
  explicit SegTree(int _n, T _default_value, T _trash_val) 
  : n(_n), trash_val(_trash_val), default_value(_default_value) {
    st.assign(4 * n + 5, default_value);
    lazy.assign(4 * n + 5, default_value);
  }  

  // one-indexed
  void build(const vector<T> &a) {
    build(a, 1, 1, n);
  }
  void build(const vector<T> &a, int id, int l, int r) {
    lazy[id] = default_value;
    if(l == r) {
      if(l < (int)a.size()) {
        st[id] = a[l];
      }
      return;
    }
    int mid = (l + r) >> 1;
    build(a, id * 2, l, mid);
    build(a, id * 2 | 1, mid + 1, r);
    st[id] = F(st[id * 2], st[id * 2 | 1]);
  }

  void down(int id) {
    if(lazy[id] == default_value) {
      return;
    }
    st[id * 2] = U(st[id * 2], lazy[id]);
    st[id * 2 | 1] = U(st[id * 2 | 1], lazy[id]);
    lazy[id * 2] = U(lazy[id * 2], lazy[id]);
    lazy[id * 2 | 1] = U(lazy[id * 2 | 1], lazy[id]);
    lazy[id] = default_value;
  }

  void update(int u, int v, T val) {
    update(u, v, val, 1, 1, n);
  }
  void update(int i, T val) {
    update(i, i, val, 1, 1, n);
  }
  void update(int u, int v, T val, int id, int l, int r) {
    if(l > r || v < l || r < u) return;
    if(u <= l && r <= v) {
      lazy[id] = U(lazy[id], val);
      st[id] = U(st[id], val);
      return;
    }
    down(id);
    int mid = (l + r) >> 1;
    update(u, v, val, id * 2, l, mid);
    update(u, v, val, id * 2 | 1, mid + 1, r);
    st[id] = F(st[id * 2], st[id * 2 | 1]);
  }

  T query(int u, int v) {
    return query(u, v, 1, 1, n);
  }
  T query(int i) {
    return query(i, i, 1, 1, n);
  }
  T query(int u, int v, int id, int l, int r) {
    if(v < l || r < u) {
      return trash_val;
    }
    if(u <= l && r <= v) {
      return st[id];
    }
    int mid = (l + r) >> 1;
    down(id);
    return F(query(u, v, id * 2, l, mid), query(u, v, id * 2 | 1, mid + 1, r));
  }

  // F usually min function, (g should return true when first enter the function, g(op(st), val) = true)
  // idx_1, return_index, idx_2..., 
  // g(value, return_index) = true
  // g(value, return_index + 1) = false
  int max_right(int l, function<bool(T, T)> g, T val) {
    return max_right(1, 1, n, l, g, val);
  }
  int max_right(int id, int l, int r, int left_bound, function<bool(T, T)> g, T val) {
    if (!g(st[id], val) || r < left_bound) {
      return -1;
    }
    if (l == r) {
      return l;
    }
    down(id);
    int mid = (l + r) >> 1;
    int ret = -1;
    if (g(st[id * 2 | 1], val)) {
      ret = max_right(id * 2 | 1, mid + 1, r, left_bound, g, val);
    }
    if (ret == -1) {
      ret = max_right(id * 2, l, mid, left_bound, g, val);
    }
    return ret;
  }

  // F usually max function, (g should return true when first enter the function, g(op(st), val) = true)
  // idx_1, return_index, idx_2..., 
  // g(value, return_index) = true
  // g(value, return_index - 1) = false
  int min_left(int r, function<bool(T, T)> g, T val) {
    return min_left(1, 1, n, r, g, val);
  }
  int min_left(int id, int l, int r, int right_bound, function<bool(T, T)> g, T val) {
    if (!g(st[id], val) || l > right_bound) {
      return -1;
    }
    if (l == r) {
      return l;
    }
    down(id);
    int mid = (l + r) >> 1;
    int ret = -1;
    if (g(st[id * 2], val)) {
      ret = min_left(id * 2, l, mid, right_bound, g, val);
    }
    if (ret == -1) {
      ret = min_left(id * 2 | 1, mid + 1, r, right_bound, g, val);
    }
    return ret;
  }
};

int add(int x, int y) {
  return x + y;
}

signed main() {
  ios::sync_with_stdio(false); 
  cin.tie(0);
  int n, m, d;
  cin >> n >> m >> d;
  vector<vector<char>> a(n + 1, vector<char>(m + 1, '#'));
  for (int i = 1; i <= n; i++) {
    for (int j = 1; j <= m; j++) {
      cin >> a[i][i];
    }
  }
  reverse(a.begin() + 1, a.end());
  SegTree<int, add, add> st(m + 1, 0, 0);
  for (int cur = 1; cur <= m; cur++) {
    if (a[1][cur] == 'X') {
      st.update(cur, 1);
    }
  }

  auto sqrt = [&] (int x) {
    return x * x;
  };

  const long double EPS = 1e9;
  auto find_r = [&] (int row, int cur_row, int idx) -> {
    int lo = idx, hi = m, ret = -1;
    while (lo <= hi) {
      int mid = (l + r) >> 1;
      long double dist = sqrt(sqrt())
    }
  };

  for (int i = 2; i <= n; i++) {
    SegTree<int, add, add> nst(m + 1, 0, 0);
    for (int cur = 1; cur <= m; cur++) {
      if (a[i][cur] == '#') {
        continue;
      }

    }
  }
}
