use crossterm::cursor::{Hide, MoveRight, MoveTo, Show};
use crossterm::event::{read, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::style::Stylize;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

use std::env;
use std::io::{self, Write};

enum Mode {
    WriteMode,
    EditMode,
    MenuMode,
}

fn move_down(
    current_line: &mut usize,
    current_char: &mut usize,
    lines: &Vec<String>,
) -> io::Result<()> {
    if *current_line == lines.len() - 1 {
        return Ok(());
    }

    *current_line += 1;

    if *current_char >= lines[*current_line].len() {
        *current_char = lines[*current_line].len()
    }
    clear_all()?;
    Ok(())
}

fn move_up(
    current_line: &mut usize,
    current_char: &mut usize,
    lines: &Vec<String>,
) -> io::Result<()> {
    if *current_line == 0 {
        return Ok(());
    }

    *current_line -= 1;

    if *current_char >= lines[*current_line].len() {
        *current_char = lines[*current_line].len()
    }
    clear_all()?;
    Ok(())
}

fn move_right(
    current_char: &mut usize,
    current_line: &usize,
    lines: &Vec<String>,
    whole_word: bool,
) -> io::Result<()> {
    if *current_char < lines[*current_line].len() {
        *current_char += 1;
    }
    clear_all()?;
    Ok(())
}

fn move_left(current_char: &mut usize) -> io::Result<()> {
    if *current_char == 0 {
        return Ok(());
    }
    *current_char -= 1;
    clear_all()?;
    Ok(())
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), Hide)?;

    let args: Vec<String> = env::args().collect();
    let mut lines: Vec<String>;
    let file_name: &str;

    println!(
        "{}",
        env::current_dir().unwrap().display().to_string() + &args[1]
    );

    if args.len() > 1 {
        let file = std::fs::read_to_string(
            env::current_dir().unwrap().display().to_string() + "/" + &args[1],
        )?;
        lines = file.lines().map(|s| s.to_string()).collect();
        file_name = &args[1];
    } else {
        lines = vec!["".to_string()];
        file_name = "new_file.txt";
    }

    clear_all()?;

    let mut current_line: usize = 0;
    let mut current_char: usize = 0;
    let mut initial = true;
    let mut current_mode = Mode::WriteMode;

    let line_count = 15;

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
                        move_down(&mut current_line, &mut current_char, &lines)?;
                        changed_line = true;
                    }
                    KeyCode::Up => {
                        move_up(&mut current_line, &mut current_char, &lines)?;
                        changed_line = true;
                    }
                    KeyCode::Right => {
                        move_right(&mut current_char, &current_line, &lines, false)?;
                        changed_line = true;
                    }
                    KeyCode::Left => {
                        move_left(&mut current_char)?;
                        changed_line = true;
                    }
                    KeyCode::Enter => {
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
                        if initial {
                            lines.remove(0);
                            initial = false;
                        }
                    }

                    KeyCode::F(2) => {
                        current_mode = match current_mode {
                            Mode::MenuMode => Mode::WriteMode,
                            _ => Mode::MenuMode,
                        };
                        changed_line = true;
                        clear_all()?;
                    }

                    KeyCode::Esc => break,
                    KeyCode::Char(c) => {
                        if key_event.modifiers == KeyModifiers::ALT && c == 'j' {
                            if matches!(current_mode, Mode::WriteMode) {
                                current_mode = Mode::EditMode;
                                clear_all()?;
                            }
                            changed_line = true;
                        } else if matches!(current_mode, Mode::EditMode) {
                            match c {
                                'i' => move_up(&mut current_line, &mut current_char, &lines)?,
                                'k' => move_down(&mut current_line, &mut current_char, &lines)?,
                                'j' => move_left(&mut current_char)?,
                                'l' => move_right(&mut current_char, &current_line, &lines, false)?,

                                'u' => {
                                    current_char = 0;
                                    clear_all()?;
                                }
                                'o' => {
                                    current_char = lines[current_line].len();
                                    clear_all()?;
                                }

                                'q' => {
                                    current_mode = Mode::WriteMode;
                                    clear_all()?;
                                }
                                _ => {}
                            }
                            changed_line = true;
                        } else {
                            current_char += 1;
                            changed_line = true;
                            if current_char >= lines[current_line].len() {
                                lines[current_line].push(c);
                            } else {
                                lines[current_line] = lines[current_line][0..current_char - 1]
                                    .to_string()
                                    + &c.to_string()
                                    + &lines[current_line]
                                        [current_char - 1..lines[current_line].len()]
                            }
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

        let mut char = match current_char == lines[current_line].len() {
            true => ' '.on_white(),
            false => lines[current_line]
                .chars()
                .nth(current_char)
                .unwrap()
                .on_white(),
        };

        if matches!(current_mode, Mode::EditMode) {
            char = char.white().on_blue();
        } else if matches!(current_mode, Mode::MenuMode) {
            char = char.white()
        }

        println!("{}", "Pico - AchoDev".dark_blue());
        print!("{}", "----| ".dark_grey());
        println!("{}", file_name.dark_grey());

        for i in 0..line_count {
            let line;
            let written_line;
            if i < lines.len() {
                line = lines[i].clone();
                written_line = true;
            } else {
                line = String::new();
                written_line = false;
            }
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

            if written_line {
                print!("{}", (i + 1).to_string().dark_grey());
            } else {
                print!("    ");
            }
            let mut spacer_length: i16 = 3;
            spacer_length -= (i + 1).to_string().len() as i16;

            while spacer_length >= 0 {
                spacer_length -= 1;
                if written_line {
                    print!(" ");
                }
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

        if matches!(current_mode, Mode::EditMode) {
            println!("\n{}", "EDIT MODE ; Exit with Q".on_yellow().white())
        } else if matches!(current_mode, Mode::MenuMode) {
            println!("\n{}", "MENU MODE".on_cyan());
            println!("\n{}", "S to save".red());
            println!("{}", "F2 to exit Menu Mode".red());
        } else {
            print!("Line: {} Char: {}", current_line + 1, current_char);
            println!("\n\n{}", "Press ALT+J to enter Edit Mode".blue());
            println!("{}", "Press F1 to enter Menu Mode".blue());
            println!("{}", "Press ESC to exit Pico".blue());
        }

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
