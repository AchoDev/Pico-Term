use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::style::Stylize;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

use std::env;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), Hide)?;

    let args: Vec<String> = env::args().collect();
    let mut lines: Vec<String>;

    println!(
        "{}",
        env::current_dir().unwrap().display().to_string() + &args[1]
    );

    if args.len() > 1 {
        let file = std::fs::read_to_string(
            env::current_dir().unwrap().display().to_string() + "/" + &args[1],
        )?;
        lines = file.lines().map(|s| s.to_string()).collect();
    } else {
        lines = vec!["".to_string()];
    }

    // clear_all()?;

    let mut current_line: usize = 0;
    let mut current_char: usize = 0;
    let mut initial = true;

    execute!(io::stdout(), MoveTo(0, 0))?;
    loop {
        let mut changed_line = false;
        if let Ok(event) = read() {
            if let Event::Key(key_event) = event {
                if key_event.kind != KeyEventKind::Press && !initial {
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
                        if current_char == 0 {
                            continue;
                        }
                        current_char -= 1;
                        changed_line = true;
                        clear_all()?;
                    }
                    KeyCode::Enter => {
                        if (initial) {
                            initial = false;
                            continue;
                        }

                        changed_line = true;
                        current_line += 1;
                        lines.insert(current_line, String::new());

                        if current_char < lines[current_line - 1].len() {
                            lines[current_line] = lines[current_line - 1]
                                [current_char..lines[current_line - 1].len()]
                                .to_string();

                            lines[current_line - 1] =
                                lines[current_line - 1][0..current_char].to_string();
                        }

                        current_char = 0;
                        clear_all()?;
                    }

                    KeyCode::Esc => break,
                    KeyCode::Char(c) => {
                        current_char += 1;
                        changed_line = true;
                        if current_char >= lines[current_line].len() {
                            lines[current_line].push(c);
                        } else {
                            lines[current_line] = lines[current_line][0..current_char - 1]
                                .to_string()
                                + &c.to_string()
                                + &lines[current_line][current_char - 1..lines[current_line].len()]
                        }
                    }
                    KeyCode::Backspace => {
                        if current_char == 0 {
                            if current_line == 0 {
                                continue;
                            }

                            let copied_line = lines[current_line].clone();
                            lines[current_line - 1].push_str(&copied_line);
                            lines.remove(current_line);

                            current_line -= 1;
                            current_char = lines[current_line].len() - copied_line.len();
                            current_char += 1;
                        } else if current_char >= lines[current_line].len() {
                            lines[current_line].pop();
                            // clear_all()?;
                        } else {
                            lines[current_line] = lines[current_line][0..current_char - 1]
                                .to_string()
                                + &lines[current_line][current_char..lines[current_line].len()]
                        }
                        current_char -= 1;
                        changed_line = true;
                        clear_all()?;
                        // execute!(
                        //     io::stdout(),
                        //     MoveTo(current_char as u16, current_line as u16)
                        // )?;
                    }

                    _ => {}
                }
            }
        }

        execute!(io::stdout(), MoveTo(0, 0))?;
        clear_screen()?;
        if !changed_line {
            continue;
        }

        // println!("{} {}", current_char, lines[current_line].len());

        let char = match current_char == lines[current_line].len() {
            true => ' '.on_white(),
            false => lines[current_line]
                .chars()
                .nth(current_char)
                .unwrap()
                .on_white(),
        };

        println!("{}", "Pico - AchoDev".dark_blue());
        println!("{}", "----| test_file.txt".dark_grey());

        for (i, line) in lines.iter().enumerate() {
            let start: &str;
            let mut end: &str = "";

            if current_line == i {
                start = &line[0..current_char];
                if current_char < line.len() {
                    end = &line[current_char + 1..line.len()];
                }
            } else {
                start = &line
            }

            print!("{}", (i + 1).to_string().dark_grey());
            let mut spacer_length: i16 = 3;
            spacer_length -= (i + 1).to_string().len() as i16;

            while spacer_length >= 0 {
                print!(" ");
                spacer_length -= 1;
            }

            print!("{}", "| ".dark_grey());
            print!("{}", start);

            if current_line == i {
                print!("{}", char);
            }

            print!("{}", end);

            // print!("{}", &lines[current_line]);

            println!("");
        }
        println!("{}", "----|".dark_grey());
        print!("Line: {} Char: {}", current_line + 1, current_char);
        println!("\n\n{}", "Press ALT to enter Edit Mode".blue());
        println!("{}", "Press F1 to enter Menu Mode".blue());
        println!("{}", "Press ESC to exit Pico".blue());

        // println!("Debug info");
        // println!("{:?}", lines);

        io::stdout().flush()?;
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
