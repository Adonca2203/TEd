use crossterm::event::{read, Event::Key, KeyCode::Char, Event::Resize, KeyEvent, KeyModifiers};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use std::io::Error;
use terminal_view::View;

mod terminal;
mod terminal_view;
use crossterm::terminal::ClearType;
use terminal::{Position, Terminal};

#[derive(Default)]
pub struct TerminalEditor {
    should_quit: bool,
    cursor_position: Position,
    view: View,
}

impl TerminalEditor {
    pub fn run(&mut self, file_name: &str) {
        Terminal::initialize().unwrap();
        self.view.needs_render = true;

        match self.view.load(file_name) {
            Ok(..) => {
                let result = self.repl();
                Terminal::terminate().unwrap();
                result.unwrap();
            }
            Err(err) => {
                Terminal::print(&err.to_string()).unwrap();
                Terminal::terminate().unwrap();
            }
        }
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

        if let Resize(..) = event {
            self.view.set_needs_render(true);
        }
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear(ClearType::All)?;
            Terminal::set_cursor_to(self.cursor_position)?;
            Terminal::print("Goodbye.\r\n")?;
            Terminal::show_cursor()?;
            Terminal::execute()?;
            return Ok(());
        } else {
            self.view.render()?;
        }
        Terminal::set_cursor_to(self.cursor_position)?;
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
}
