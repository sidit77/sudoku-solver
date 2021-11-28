use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Range};
use std::str::FromStr;
use console_engine::{Color, ConsoleEngine, KeyCode};
use console_engine::crossterm::event::KeyEvent;
use console_engine::events::Event;
use console_engine::pixel::Pixel;

fn main() {
    //let mut lines: String = String::new();
    //BufReader::new(File::open("sudoku2.txt").expect("cannot open file"))
    //    .read_to_string(&mut lines).expect("error reading file");
//
    //let mut sudoku = lines.parse::<Sudoku>().expect("error parsing");
    //println!("Trying to solved to following sudoku:\n{}", sudoku);
    //let time = Instant::now();
    //let sudoku = sudoku.solve();
    //let time = Instant::now() - time;
    //match sudoku {
    //    None => println!("No solution found!"),
    //    Some(solved) => println!("Found solution ({}s):\n{}", time.as_secs_f64(), solved)
    //}

    // initializes the engine
    let mut engine = console_engine::ConsoleEngine::init(25, 13, 1).unwrap();

    let mut selected_cell = (0,0);
    let mut sudoku = Sudoku::empty();
    let mut sudoku_solved = sudoku.solve();

    draw(&mut engine, &sudoku, &sudoku_solved, selected_cell);
    loop {
        // Poll next event
        match engine.poll() {

            Event::Key(keyevent) => {
                let mut set = | x, y, v | {
                    sudoku.set(x, y, v);
                    sudoku_solved = sudoku.solve();
                };

                match keyevent {
                    KeyEvent { code: KeyCode::Char('q'), .. } => break,
                    KeyEvent { code: KeyCode::Char('n'), .. } => sudoku = Sudoku::empty(),
                    KeyEvent { code: KeyCode::Left, .. } =>
                        selected_cell.0 = (selected_cell.0 - 1).clamp(0, 8),
                    KeyEvent { code: KeyCode::Right, .. } =>
                        selected_cell.0 = (selected_cell.0 + 1).clamp(0, 8),
                    KeyEvent { code: KeyCode::Up, .. } =>
                        selected_cell.1 = (selected_cell.1 - 1).clamp(0, 8),
                    KeyEvent { code: KeyCode::Down, .. } =>
                        selected_cell.1 = (selected_cell.1 + 1).clamp(0, 8),
                    KeyEvent { code: KeyCode::Char('1'), .. } => set(selected_cell.0, selected_cell.1, Some(0)),
                    KeyEvent { code: KeyCode::Char('2'), .. } => set(selected_cell.0, selected_cell.1, Some(1)),
                    KeyEvent { code: KeyCode::Char('3'), .. } => set(selected_cell.0, selected_cell.1, Some(2)),
                    KeyEvent { code: KeyCode::Char('4'), .. } => set(selected_cell.0, selected_cell.1, Some(3)),
                    KeyEvent { code: KeyCode::Char('5'), .. } => set(selected_cell.0, selected_cell.1, Some(4)),
                    KeyEvent { code: KeyCode::Char('6'), .. } => set(selected_cell.0, selected_cell.1, Some(5)),
                    KeyEvent { code: KeyCode::Char('7'), .. } => set(selected_cell.0, selected_cell.1, Some(6)),
                    KeyEvent { code: KeyCode::Char('8'), .. } => set(selected_cell.0, selected_cell.1, Some(7)),
                    KeyEvent { code: KeyCode::Char('9'), .. } => set(selected_cell.0, selected_cell.1, Some(8)),
                    KeyEvent { code: KeyCode::Char(' '), .. } => set(selected_cell.0, selected_cell.1, None),
                    _ => {}
                }

                draw(&mut engine, &sudoku, &sudoku_solved, selected_cell);
            }
            _ => {}
        }
    }
}

fn cell_to_pxl(x: i32, y: i32) -> (i32, i32) {
    let x = 2 * (1 + x + (x / 3));
    let y = 1 + y + (y / 3);
    (x,y)
}

fn draw(engine: &mut ConsoleEngine, sudoku: &Sudoku, solved: &Option<Sudoku>, selected: (usize, usize)) {
    engine.clear_screen();
    engine.print_fbg(0, 0, "
┌───────┬───────┬───────┐
│ _ _ _ │ _ _ _ │ _ _ _ │
│ _ _ _ │ _ _ _ │ _ _ _ │
│ _ _ _ │ _ _ _ │ _ _ _ │
├───────┼───────┼───────┤
│ _ _ _ │ _ _ _ │ _ _ _ │
│ _ _ _ │ _ _ _ │ _ _ _ │
│ _ _ _ │ _ _ _ │ _ _ _ │
├───────┼───────┼───────┤
│ _ _ _ │ _ _ _ │ _ _ _ │
│ _ _ _ │ _ _ _ │ _ _ _ │
│ _ _ _ │ _ _ _ │ _ _ _ │
└───────┴───────┴───────┘
    ".trim(), Color::White, Color::Black);

    for x in 0..Sudoku::size() {
        for y in 0..Sudoku::size() {
            let (px, py) = cell_to_pxl(x as i32, y as i32);

            let mut pxl = match sudoku.get(x, y) {
                None => match solved.as_ref().map(|s|*s.get(x, y)).flatten() {
                    None => Pixel {
                        bg: Color::Black,
                        fg: Color::White,
                        chr: ' '
                    },
                    Some(i) => Pixel {
                        bg: Color::Black,
                        fg: Color::DarkGrey,
                        chr: char::from_digit(i as u32 + 1, 10).unwrap()
                    }
                },
                Some(i) => Pixel {
                    bg: Color::Black,
                    fg: Color::White,
                    chr: char::from_digit(*i as u32 + 1, 10).unwrap()
                }
            };
            if x == selected.0 && y == selected.1 {
                let tmp = pxl.fg;
                pxl.fg = pxl.bg;
                pxl.bg = tmp;
            }

            engine.set_pxl(px, py, pxl);
        }
    }

    engine.draw();
}

#[derive(Debug, Clone)]
pub struct SudokuField<T> {
    elements: [T; SudokuField::<()>::size() * SudokuField::<()>::size()]
}

impl<T> SudokuField<T>{

    pub const fn values() -> Range<u8> {
        0u8..(Self::size() as u8)
    }

    pub const fn size() -> usize {
        Self::cell_size() * Self::cell_size()
    }

    pub const fn cell_size() -> usize {
        3
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        debug_assert!(x < Self::size());
        debug_assert!(y < Self::size());
        &self.elements[y * Self::size() + x]
    }

    pub fn set(&mut self, x: usize, y: usize, v: T) {
        debug_assert!(x < Self::size());
        debug_assert!(y < Self::size());
        self.elements[y * Self::size() + x] = v;
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
            elements: [full_set; Self::size() * Self::size()]
        }
    }

    fn row(x: usize, y: usize) -> impl Iterator<Item=(usize,usize)> {
        (0..Self::size()).filter(move |i| *i != x).map(move |x| (x, y))
    }

    fn column(x: usize, y: usize) -> impl Iterator<Item=(usize,usize)> {
        (0..Self::size()).filter(move |i| *i != y).map(move |y| (x, y))
    }

    fn cell(x: usize, y: usize) -> impl Iterator<Item=(usize,usize)> {
        let cell_x = (x / Self::cell_size()) * Self::cell_size();
        let cell_y = (y / Self::cell_size()) * Self::cell_size();
        (0..Self::cell_size())
            .flat_map(move |cx|(0..Self::cell_size())
                .map(move |cy|(cell_x + cx, cell_y + cy)))
            .filter(move |(cx, cy)| *cx != x || *cy != y)
    }

    fn set_constraint(&mut self, x: usize, y: usize, v: u8) {
        debug_assert!(Self::values().contains(&v));
        self.set(x, y, ValueSet::singleton(v));
        self.propagate(x, y, v);
    }

    fn propagate(&mut self, x: usize, y: usize, v: u8){
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
        let old = *self.get(x, y);
        let new = old.remove(v);
        if new != old {
            self.set(x, y, new);
            if let Some(v) = new.iter().single() {
                self.propagate(x, y, v);
            }
        }
    }

    fn is_valid(&self) -> bool {
        !self.elements.iter().any(|elem| elem.is_empty())
    }

    fn lowest_entropy_field(&self) -> Option<(usize, usize)> {
        self.elements
            .iter()
            .enumerate()
            .map(|(i, v)|(i, v.len()))
            .filter(|(_, v)| *v > 1)
            .reduce(|t1, t2 | if t2.1 < t1.1 { t2 } else { t1 })
            .map(|(i, _) | (i % Self::size(), i / Self::size()))
    }

    fn solve(self) -> Option<SudokuSolver>{
        match self.is_valid() {
            true => match self.lowest_entropy_field() {
                None => {
                    debug_assert!(!self.elements.iter().any(|elem| elem.len() > 1));
                    Some(self)
                },
                Some((x, y)) => self
                    .get(x, y)
                    .iter()
                    .filter_map(|v|{
                        let mut step = self.clone();
                        step.set_constraint(x, y, v);
                        step.solve()
                    })
                    .next()
            }
            false => None
        }
    }

}

pub type Sudoku = SudokuField<Option<u8>>;

impl Sudoku {
    pub fn empty() -> Self {
        Self {
            elements: [None; Self::size() * Self::size()]
        }
    }

    pub fn solve(&self) -> Option<Sudoku>{
        SudokuSolver::from(self).solve().map(|result|result.into())
    }
}

impl From<&Sudoku> for SudokuSolver {
    fn from(sudoku: &Sudoku) -> Self {
        let mut solver = SudokuSolver::empty();
        for y in 0..Self::size() {
            for x in 0..Self::size() {
                if let Some(v) = *sudoku.get(x, y) {
                    solver.set_constraint(x,y,v);
                }
            }
        }
        solver
    }
}

impl From<SudokuSolver> for Sudoku {
    fn from(solver: SudokuSolver) -> Self {
        let mut sudoku = Self::empty();
        for y in 0..Self::size() {
            for x in 0..Self::size() {
                sudoku.set(x, y, solver.get(x, y).iter().single());
            }
        }
        sudoku
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SudokuParseError {
    TooFewValues,
    TooManyValues
}

impl Display for SudokuParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SudokuParseError::TooFewValues =>
                write!(f, "Got fewer than {} valid characters ('_', 1-9)", Sudoku::size() * Sudoku::size()),
            SudokuParseError::TooManyValues =>
                write!(f, "Got more than {} valid characters ('_', 1-9)", Sudoku::size() * Sudoku::size()),
        }
    }
}

impl Error for SudokuParseError {}

impl FromStr for Sudoku {
    type Err = SudokuParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut result = Sudoku::empty();

        let mut chars = str.chars().filter_map(|c|match c {
           '_' => Some(None),
            _ => match c.to_digit(10).unwrap_or(0) {
                0 => None,
                v => Some(Some((v as u8) - 1))
            }
        });

        for y in 0..Self::size() {
            for x in 0..Self::size() {
                match chars.next() {
                    None => Err(SudokuParseError::TooFewValues)?,
                    Some(v) => result.set(x, y, v)
                }
            }
        }

        match chars.next() {
            None => Ok(result),
            Some(_) => Err(SudokuParseError::TooManyValues)
        }
    }
}

impl Display for Sudoku {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        const TEMPLATE: &str = "
┌───────┬───────┬───────┐
│ _ _ _ │ _ _ _ │ _ _ _ │
│ _ _ _ │ _ _ _ │ _ _ _ │
│ _ _ _ │ _ _ _ │ _ _ _ │
├───────┼───────┼───────┤
│ _ _ _ │ _ _ _ │ _ _ _ │
│ _ _ _ │ _ _ _ │ _ _ _ │
│ _ _ _ │ _ _ _ │ _ _ _ │
├───────┼───────┼───────┤
│ _ _ _ │ _ _ _ │ _ _ _ │
│ _ _ _ │ _ _ _ │ _ _ _ │
│ _ _ _ │ _ _ _ │ _ _ _ │
└───────┴───────┴───────┘
        ";

        let mut elements = self.elements.iter().map(|v| match v {
            None => ' ',
            Some(v) => char::from_digit(*v as u32 + 1, 10).unwrap()
        });

        TEMPLATE.trim().chars().map(|c| match c {
            '_' => elements.next().expect("wrong format string"),
            _ => c
        }).map(|c| write!(f, "{}", c)).collect()
    }
}

pub trait Single: Iterator {
    fn single(self) -> Option<Self::Item>;
}

impl<I: Iterator> Single for I {
    fn single(mut self) -> Option<Self::Item> {
        match self.next() {
            None => None,
            Some(element) => match self.next() {
                None => Some(element),
                Some(_) => None,
            },
        }
    }
}
