use lca::*;
pub mod lca {
  use std::mem::swap;
  pub struct Lca {
    parent: Vec<Vec<usize>>,
    dep: Vec<usize>,
  }

  impl Lca {
    // one indexed
    pub fn new(root: usize, adj: &Vec<Vec<usize>>) -> Self {
      let n = adj.len() - 1;
      let lg: usize = (usize::BITS - n.leading_zeros()) as usize;
      let (dep, parent) = {
        fn dfs(
          dep: &mut Vec<usize>,
          parent: &mut Vec<Vec<usize>>,
          u: usize,
          p: usize,
          adj: &Vec<Vec<usize>>,
        ) {
          parent[u][0] = p;
          for ll in 1..parent[u].len() {
            parent[u][ll] = parent[parent[u][ll - 1]][ll - 1];
          }
          for &v in &adj[u] {
            if v == p {
              continue;
            }
            dep[v] = dep[u] + 1;
            dfs(dep, parent, v, u, adj);
          }
        }
        let mut parent = vec![vec![0; lg]; n + 1];
        let mut dep = vec![0; n + 1];
        dfs(&mut dep, &mut parent, root, root, adj);
        (dep, parent)
      };
      Self { parent, dep }
    }

    pub fn lca(&self, u: usize, v: usize) -> usize {
      let mut u = u;
      let mut v = v;
      if self.dep[u] < self.dep[v] {
        swap(&mut u, &mut v);
      }
      let k = self.dep[u] - self.dep[v];
      let lg = self.parent[0].len();
      for j in 0..lg {
        if (k >> j) & 1 == 1 {
          u = self.parent[u][j];
        }
      }
      if u == v {
        return u;
      }
      for j in (0..lg).rev() {
        if self.parent[u][j] != self.parent[v][j] {
          u = self.parent[u][j];
          v = self.parent[v][j];
        }
      }
      self.parent[u][0]
    }

    pub fn distance(&self, u: usize, v: usize) -> usize {
      self.dep[u] + self.dep[v] - 2 * self.dep[self.lca(u, v)]
    }

    pub fn is_path_on(&self, u: usize, v: usize, a: usize) -> bool {
      self.distance(u, v) == self.distance(u, a) + self.distance(a, v)
    }
  }
}
