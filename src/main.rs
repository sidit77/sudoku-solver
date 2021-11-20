use std::fs::File;
use std::io;
use std::convert::Infallible;
use std::fmt::{Debug, Display, Formatter};
use std::io::{BufRead, BufReader};
use std::ops::{Range};
use std::str::FromStr;

fn main() -> anyhow::Result<()>{
    let file = File::open("sudoku2.txt")?;
    let lines: String = BufReader::new(file)
        .lines()
        .collect::<io::Result<Vec<String>>>()?
        .join("\n");

    let sudoku = lines.parse::<Sudoku>()?;
    println!("{}", sudoku);
    let solver = Into::<SudokuSolver>::into(sudoku);
    println!("{}", solver);
    println!("{}", Into::<Sudoku>::into(solver));

    Ok(())
}

#[derive(Debug)]
struct SudokuField<T> {
    rows: [[T; SudokuField::<()>::size()]; SudokuField::<()>::size()]
}

impl<T> SudokuField<T>{

    const fn values() -> Range<u8> {
        0u8..(Self::size() as u8)
    }

    const fn size() -> usize {
        Self::cell_size() * Self::cell_size()
    }

    const fn cell_size() -> usize {
        3
    }

    fn row(x: usize, y: usize) -> impl Iterator<Item=(usize,usize)> {
        debug_assert!(x < Self::size());
        debug_assert!(y < Self::size());
        (0..Self::size()).filter(move |i| *i != x).map(move |x| (x, y))
    }

    fn column(x: usize, y: usize) -> impl Iterator<Item=(usize,usize)> {
        debug_assert!(x < Self::size());
        debug_assert!(y < Self::size());
        (0..Self::size()).filter(move |i| *i != y).map(move |y| (x, y))
    }

    fn cell(x: usize, y: usize) -> impl Iterator<Item=(usize,usize)> {
        debug_assert!(x < Self::size());
        debug_assert!(y < Self::size());
        let cell_x = (x / Self::cell_size()) * Self::cell_size();
        let cell_y = (y / Self::cell_size()) * Self::cell_size();
        (0..Self::cell_size())
            .flat_map(move |cx|(0..Self::cell_size())
                .map(move |cy|(cell_x + cx, cell_y + cy)))
            .filter(move |(cx, cy)| *cx != x || *cy != y)
    }

    fn get(&self, x: usize, y: usize) -> &T {
        debug_assert!(x < Self::size());
        debug_assert!(y < Self::size());
        &self.rows[y][x]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        debug_assert!(x < Self::size());
        debug_assert!(y < Self::size());
        &mut self.rows[y][x]
    }



}


type Sudoku = SudokuField<Option<u8>>;

impl FromStr for Sudoku {
    type Err = Infallible;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Ok(Sudoku{ rows: str
            .lines()
            .map(|line| line
                .split(' ')
                .map(|str| match str {
                    "_" => None,
                    str => Some(str.parse::<u8>().unwrap())
                })
                .collect::<Vec<_>>()
                .try_into().unwrap())
            .collect::<Vec<_>>()
            .try_into().unwrap() })
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y1 in 0..Self::cell_size() {
            for y2 in 0..Self::cell_size() {
                for x1 in 0..Self::cell_size() {
                    for x2 in 0..Self::cell_size() {
                        match self.get(x1 * 3 + x2, y1 * 3 + y2) {
                            None => write!(f, " "),
                            Some(v) => write!(f, "{}", v)
                        }?;
                        if x2 < Self::cell_size() - 1 {
                            write!(f, " ")?;
                        }
                    }
                    if x1 < Self::cell_size() - 1 {
                        write!(f, " | ")?;
                    }

                }
                writeln!(f, "")?;
            }
            if y1 < Self::cell_size() - 1 {
                for y2 in 0..Self::cell_size() {
                    for y3 in 0..Self::cell_size() {
                        write!(f, "-")?;
                        if y3 < Self::cell_size() - 1 {
                            write!(f, "-")?;
                        }
                    }
                    if y2 < Self::cell_size() - 1 {
                        write!(f, "-+-")?;
                    }
                }
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}

type ValueSet = smallbitset::Set16;
type SudokuSolver = SudokuField<ValueSet>;

impl SudokuSolver {
    fn empty() -> Self {
        let full_set = Self::values()
            .map(|v| ValueSet::singleton(v))
            .fold(ValueSet::empty(), ValueSet::union);
        Self {
            rows: [[full_set; Self::size()]; Self::size()]
        }
    }

    fn set(&mut self, x: usize, y: usize, v: u8) {
        debug_assert!(Self::values().contains(&v));
        debug_assert!(self.get(x, y).contains(v));
        *self.get_mut(x, y) = ValueSet::singleton(v);
        self.propagate(x, y);
        debug_assert!(!self.get(x,y).is_empty(), "{}, {} is empty", x, y)
    }

    fn propagate(&mut self, x: usize, y: usize){
        debug_assert!(self.get(x, y).len() == 1);
        let v = self.get(x, y).iter().nth(0).unwrap();
        for (x, y) in Self::row(x,y) {
            self.remove(x, y, v);
        }
        for (x, y) in Self::column(x,y) {
            self.remove(x, y, v);
        }
        for (x, y) in Self::cell(x,y) {
            self.remove(x, y, v);
        }
    }

    fn remove(&mut self, x: usize, y: usize, v: u8){
        debug_assert!(Self::values().contains(&v));
        let l = self.get(x, y).len();
        *self.get_mut(x, y) = self.get(x, y).remove(v);
        if self.get(x, y).len() == 1 && l > self.get(x, y).len() {
            self.propagate(x, y);
        }
        debug_assert!(!self.get(x,y).is_empty(), "{}, {} is empty", x, y)
    }

}

impl From<Sudoku> for SudokuSolver {
    fn from(sudoku: Sudoku) -> Self {
        let mut solver = SudokuSolver::empty();
        for x in 0..Self::size() {
            for y in 0..Self::size() {
                if let Some(v) = *sudoku.get(x, y) {
                    // println!("{}, {} = {}", x, y, v);
                    solver.set(x,y,v);
                    // println!("\n{}", solver)
                }
            }
        }
        solver
    }
}

impl From<SudokuSolver> for Sudoku {
    fn from(solver: SudokuSolver) -> Self {
        let mut sudoku = Self {
            rows: [[None; Self::size()]; Self::size()]
        };
        for x in 0..Self::size() {
            for y in 0..Self::size() {
                if solver.get(x, y).len() == 1 {
                    *sudoku.get_mut(x, y) = Some(solver.get(x, y).iter().nth(0).unwrap());
                }
            }
        }
        sudoku
    }
}

impl Display for SudokuSolver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y1 in 0..Self::cell_size() {
            for y2 in 0..Self::cell_size() {
                for x1 in 0..Self::cell_size() {
                    for x2 in 0..Self::cell_size() {
                        let cell = *self.get(x1 * 3 + x2, y1 * 3 + y2);
                        let list = Self::values()
                            .map(|v| if cell.contains(v) { format!("{}", v)} else {"_".to_string()})
                            .collect::<Vec<_>>()
                            .join(", ");
                        write!(f, "[{}]", list)?;
                        //write!(f, "{}", cell.len())?;
                        if x2 < Self::cell_size() - 1 {
                            write!(f, " ")?;
                        }
                    }
                    if x1 < Self::cell_size() - 1 {
                        write!(f, " | ")?;
                    }

                }
                writeln!(f, "")?;
            }
            if y1 < Self::cell_size() - 1 {
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}
