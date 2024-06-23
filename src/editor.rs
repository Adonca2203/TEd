use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::stdout;

pub struct TerminalEditor {
    should_quit: bool,
    cursor_location: (u16, u16),
}

impl TerminalEditor {
    pub fn default() -> Self {
        TerminalEditor {
            should_quit: false,
            cursor_location: (0, 0),
        }
    }

    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("Goodbye.\r\n");
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        self.cursor_location = self.draw_rows();
        loop {
            if let Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) = read()?
            {
                println!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?}");
                self.cursor_location.1 += 1;
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(self.cursor_location.0, self.cursor_location.1)
                )
                .unwrap();
                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    }
                    _ => (),
                }
            }
            if self.should_quit {
                break;
            }
        }
        disable_raw_mode()?;
        Ok(())
    }

    fn draw_rows(&self) -> (u16, u16) {
        let terminal_size = crossterm::terminal::size();

        for row in 1..terminal_size.unwrap().1 {
            execute!(stdout(), crossterm::cursor::MoveTo(0, row)).unwrap();
            println!("~");
        }
        execute!(stdout(), crossterm::cursor::MoveTo(2, 0)).unwrap();
        (2, 0)
    }
}
