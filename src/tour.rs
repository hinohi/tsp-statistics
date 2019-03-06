use rand::{seq::SliceRandom, Rng};

use crate::town::TownDistance;
use crate::utils::order_ab;

pub struct Tour {
    town: TownDistance,
    path: Vec<usize>,
}

impl Tour {
    pub fn new(town: TownDistance, path: Vec<usize>) -> Tour {
        assert_eq!(town.len(), path.len());
        let mut visited = vec![false; path.len()];
        for &p in path.iter() {
            assert!(!visited[p]);
            visited[p] = true;
        }
        Tour { town, path }
    }
    pub fn with_random<R: Rng>(town: TownDistance, r: &mut R) -> Tour {
        assert!(town.len() >= 1);
        let mut path = (0..town.len()).collect::<Vec<_>>();
        path.shuffle(r);
        Self::new(town, path)
    }

    pub fn get_total_dist(&self) -> f64 {
        let mut total_dist = 0.0;
        for i in 1..self.path.len() {
            total_dist += self.town.dist(self.path[i - 1], self.path[i]);
        }
        total_dist += self.town.dist(self.path[self.path.len() - 1], self.path[0]);
        total_dist
    }

    pub fn get_path(&self) -> Vec<usize> {
        self.path.clone()
    }

    pub fn try_2opt(&self, a: usize, b: usize) -> f64 {
        if a == b {
            return 0.0;
        }
        let n = self.path.len();
        let before = self.town.dist(self.path[a], self.path[(a + 1) % n])
            + self.town.dist(self.path[b], self.path[(b + 1) % n]);
        let after = self.town.dist(self.path[a], self.path[b])
            + self
                .town
                .dist(self.path[(a + 1) % n], self.path[(b + 1) % n]);
        after - before
    }

    pub fn do_2opt(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }
        let n = self.path.len();
        let (mut a, mut b) = order_ab(a, b);
        if (b - a) * 2 <= n {
            a += 1;
            while a < b {
                self.path.swap(a, b);
                a += 1;
                b -= 1;
            }
        } else {
            a += n;
            b += 1;
            while b < a {
                self.path.swap(a % n, b % n);
                a -= 1;
                b += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::town::{DistType, TownDistance};

    #[test]
    fn test_2opt() {
        let town_pos = [[0.0], [1.0], [2.0], [3.0], [4.0]];
        let town = TownDistance::new(&town_pos, DistType::L2);
        let mut tour = Tour::new(town, vec![0, 1, 2, 3, 4]);
        assert_eq!(tour.get_total_dist(), 8.0);
        assert_eq!(tour.try_2opt(0, 1), 0.0);
        assert_eq!(tour.try_2opt(0, 2), 2.0);
        assert_eq!(tour.try_2opt(0, 3), 4.0);
        assert_eq!(tour.try_2opt(2, 0), 2.0);
        assert_eq!(tour.try_2opt(0, 4), 0.0);
        assert_eq!(tour.try_2opt(1, 4), 0.0);
        tour.do_2opt(0, 2);
        assert_eq!(tour.get_path(), vec![0, 2, 1, 3, 4]);
        assert_eq!(tour.get_total_dist(), 10.0);
        tour.do_2opt(4, 1);
        assert_eq!(tour.get_path(), vec![2, 0, 1, 3, 4]);
    }
}
