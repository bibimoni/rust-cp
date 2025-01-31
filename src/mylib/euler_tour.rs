use euler_tour::*;
pub mod euler_tour {
  #[derive(Clone, Copy, Debug, PartialEq, Eq)]
  pub enum InOut {
    In(usize),
    Out(usize),
  }

  pub struct EulerTour {
    pub tour: Vec<InOut>,
    pub start: Vec<usize>,
    pub end: Vec<usize>,
  }

  impl EulerTour {
    // one-indexed
    pub fn new(root: usize, adj: &Vec<Vec<usize>>) -> Self {
      let n: usize = adj.len() - 1;

      let tour = {
        fn dfs(tour: &mut Vec<InOut>, u: usize, p: usize, adj: &Vec<Vec<usize>>) {
          tour.push(InOut::In(u));

          for &v in &adj[u] {
            if p == v {
              continue;
            }
            dfs(tour, v, u, adj);
          }

          tour.push(InOut::Out(u));
        }
        let mut tour: Vec<InOut> = vec![];
        dfs(&mut tour, root, root, adj);
        tour
      };

      let (start, end) = {
        let mut start = vec![0; n + 1];
        let mut end = vec![0; n + 1];
        for (time, current) in tour.iter().copied().enumerate() {
          match current {
            InOut::In(u) => start[u] = time,
            InOut::Out(u) => end[u] = time,
          }
        }
        (start, end)
      };

      Self { tour, start, end }
    }

    // is u ancestor of v
    pub fn is_ancestor_of(&self, u: usize, v: usize) -> bool {
      self.start[u] <= self.start[v] && self.end[v] <= self.end[u]
    }
  }
}
