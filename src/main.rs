use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{read, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::style::Stylize;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size};
use functions::{clear, move_to, purge};

use std::env;
use std::fs::read_to_string;
use std::io::{self, Write};

mod console;
mod functions;
mod menu;

use console::Console;
use menu::Menu;

enum Mode {
    WriteMode,
    EditMode,
    MenuMode,
    ConsoleMode,
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
    clear()?;
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
    clear()?;
    Ok(())
}

fn move_right(
    current_char: &mut usize,
    current_line: &usize,
    lines: &Vec<String>,
    whole_word: bool,
) -> io::Result<()> {
    if *current_char >= lines[*current_line].len() {
        return Ok(());
    }

    *current_char += 1;

    if whole_word {
        while *current_char < lines[*current_line].len()
            && lines[*current_line].chars().nth(*current_char).unwrap() == ' '
        {
            *current_char += 1;
        }
        while *current_char < lines[*current_line].len()
            && lines[*current_line].chars().nth(*current_char).unwrap() != ' '
        {
            *current_char += 1;
        }
    }

    clear()?;
    Ok(())
}

fn move_left(
    current_char: &mut usize,
    current_line: &usize,
    lines: &mut Vec<String>,
    whole_word: bool,
) -> io::Result<()> {
    if *current_char == 0 {
        return Ok(());
    }

    *current_char -= 1;

    if whole_word {
        while *current_char > 0 && lines[*current_line].chars().nth(*current_char).unwrap() == ' ' {
            *current_char -= 1;
        }
        while *current_char > 0
            && lines[*current_line].chars().nth(*current_char - 1).unwrap() != ' '
        {
            *current_char -= 1;
        }
    }

    clear()?;
    Ok(())
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), Hide)?;

    let args: Vec<String> = env::args().collect();
    let mut lines: Vec<String>;
    let mut file_name: String;
    let file_path: String;
    let mut info_text = String::new();

    // println!(
    //     "{}",
    //     env::current_dir().unwrap().display().to_string() + &args[1]
    // );

    if args.len() > 1 {
        file_path = env::current_dir().unwrap().display().to_string() + "/" + &args[1];
        let file = read_to_string(&file_path)?;
        lines = file.lines().map(|s| s.to_string()).collect();
        file_name = args[1].clone();
    } else {
        lines = vec!["".to_string()];
        file_name = String::from("new_file.txt");
        file_path = env::current_dir().unwrap().display().to_string() + "/new_file.txt";
    }

    clear()?;

    let mut current_line: usize = 0;
    let mut current_char: usize = 0;

    let mut menu = Menu::new();
    let mut console = Console::new();

    let mut initial = true;
    let mut current_mode = Mode::WriteMode;
    let mut term_size = size().unwrap();

    let save_file_as = |lines: &Vec<String>, name: &str| -> io::Result<String> {
        clear()?;
        std::fs::write(
            env::current_dir().unwrap().display().to_string() + "/" + &name,
            lines.clone().join("\n"),
        )?;

        Ok("File saved as '".to_owned() + name + "'")
    };

    execute!(io::stdout(), MoveTo(0, 0))?;
    loop {
        let mut changed_line = false;
        if let Ok(event) = read() {
            if let Event::Resize(width, height) = event {
                term_size.0 = width;
                term_size.1 = height;
                changed_line = true;
                clear()?;
            }
            if let Event::Key(key_event) = event {
                if key_event.kind != KeyEventKind::Press && !initial {
                    continue;
                }
                // initial = false;
                match key_event.code {
                    KeyCode::Down => {
                        info_text = String::new();

                        match current_mode {
                            Mode::MenuMode => {
                                menu.move_down();
                            }
                            _ => move_down(&mut current_line, &mut current_char, &lines)?,
                        }

                        changed_line = true;
                    }
                    KeyCode::Up => {
                        info_text = String::new();

                        match current_mode {
                            Mode::MenuMode => {
                                menu.move_up();
                            }
                            _ => move_up(&mut current_line, &mut current_char, &lines)?,
                        }

                        changed_line = true;
                    }
                    KeyCode::Right => {
                        info_text = String::new();

                        match current_mode {
                            Mode::MenuMode => {
                                menu.move_right();
                                clear()?;
                            }
                            _ => move_right(&mut current_char, &current_line, &lines, false)?,
                        }

                        changed_line = true;
                    }
                    KeyCode::Left => {
                        info_text = String::new();

                        match current_mode {
                            Mode::MenuMode => {
                                menu.move_left();
                                clear()?;
                            }
                            _ => move_left(&mut current_char, &current_line, &mut lines, false)?,
                        }

                        changed_line = true;
                    }
                    KeyCode::Enter => match current_mode {
                        Mode::MenuMode => {
                            let selected_action = menu.select();
                            current_mode = Mode::WriteMode;
                            changed_line = true;

                            match selected_action {
                                "Save" => {
                                    info_text = save_file_as(&lines, &file_name)?;
                                }
                                "Save as" => {
                                    let name = console.;

                                    if name.is_ok() {
                                        let name = name.unwrap();
                                        info_text = save_file_as(&lines, name)?;
                                        file_name = name.to_owned();
                                    }
                                }

                                _ => clear()?,
                            }
                        }

                        _ => {
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
                            clear()?;
                            if initial {
                                lines.remove(0);
                                current_line -= 1;
                                initial = false;
                            }
                        }
                    },

                    KeyCode::F(2) => {
                        current_mode = match current_mode {
                            Mode::MenuMode => {
                                menu.hide();
                                Mode::WriteMode
                            }
                            _ => Mode::MenuMode,
                        };
                        changed_line = true;
                        clear()?;
                    }

                    KeyCode::Tab => {
                        current_char += 1;
                        changed_line = true;
                        if current_char >= lines[current_line].len() {
                            lines[current_line].push('\t');
                        } else {
                            lines[current_line] = lines[current_line][0..current_char - 1]
                                .to_string()
                                + "\t"
                                + &lines[current_line][current_char - 1..lines[current_line].len()]
                        }
                        clear()?;
                    }

                    KeyCode::Esc => {
                        clear()?;
                        execute!(io::stdout(), MoveTo(0, 0))?;
                        break;
                    }
                    KeyCode::Char(c) => {
                        if key_event.modifiers == KeyModifiers::ALT && c == 'j' {
                            if matches!(current_mode, Mode::WriteMode) {
                                current_mode = Mode::EditMode;
                                clear()?;
                            } else {
                                move_left(&mut current_char, &current_line, &mut lines, true)?;
                            }
                            changed_line = true;
                        } else if key_event.modifiers == KeyModifiers::CONTROL && c == 's' {
                            info_text = save_file_as(&lines, &file_name)?;
                            changed_line = true;
                        } else if matches!(current_mode, Mode::EditMode) {
                            match c {
                                'i' => match key_event.modifiers {
                                    KeyModifiers::ALT => {
                                        if current_line > 0 {
                                            let cursor_line = lines[current_line].clone();
                                            let next_line = lines[current_line - 1].clone();

                                            lines[current_line] = next_line;
                                            lines[current_line - 1] = cursor_line;
                                            current_line -= 1;

                                            clear()?;
                                        }
                                    }
                                    _ => move_up(&mut current_line, &mut current_char, &lines)?,
                                },
                                'k' => match key_event.modifiers {
                                    KeyModifiers::ALT => {
                                        if current_line < lines.len() - 1 {
                                            let cursor_line = lines[current_line].clone();
                                            let next_line = lines[current_line + 1].clone();

                                            lines[current_line] = next_line;
                                            lines[current_line + 1] = cursor_line;
                                            current_line += 1;
                                            clear()?;
                                        }
                                    }

                                    _ => move_down(&mut current_line, &mut current_char, &lines)?,
                                },
                                'l' => {
                                    let whole_word: bool = match key_event.modifiers {
                                        KeyModifiers::ALT => true,
                                        _ => false,
                                    };

                                    move_right(
                                        &mut current_char,
                                        &current_line,
                                        &lines,
                                        whole_word,
                                    )?;
                                }

                                'j' => {
                                    move_left(&mut current_char, &current_line, &mut lines, false)?
                                }

                                'u' => {
                                    current_char = 0;
                                    clear()?;
                                }
                                'o' => {
                                    current_char = lines[current_line].len();
                                    clear()?;
                                }

                                'q' => {
                                    current_mode = Mode::WriteMode;
                                    clear()?;
                                }
                                _ => {}
                            }
                            changed_line = true;
                        } else {
                            info_text = String::new();
                            clear()?;
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
                            info_text = String::new();
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
                        clear()?;
                        // execute!(
                        //     io::stdout(),
                        //     MoveTo(current_char as u16, current_line as u16)
                        // )?;
                    }

                    _ => {}
                }
            }
        }

        move_to(0, 0)?;
        purge()?;

        if !changed_line && !initial {
            continue;
        }

        initial = false;

        draw_skeleton(
            &lines,
            &current_mode,
            &(term_size.1 as usize),
            &current_line,
            &current_char,
            &info_text,
            &file_name,
        );

        move_to(0, 0)?;

        match current_mode {
            Mode::MenuMode => menu.draw()?,
            Mode::ConsoleMode => {
                move_to(0, &term_size.1 - 3)?;
                menu.draw_header()?;
                console.draw();
            }
            _ => {
                menu.draw_header()?;
            }
        }

        io::stdout().flush()?;
    }

    disable_raw_mode()?;
    execute!(io::stdout(), Show)?;
    Ok(())
}

fn draw_skeleton(
    lines: &Vec<String>,
    mode: &Mode,
    height: &usize,
    current_line: &usize,
    current_char: &usize,
    info_text: &String,
    file_name: &String,
) {
    let mut char = match *current_char == lines[*current_line].len() {
        true => ' '.on_white(),
        false => lines[*current_line]
            .chars()
            .nth(*current_char)
            .unwrap()
            .on_white(),
    };

    if matches!(*mode, Mode::EditMode) {
        char = char.white().on_dark_green();
    }

    println!("{}", "Pico - AchoDev".dark_blue());
    print!("{}", "----| ".dark_grey());
    println!("{}", file_name.clone().dark_grey());

    for i in 0..height - 9 {
        let line;
        let written_line;
        if i < lines.len() {
            line = lines[i as usize].clone();
            written_line = true;
        } else {
            line = String::new();
            written_line = false;
        }
        let start: &str;
        let mut end: &str = "";

        if *current_line == i as usize {
            start = &line[0..*current_char];
            if current_char < &line.len() {
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

        if *current_line == i {
            print!("{}", char);
        }

        print!("{}", end);

        println!("");
    }

    println!("{}", "----|".dark_grey());
    println!("\nLine: {} Char: {}", current_line + 1, current_char);
    print!("{}", info_text.clone().on_white());
    if matches!(*mode, Mode::EditMode) {
        println!("\n{}", "EDIT MODE".on_dark_green());
        print!("{}", "Switch to write mode: Q".dark_green());
    } else {
        println!("\n{}", "WRITE MODE".on_blue());
        print!("{}", "Switch to edit mode: ALT + J ".blue());
        print!("{}", "|".dark_blue());
        // println!("{}", "Press F1 to enter Menu Mode".blue());
        print!("{}", " Exit Pico: ESC".blue());
    }
}
