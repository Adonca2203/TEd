use crossterm::cursor::{self, Hide, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp, Show};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{queue, Command};
use std::io::{stdout, Error, Write};

pub struct Terminal {}

#[derive(Clone, Copy)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub const fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl Terminal {
    pub fn terminate() -> Result<(), std::io::Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear(ClearType::All)?;
        Self::set_cursor_to(Position::zero())?;
        Self::execute()?;
        Ok(())
    }

    pub fn clear(clear_type: ClearType) -> Result<(), Error> {
        Self::queue_command(Clear(clear_type))
    }

    pub fn set_cursor_to(pos: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(pos.x, pos.y))
    }

    pub fn move_curstor_to(dir: Direction) -> Result<(), Error> {
        match dir {
            Direction::LEFT => Self::queue_command(MoveLeft(1)),
            Direction::RIGHT => Self::queue_command(MoveRight(1)),
            Direction::UP => Self::queue_command(MoveUp(1)),
            Direction::DOWN => Self::queue_command(MoveDown(1)),
        }
    }

    pub fn get_cursor_position() -> Position {
        Position{
            x: cursor::position().unwrap().0,
            y: cursor::position().unwrap().1,
        }
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size { height, width })
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)
    }

    pub fn print(msg: &str) -> Result<(), Error> {
        Self::queue_command(Print(msg))
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
}
