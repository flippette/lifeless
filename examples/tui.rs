use std::{
    error::Error,
    io::{self, Write},
};

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use lifeless::{Cell, Coord, Grid};

const CLEAR_ALL: Clear = Clear(ClearType::All);
const RESET_CUR: MoveTo = MoveTo(0, 0);

fn main() -> Result<(), Box<dyn Error>> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Clear(ClearType::All))?;

    let mut grid = Grid::<24, 16>::new();

    grid[Coord(1, 2)] = Cell::Alive;
    grid[Coord(2, 3)] = Cell::Alive;
    grid[Coord(3, 1)] = Cell::Alive;
    grid[Coord(3, 2)] = Cell::Alive;
    grid[Coord(3, 3)] = Cell::Alive;

    loop {
        draw_grid(&mut stdout, &grid)?;

        if let Event::Key(ev) = event::read()? {
            match ev.code {
                KeyCode::Esc | KeyCode::Char('q') => break,
                KeyCode::Enter | KeyCode::Char(' ') => {
                    grid = grid.step();
                }
                _ => {}
            }
        }
    }

    execute!(stdout, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

fn draw_grid<const W: usize, const H: usize>(
    writer: &mut impl Write,
    grid: &Grid<W, H>,
) -> Result<(), io::Error> {
    execute!(writer, CLEAR_ALL, RESET_CUR)?;

    for row in 0..H {
        for col in 0..W {
            write!(
                writer,
                "{}",
                match grid[Coord(col, row)] {
                    Cell::Alive => "██",
                    Cell::Dead => "░░",
                }
            )?;
        }
        writeln!(writer, "\r")?;
    }

    Ok(())
}
