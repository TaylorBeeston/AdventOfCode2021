mod diagram;
mod input;
mod lines;

use crate::diagram::Diagram;
use crate::input::get_input;

fn main() {
    let input = get_input();

    let mut diagram = Diagram::new();

    input.iter().for_each(|line| diagram.add_line(line));

    diagram.draw();

    println!("Hello, world!");
}
