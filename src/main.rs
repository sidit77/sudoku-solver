use std::fs::File;
use std::io;
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader};
use std::num::NonZeroU8;
use std::str::FromStr;

fn main() -> anyhow::Result<()>{
    let file = File::open("sudoku.txt")?;
    let lines: String = BufReader::new(file)
        .lines()
        .collect::<io::Result<Vec<String>>>()?
        .join("\n");

    let sudoku = lines.parse::<Sudoku>()?;
    println!("{}", sudoku);

    Ok(())
}

const SIZE: usize = 3;

#[derive(Debug)]
struct Sudoku {
    fields: [[Option<NonZeroU8>; SIZE * SIZE]; SIZE * SIZE]
}

impl FromStr for Sudoku {
    type Err = Infallible;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Ok(Sudoku{ fields: str
            .lines()
            .map(|line| line
                .split(' ')
                .map(|str| match str {
                    "_" => None,
                    str => Some(str.parse::<NonZeroU8>().unwrap())
                })
                .collect::<Vec<_>>()
                .try_into().unwrap())
            .collect::<Vec<_>>()
            .try_into().unwrap() })
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y1 in 0..SIZE {
            for y2 in 0..SIZE {
                for x1 in 0..SIZE {
                    for x2 in 0..SIZE {
                        match self.fields[y1 * 3 + y2][x1 * 3 + x2] {
                            None => write!(f, " "),
                            Some(v) => write!(f, "{}", v)
                        }?;
                        if x2 < SIZE - 1 {
                            write!(f, " ")?;
                        }
                    }
                    if x1 < SIZE - 1 {
                        write!(f, " | ")?;
                    }

                }
                writeln!(f, "")?;
            }
            if y1 < SIZE - 1 {
                for y2 in 0..SIZE {
                    for y3 in 0..SIZE {
                        write!(f, "-")?;
                        if y3 < SIZE - 1 {
                            write!(f, "-")?;
                        }
                    }
                    if y2 < SIZE - 1 {
                        write!(f, "-+-")?;
                    }
                }
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}

impl Sudoku {

}