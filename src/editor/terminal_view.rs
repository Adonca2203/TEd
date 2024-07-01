use crossterm::terminal::ClearType;

use super::terminal::{Position, Size, Terminal};
use std::io::Error;

mod buffer;

use buffer::Buffer;

// const NAME: &str = env!("CARGO_PKG_NAME");
// const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    buffer: Buffer,
    pub needs_render: bool,
}

impl View {
    pub fn render(&mut self) -> Result<(), Error> {
        if !self.needs_render {
            return Ok(());
        }

        let Size { height, width } = Terminal::size()?;
        Terminal::set_cursor_to(Position::zero())?;

        for current_row in 0..height {
            Terminal::clear(ClearType::CurrentLine)?;

            if let Some(line) = self.buffer.lines.get(current_row) {
                let mut copy = line.clone();
                copy.truncate(width);
                Terminal::print(&copy)?;
                Terminal::set_cursor_to(Position {
                    x: 0,
                    y: current_row,
                })?;
                continue;
            } else {
                Self::draw_empty_row()?;
            }

            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }

        self.set_needs_render(false);
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    pub fn load(&mut self, file_name: &str) -> Result<(), Error> {
        match Buffer::load_file(file_name) {
            Ok(buffer) => self.buffer = buffer,
            Err(err) => {
                return Err(err);
            }
        }
        Ok(())
    }

    pub fn set_needs_render(&mut self, val: bool) {
        self.needs_render = val;
    }
}
