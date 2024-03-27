use rand::prelude::*;
use std::iter;

pub struct Pattern {
    pub size: usize,
    order: Vec<u8>,
    free_space: usize,
}

impl Pattern {
    pub fn new(size: usize) -> Pattern {
        let sq_size = size * size;

        Pattern {
            order: (1..=(sq_size as u8)).collect(),
            size,
            free_space: sq_size - 1,
        }
    }

    pub fn get_last_tile(&self) -> usize {
        self.size * self.size
    }

    pub fn shuffle(&mut self, steps: usize) {
        let mut rng = rand::thread_rng();
        let random_u8 = iter::repeat_with(|| rng.gen::<u8>());

        for r in random_u8.take(steps) {
            let s = r % 4;
            if s == 0 {
                self.up();
            } else if s == 1 {
                self.down();
            } else if s == 2 {
                self.right();
            } else {
                self.left();
            }
        }
    }

    pub fn get_row(&self, row: usize) -> Option<&[u8]> {
        if row >= self.size {
            return None;
        }

        let start = row * self.size;
        let end = start + self.size;

        Some(&self.order[start..end])
    }

    pub fn is_in_order(&self) -> bool {
        self.order
            .iter()
            .enumerate()
            .all(|(i, &value)| i + 1 == value.into())
    }

    pub fn up(&mut self) {
        if self.free_space / self.size > 0 {
            self.order
                .swap(self.free_space, self.free_space - self.size);
            self.free_space -= self.size;
        }
    }

    pub fn down(&mut self) {
        if self.free_space / self.size < self.size - 1 {
            self.order
                .swap(self.free_space, self.free_space + self.size);
            self.free_space += self.size;
        }
    }

    pub fn left(&mut self) {
        if self.free_space % self.size > 0 {
            self.order.swap(self.free_space, self.free_space - 1);
            self.free_space -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.free_space % self.size < self.size - 1 {
            self.order.swap(self.free_space, self.free_space + 1);
            self.free_space += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_patter_in_order() {
        let mut p = Pattern::new(4);
        println!("{:#?}", p.order);
        assert!(p.is_in_order());
        p.up();
        assert!(!p.is_in_order());
    }

    #[test]
    fn get_row() {
        let p = Pattern::new(5);
        let row = p.get_row(2);

        assert_eq!(row, Some(&[11, 12, 13, 14, 15][..]));
    }

    #[test]
    fn shuffle() {
        let mut p = Pattern::new(4);
        p.shuffle(25);
        assert!(!p.is_in_order());
    }

    #[test]
    fn up() {
        let mut p = Pattern::new(4);
        assert_eq!(p.order[15], 16);
        p.up();
        assert_eq!(p.order[11], 16);
        assert_eq!(p.order[15], 12);
        p.up();
        assert_eq!(p.order[7], 16);
        p.up();
        p.up();
        p.up();
        p.up();
        p.up();
        assert_eq!(p.order[3], 16);
    }

    #[test]
    fn down() {
        let mut p = Pattern::new(4);
        p.up();
        p.up();
        p.down();
        assert_eq!(p.order[11], 16);
        p.down();
        assert!(p.is_in_order());
    }

    #[test]
    fn left() {
        let mut p = Pattern::new(4);
        assert_eq!(p.order[15], 16);
        p.left();
        assert_eq!(p.order[14], 16);
        assert_eq!(p.order[15], 15);
        p.left();
        assert_eq!(p.order[13], 16);
        p.left();
        p.left();
        p.left();
        p.left();
        p.left();
        assert_eq!(p.order[12], 16);
    }

    #[test]
    fn right() {
        let mut p = Pattern::new(4);
        p.left();
        p.left();
        p.right();
        assert_eq!(p.order[14], 16);
        p.right();
        assert!(p.is_in_order());
    }
}
