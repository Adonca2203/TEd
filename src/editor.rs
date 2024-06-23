use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use std::io::Error;

mod terminal;
use crossterm::terminal::ClearType;
use terminal::{Position, Size, Terminal};

pub struct TerminalEditor {
    should_quit: bool,
    cursor_position: Position,
}

impl TerminalEditor {
    pub const fn default() -> Self {
        Self {
            should_quit: false,
            cursor_position: Position::zero(),
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }

            let event = read()?;
            self.evalutate_event(&event);
        }
        Ok(())
    }

    fn evalutate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.cursor_position = Position::zero();
                    self.should_quit = true;
                }
                KeyCode::Right => {
                    Terminal::move_curstor_to(terminal::Direction::RIGHT).unwrap();
                    self.cursor_position = Terminal::get_cursor_position();
                }
                KeyCode::Left => {
                    Terminal::move_curstor_to(terminal::Direction::LEFT).unwrap();
                    self.cursor_position = Terminal::get_cursor_position();
                }
                KeyCode::Up => {
                    Terminal::move_curstor_to(terminal::Direction::UP).unwrap();
                    self.cursor_position = Terminal::get_cursor_position();
                }
                KeyCode::Down => {
                    Terminal::move_curstor_to(terminal::Direction::DOWN).unwrap();
                    self.cursor_position = Terminal::get_cursor_position();
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear(ClearType::All)?;
            Terminal::set_cursor_to(self.cursor_position)?;
            Terminal::print("Goodbye.\r\n")?;
            Terminal::show_cursor()?;
            Terminal::execute()?;
            return Ok(());
        } else {
            self.draw_rows()?;
            let size = Terminal::size()?;
            Terminal::set_cursor_to(Position {
                x: size.width / 2,
                y: size.height / 3,
            })?;
            Terminal::print("TEd 1.x")?;
        }
        Terminal::set_cursor_to(self.cursor_position)?;
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows(&self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        Terminal::set_cursor_to(Position::zero())?;

        for current_row in 0..height {
            Terminal::clear(ClearType::CurrentLine)?;
            Terminal::print("~")?;

            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }
}
