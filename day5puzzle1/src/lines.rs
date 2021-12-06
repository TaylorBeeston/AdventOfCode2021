pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl Line {
    pub fn new(p1: (usize, usize), p2: (usize, usize)) -> Self {
        let ((x1, y1), (x2, y2)) = (p1, p2);

        Line {
            p1: Point::new(x1, y1),
            p2: Point::new(x2, y2),
        }
    }

    pub fn new_from_input(input: &str) -> Self {
        let (p1, p2) = input.split_once(" -> ").unwrap();

        let ((x1, y1), (x2, y2)) = (p1.split_once(",").unwrap(), p2.split_once(",").unwrap());

        Line {
            p1: Point::new(x1.parse().unwrap(), y1.parse().unwrap()),
            p2: Point::new(x2.parse().unwrap(), y2.parse().unwrap()),
        }
    }

    pub fn points(&self) -> ((usize, usize), (usize, usize)) {
        let Line { p1, p2 } = self;
        ((p1.x, p1.y), (p2.x, p2.y))
    }
}
