use super::modular::ModularAdd;

#[derive(Debug, Clone)]
pub struct Bijection {
    pub f: Vec<usize>,
    pub g: Vec<usize>,
}

impl Bijection {
    pub fn new(len: usize) -> Self {
        Self {
            f: (0..len).collect::<Vec<_>>(),
            g: (0..len).collect::<Vec<_>>(),
        }
    }

    pub fn len(&self) -> usize {
        self.f.len()
    }

    pub fn valid(&self) -> bool {
        (0..self.len()).all(|i| self.g[self.f[i]] == i)
            && (0..self.len()).all(|j| self.f[self.g[j]] == j)
    }

    pub fn swap(&mut self, a_i: usize, b_i: usize) {
        let a_j = self.f[a_i];
        let b_j = self.f[b_i];
        self.f.swap(a_i, b_i);
        self.g.swap(a_j, b_j);

        // debug_assert!(self.valid());
    }

    pub fn swap_adj(&mut self, a_i: usize, offset: usize) {
        let a_j = self.f[a_i];
        let b_j = a_j.add_n(offset, self.len());
        let b_i = self.g[b_j];
        self.swap(a_i, b_i);
    }

    pub fn swap_with_right(&mut self, a_i: usize) {
        self.swap_adj(a_i, 1);
    }

    pub fn swap_with_left(&mut self, a_i: usize) {
        self.swap_adj(a_i, self.len() - 1);
    }
}
