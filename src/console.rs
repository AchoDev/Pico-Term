use std::io::{self, Error, ErrorKind, Write};

use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crossterm::execute;
use crossterm::style::Stylize;
use crossterm::terminal::{size, Clear, ClearType};

pub struct Console {
    prompt: String,
    input: String,
    current_char: usize,
    submitted: bool,
}

impl Console {
    pub fn new() -> Self {
        Self {
            prompt: String::new(),
            input: String::new(),
            current_char: 0,
            submitted: false,
        }
    }

    pub fn open(&mut self, prompt: &str) -> io::Result<&str> {
        self.current_char = 0;
        self.input.clear();
        self.prompt = String::from(prompt);

        loop {
            let mut changed_line = false;

            if let Ok(event) = read() {
                if let Event::Key(key_event) = event {
                    if key_event.kind != KeyEventKind::Press {
                        continue;
                    }
                    match key_event.code {
                        KeyCode::Char(c) => {
                            self.input.push(c);
                            changed_line = true;
                        }
                        KeyCode::Esc => return Err(Error::new(ErrorKind::Other, "esc")),
                        _ => {}
                    }
                }
            };

            if changed_line {
                clear_all()?;
                execute!(io::stdout(), MoveTo(0, size().unwrap().0 - 1))?;
                self.draw();
            }
        }
    }

    pub fn draw(&mut self) {
        println!("{}", self.prompt.clone().red());
        print!("{}", "CONSOLE />".red());

        print!("{}", &self.input[0..self.current_char]);
        print!(
            "{}",
            &self.input[self.current_char..self.current_char + 1].on_red()
        );
        print!(
            "{}",
            &self.input[self.current_char + 1..self.input.len() - 1]
        );
    }

    pub fn submit(&mut self) -> String {
        let result = self.input.clone();
        self.input.clear();
        self.current_char = 0;
        return result;
    }
}

fn clear_all() -> io::Result<()> {
    execute!(io::stdout(), Clear(ClearType::CurrentLine))?;
    execute!(io::stdout(), Clear(ClearType::FromCursorUp))?;

    io::stdout().flush()?;
    Ok(())
}
