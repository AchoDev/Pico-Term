use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::style::Stylize;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

use std::io::{self, Write};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), Hide)?;

    clear_all()?;

    let mut lines: Vec<String> = Vec::from([String::new()]);
    let mut current_line: usize = 0;
    let mut current_char: usize = 0;
    let mut initial = true;

    execute!(io::stdout(), MoveTo(0, 0))?;
    loop {
        let mut changed_line = false;
        if let Ok(event) = read() {
            if let Event::Key(key_event) = event {
                if key_event.kind != KeyEventKind::Press {
                    continue;
                }
                match key_event.code {
                    KeyCode::Down => {
                        if current_line == lines.len() - 1 {
                            continue;
                        }

                        current_line += 1;
                        changed_line = true;

                        if current_char >= lines[current_line].len() {
                            current_char = lines[current_line].len()
                        }
                        clear_all()?;
                    }
                    KeyCode::Up => {
                        if current_line == 0 {
                            continue;
                        }

                        current_line -= 1;
                        changed_line = true;

                        if current_char >= lines[current_line].len() {
                            current_char = lines[current_line].len()
                        }
                        clear_all()?;
                    }
                    KeyCode::Right => {
                        if current_char < lines[current_line].len() {
                            current_char += 1;
                            changed_line = true;
                        }
                    }
                    KeyCode::Left => {
                        current_char -= 1;
                        changed_line = true;
                    }
                    KeyCode::Enter => {
                        changed_line = true;
                        current_char = 0;
                        current_line += 1;
                        lines.insert(current_line, String::new());
                        clear_all()?;
                    }

                    KeyCode::Esc => break,
                    KeyCode::Char(c) => {
                        current_char += 1;
                        changed_line = true;
                        if current_char >= lines[current_line].len() {
                            lines[current_line].push(c);
                        } else {
                            lines[current_line] = lines[current_line][0..current_char].to_string()
                                + &c.to_string()
                                + &lines[current_line][current_char + 1..lines[current_line].len()]
                        }
                    }
                    KeyCode::Backspace => {
                        if lines[current_line].len() == 0 {
                            continue;
                        }
                        changed_line = true;
                        if current_char >= lines[current_line].len() {
                            lines[current_line].pop();
                            // clear_all()?;
                        } else {
                            lines[current_line] = lines[current_line][0..current_char - 1]
                                .to_string()
                                + &lines[current_line][current_char..lines[current_line].len()]
                        }
                        current_char -= 1;

                        execute!(
                            io::stdout(),
                            MoveTo(current_char as u16, current_line as u16)
                        )?;
                        clear_all()?;
                    }

                    _ => {}
                }
            }
        }

        execute!(io::stdout(), MoveTo(0, 0))?;

        if initial {
            initial = false;
        } else if !changed_line {
            continue;
        }

        let char = match current_char == lines[current_line].len() {
            true => ' '.on_white(),
            false => lines[current_line]
                .chars()
                .nth(current_char)
                .unwrap()
                .on_white(),
        };

        for (i, line) in lines.iter().enumerate() {
            let mut start: &str = "";
            let mut end: &str = "";

            if current_line == i {
                start = &line[0..current_char];
                if current_char < line.len() {
                    end = &line[current_char + 1..line.len()];
                }
            } else {
                start = &line
            }

            clear_screen()?;
            print!("{}", start);

            if current_line == i {
                print!("{}", char);
            }

            print!("{}", end);

            // print!("{}", &lines[current_line]);

            println!("");
            io::stdout().flush()?;
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), Show)?;
    Ok(())
}

fn clear_screen() -> io::Result<()> {
    execute!(io::stdout(), Clear(ClearType::Purge))?;

    io::stdout().flush()?;

    Ok(())
}

fn clear_all() -> io::Result<()> {
    execute!(io::stdout(), Clear(ClearType::CurrentLine))?;
    execute!(io::stdout(), Clear(ClearType::FromCursorUp))?;

    io::stdout().flush()?;
    Ok(())
}
