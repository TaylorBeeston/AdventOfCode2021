struct BoardNumber {
    pub value: usize,
    pub checked: bool,
}

impl BoardNumber {
    pub fn new(value: usize, checked: bool) -> Self {
        BoardNumber { value, checked }
    }
}

pub struct Board {
    values: Vec<Vec<BoardNumber>>,
}

impl Board {
    pub fn new(values: &str) -> Self {
        let board_values: Vec<Vec<BoardNumber>> = values
            .lines()
            .map(|line| {
                line.split(" ")
                    .map(|value| BoardNumber::new(value.parse::<usize>().unwrap(), false))
                    .collect()
            })
            .collect();

        Board {
            values: board_values,
        }
    }

    pub fn check_num(&self, number: usize) -> Self {
        let new_values: Vec<Vec<BoardNumber>> = self
            .values
            .iter()
            .map(|row| {
                row.iter()
                    .map(|column| {
                        if column.value == number {
                            BoardNumber::new(column.value, true)
                        } else {
                            BoardNumber::new(column.value, column.checked)
                        }
                    })
                    .collect()
            })
            .collect();

        Board { values: new_values }
    }

    pub fn check_won(&self) -> bool {
        if self
            .values
            .iter()
            .any(|row| row.iter().all(|column| column.checked))
        {
            return true;
        }
        if self.values[0]
            .iter()
            .enumerate()
            .any(|(index, _column)| self.values.iter().all(|row| row[index].checked))
        {
            return true;
        }

        false
    }

    pub fn unmarked_sum(&self) -> usize {
        self.values
            .iter()
            .map(|row| {
                row.iter()
                    .map(|column| if !column.checked { column.value } else { 0 })
                    .sum::<usize>()
            })
            .sum()
    }

    pub fn print(&self) {
        println!("---------");

        self.values.iter().for_each(|row| {
            let line: String = row
                .iter()
                .map(|column| {
                    let checked = if column.checked { "✅" } else { "" };
                    format!("{} {}", column.value, checked)
                })
                .collect();
            println!("{}", line);
        });

        println!("---------");
    }
}
