use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{
    read, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers, MouseButton, MouseEvent,
    MouseEventKind,
};
use crossterm::execute;
use crossterm::style::Stylize;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size};
use functions::{clear, move_to, purge};

use std::env;
use std::fs::read_to_string;
use std::io::{self, Write};

mod console;
mod editmode;
mod functions;
mod menu;
mod skeleton;
mod writemode;

use console::{Console, ConsoleAction};
use functions::*;
use menu::Menu;

pub enum Mode {
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
    let mut current_scroll: usize = 0;

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
    execute!(io::stdout(), EnableMouseCapture)?;

    loop {
        let mut changed_line = false;

        if matches!(current_mode, Mode::ConsoleMode) {
            if let Ok(event) = read() {
                if let Event::Key(key_event) = event {
                    if key_event.kind != KeyEventKind::Press && !initial {
                        continue;
                    }
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
            if let Event::Mouse(mouse_event) = event {
                match mouse_event.kind {
                    MouseEventKind::ScrollDown => {
                        if current_scroll + calculate_editor_height(&(term_size.1 as usize))
                            < lines.len()
                        {
                            current_scroll += 1;
                            changed_line = true;
                            clear()?;
                        }
                    }
                    MouseEventKind::ScrollUp => {
                        if current_scroll > 0 {
                            current_scroll -= 1;
                            changed_line = true;
                            clear()?;
                        }
                    }
                    // MouseEventKind::Moved => {
                    //     lines[0] = String::from("hover x: ") + &mouse_event.column.to_string();
                    //     lines[1] = String::from("hover y: ") + &mouse_event.row.to_string();
                    //     changed_line = true;
                    //     clear()?;
                    // }
                    _ => {}
                }
            }
            if let Event::Key(key_event) = event {
                if key_event.kind != KeyEventKind::Press && !initial {
                    continue;
                }

                let mut block_event = false;

                match key_event.code {
                    KeyCode::Esc => {
                        clear()?;
                        move_to(0, 0)?;
                        break;
                    }
                    KeyCode::F(2) => {}
                    KeyCode::Char('j') => {
                        if key_event.modifiers == KeyModifiers::ALT {
                            match current_mode {
                                Mode::WriteMode => current_mode = Mode::EditMode,
                                Mode::EditMode => {
                                    move_left(&mut current_char, &current_line, &mut lines, true)?
                                }
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Char('s') => {
                        if key_event.modifiers == KeyModifiers::CONTROL {
                            info_text = save_file_as(&lines, &file_name)?;
                            block_event = true;
                            changed_line = true;
                        }
                    }
                    _ => {}
                }

                if key_event.code == KeyCode::Esc {}

                if !block_event {
                    match current_mode {
                        Mode::ConsoleMode => {}
                        Mode::MenuMode => changed_line = menu.handle_key_event(key_event)?,
                        Mode::WriteMode => {
                            changed_line = writemode::handle_key_event(
                                key_event,
                                &mut info_text,
                                &mut current_line,
                                &mut current_char,
                                &mut current_scroll,
                                &calculate_editor_height(&(term_size.1 as usize)),
                                &mut lines,
                                initial,
                            )?;
                        }
                        Mode::EditMode => {
                            changed_line = editmode::handle_key_event(
                                key_event,
                                &mut current_line,
                                &mut current_char,
                                &mut current_scroll,
                                &(term_size.1 as usize),
                                &mut current_mode,
                                &mut lines,
                            )?
                        }
                    }
                }

                // initial = false;
            }
        }

        move_to(0, 0)?;
        purge()?;

        if !changed_line && !initial {
            continue;
        }

        initial = false;

        skeleton::draw_skeleton(
            &(term_size.0 as usize),
            &(term_size.1 as usize),
            &current_mode,
            &current_line,
            &current_char,
        )?;

        move_to(0, 0);
        draw_editor(
            &lines,
            &current_mode,
            &(term_size.1 as usize),
            &(term_size.0 as usize),
            &current_line,
            &current_char,
            &current_scroll,
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

fn draw_editor(
    lines: &Vec<String>,
    mode: &Mode,
    height: &usize,
    width: &usize,
    current_line: &usize,
    current_char: &usize,
    current_scroll: &usize,
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

    // println!("{}", "Pico - AchoDev".dark_blue());

    if *current_scroll > 0 {
        print!("{}", "----| ".dark_grey());
    } else {
        print!("{}", "      ".dark_grey());
        print!("{}", file_name.clone().dark_grey());
    }

    print!("\n");

    print!("\n");

    let editor_height = calculate_editor_height(height);

    let loop_count = if lines.len() > editor_height {
        editor_height + 1
    } else {
        editor_height
    };

    for i in *current_scroll..loop_count + current_scroll {
        // print!("        ");

        let line;
        let written_line;
        if i < lines.len() {
            line = lines[i as usize].clone();
            written_line = true;
        } else {
            line = String::new();
            written_line = false;
        }
        // strings before and after cursor (char variable)
        let mut start = String::new();
        let mut end = String::new();

        let line_chars: Vec<char> = line.chars().collect();

        if *current_line == i as usize {
            start = line_chars[0..*current_char].iter().collect();
            if current_char < &line.len() {
                end = line_chars[current_char + 1..line_chars.len()]
                    .iter()
                    .collect::<String>();
            }
        } else {
            start = line.clone();
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

        print!("{}", "│ ".dark_grey());
        print!("{}", start);

        if *current_line == i {
            print!("{}", char);
        }

        print!("{}", end);

        println!("");
    }
}
