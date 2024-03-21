use std::io::{self, Error, ErrorKind, Write};

use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crossterm::execute;
use crossterm::style::Stylize;
use crossterm::terminal::{size, Clear, ClearType};

pub enum ConsoleAction {
    SaveAs,
}

pub struct Console {
    prompt: String,
    input: String,
    current_char: usize,
    submitted: bool,
    action: ConsoleAction,
}

impl Console {
    pub fn new() -> Self {
        Self {
            prompt: String::new(),
            input: String::new(),
            current_char: 0,
            submitted: false,
            action: ConsoleAction::SaveAs,
        }
    }

    pub fn get_action(&mut self) -> &ConsoleAction {
        return &self.action;
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

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(c) => self.input.push(c),
            _ => {}
        }
    }
}

fn clear_all() -> io::Result<()> {
    execute!(io::stdout(), Clear(ClearType::CurrentLine))?;
    execute!(io::stdout(), Clear(ClearType::FromCursorUp))?;

    io::stdout().flush()?;
    Ok(())
}
