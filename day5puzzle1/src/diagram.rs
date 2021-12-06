use crate::lines::Line;

pub struct Diagram {
    grid: Vec<Vec<usize>>,
}

impl Diagram {
    pub fn new() -> Self {
        Diagram { grid: vec![vec![]] }
    }

    pub fn x(&self) -> usize {
        if self.grid.len() > 0 {
            self.grid[0].len()
        } else {
            0
        }
    }

    pub fn y(&self) -> usize {
        self.grid.len()
    }

    fn grow_x(&mut self, length: usize) {
        self.grid
            .iter_mut()
            .for_each(|row| row.resize_with(length, || 0));
    }

    fn grow_y(&mut self, length: usize) {
        let x = self.x();

        self.grid.resize_with(length, || vec![0; x]);
    }

    fn resize_grid(&mut self, line: &Line) {
        let ((x1, y1), (x2, y2)) = line.points();

        let biggest_x = x1.max(x2);
        let biggest_y = y1.max(y2);

        if biggest_x + 1 > self.x() {
            self.grow_x(biggest_x + 1)
        }
        if biggest_y + 1 > self.y() {
            self.grow_y(biggest_y + 1)
        }
    }

    fn add_horizontal_line(&mut self, line: &Line) {
        let ((x1, y1), (x2, y2)) = line.points();

        for x in x1..x2 {
            self.grid[y1][x] += 1;
        }
    }

    fn add_vertical_line(&mut self, line: &Line) {
        let ((x1, y1), (x2, y2)) = line.points();

        (y1..y2).for_each(|y| {
            println!("Incrementing ({}, {})", x1, y);
            println!("(Currently {})", self.grid[y][x1]);
            self.grid[y][x1] += 1;
        })
    }

    pub fn add_line(&mut self, line: &Line) {
        let ((x1, y1), (x2, y2)) = line.points();

        println!("Adding ({}, {}), ({}, {})", x1, y1, x2, y2);

        self.resize_grid(&line);

        if x1 == x2 {
            self.add_vertical_line(&line);
        } else if y1 == y2 {
            self.add_horizontal_line(&line);
        }
    }

    pub fn draw(&self) {
        self.grid.iter().for_each(|row| {
            println!(
                "{}",
                row.iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(" | ")
            );
        })
    }
}
