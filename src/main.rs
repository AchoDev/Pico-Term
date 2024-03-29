use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{read, Event, KeyCode, KeyEventKind};
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
mod writemode;

use console::{Console, ConsoleAction};
use functions::*;
use menu::Menu;

enum Mode {
    WriteMode,
    EditMode,
    MenuMode,
    ConsoleMode,
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

        if matches!(current_mode, Mode::ConsoleMode) {
            if let Ok(event) = read() {
                if let Event::Key(key_event) = event {
                    if key_event.kind != KeyEventKind::Press && !initial {
                        continue;
                    }
                    changed_line = true;
                    match key_event.code {
                        KeyCode::Enter => {
                            let result = console.submit();
                            match *console.get_action() {
                                ConsoleAction::SaveAs => info_text = save_file_as(&lines, &result)?,
                            }
                        }

                        _ => console.handle_key_event(key_event),
                    }
                }
            }

            continue;
        }

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

                match key_event.code {
                    KeyCode::Esc => {
                        clear()?;
                        move_to(0, 0)?;
                        break;
                    }
                    KeyCode::F(2) => {}
                }

                if (key_event.code == KeyCode::Esc) {}

                match current_mode {
                    Mode::ConsoleMode => {}
                    Mode::MenuMode => {}
                    Mode::WriteMode => {}
                    Mode::EditMode => {}
                }

                // initial = false;
                writemode::handle_key_event(
                    key_event,
                    &mut info_text,
                    &mut current_line,
                    &mut current_char,
                )
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
