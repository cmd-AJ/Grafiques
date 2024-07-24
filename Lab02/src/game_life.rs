pub struct GameOfLife {
    pub grid: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
    pub periods: Vec<(String, usize)>,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![false; width]; height];
        let periods = vec![];
        Self { grid, width, height, periods }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, state: bool) {
        if x < self.width && y < self.height {
            self.grid[y][x] = state;
        }
    }

    pub fn clear(&mut self) {
        for row in &mut self.grid {
            for cell in row {
                *cell = false;
            }
        }
    }

    pub fn update(&mut self) {
        let mut new_grid = self.grid.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let live_neighbors = self.live_neighbor_count(x, y);
                new_grid[y][x] = match (self.grid[y][x], live_neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }
        self.grid = new_grid;
    }

    pub fn live_neighbor_count(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dy in [self.height - 1, 0, 1].iter().cloned() {
            for dx in [self.width - 1, 0, 1].iter().cloned() {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if self.grid[(y + dy) % self.height][(x + dx) % self.width] {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn set_pattern(&mut self, pattern: &str, start_x: usize, start_y: usize) {
        let patterns = [
            ("glider", vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)], 1),
            ("block", vec![(0, 0), (0, 1), (1, 0), (1, 1)], 1),
            ("blinker", vec![(0, 0), (1, 0), (2, 0)], 2),
            ("toad", vec![(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)], 2),
            ("beacon", vec![(0, 0), (1, 0), (0, 1), (1, 1), (2, 2), (3, 2), (2, 3), (3, 3)], 2),
            ("lwss", vec![(1, 0), (4, 0), (0, 1), (0, 2), (4, 2), (0, 3), (1, 3), (2, 3), (3, 3)], 1),
            ("pulsar", vec![
                (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
                (0, 2), (5, 2), (7, 2), (12, 2),
                (0, 3), (5, 3), (7, 3), (12, 3),
                (0, 4), (5, 4), (7, 4), (12, 4),
                (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),

                (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
                (0, 8), (5, 8), (7, 8), (12, 8),
                (0, 9), (5, 9), (7, 9), (12, 9),
                (0, 10), (5, 10), (7, 10), (12, 10),
                (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12)
            ], 3),
            ("penta-decathlon", vec![
                (2, 0), (3, 0), (4, 0), (2, 1), (4, 1),
                (2, 2), (3, 2), (4, 2), (2, 4), (3, 4), (4, 4),
                (2, 5), (4, 5), (2, 6), (3, 6), (4, 6),
                (2, 8), (3, 8), (4, 8), (2, 9), (4, 9),
                (2, 10), (3, 10), (4, 10)
            ], 15),
            ("boat", vec![
                (0, 0), (1, 0), (2, 1), (0, 1), (1, 2)
            ], 1),
            ("tub", vec![
                (1, 0), (0, 1), (2, 1), (1, 2)
            ], 1),
            ("hwss", vec![
                (2, 0), (3, 0), (4, 0), (5, 0), (1, 1), (6, 1),
                (0, 2), (6, 2), (6, 3), (0, 3), (1, 4), (5, 4), (2, 4), (3, 4), (4, 4)
            ], 1),
            ("mwss", vec![
                (1, 0), (2, 0), (3, 0), (4, 0), (5, 1), (0, 1),
                (5, 2), (5, 3), (4, 3), (0, 3), (1, 4), (4, 4), (3, 4), (2, 4)
            ], 1),
            ("beehive", vec![
                (1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)
            ], 1),
            ("puff", vec![
                (0, 1), (1, 1), (2, 1), (4, 1), (5, 1), (6, 1),
                (1, 2), (4, 2), (6, 2),
                (1, 3), (4, 3), (6, 3),
                (2, 4), (3, 4), (4, 4), (5, 4),
                (2, 5), (5, 5),
                (2, 6),
                (1, 7), (2, 7), (3, 7), (4, 7), (5, 7),
                (3, 8)
            ], 1)
        ];

        if let Some(&(name, ref cells, period)) = patterns.iter().find(|&&(name, _, _)| name == pattern) {
            for &(dx, dy) in cells {
                self.set_cell(start_x + dx, start_y + dy, true);
            }
            self.periods.push((name.to_string(), period));
        }
    }

    
}
