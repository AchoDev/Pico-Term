use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{
    read, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers, MouseButton, MouseEvent,
    MouseEventKind,
};
use crossterm::execute;
use crossterm::style::{StyledContent, Stylize};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size};
use functions::{clear, move_to, purge};

use std::env;
use std::fs::read_to_string;
use std::io::{self, Write};

mod console;
mod editmode;
mod format;
mod functions;
mod menu;
mod skeleton;
mod writemode;

use console::{Console, ConsoleAction};
use format::format;
use functions::*;
use menu::Menu;

pub enum Mode {
    WriteMode,
    EditMode,
    MenuMode,
    ConsoleMode,
}

pub enum ChangedLineType {
    None,
    All,
    Skeleton,
    Line(usize),
    Lines(usize, usize),
    AllLines,
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), Hide)?;

    let args: Vec<String> = env::args().collect();
    let mut lines: Vec<String>;
    let mut cached_lines: Vec<Vec<StyledContent<String>>> = Vec::new();
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
        let mut changed_line = ChangedLineType::None;

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
                changed_line = ChangedLineType::All;
                clear()?;
            }
            if let Event::Mouse(mouse_event) = event {
                match mouse_event.kind {
                    MouseEventKind::ScrollDown => {
                        if current_scroll + calculate_editor_height(&(term_size.1 as usize))
                            < lines.len()
                        {
                            current_scroll += 2;
                            changed_line = ChangedLineType::All;
                            clear()?;
                        }
                    }
                    MouseEventKind::ScrollUp => {
                        if current_scroll == 0 {
                            changed_line = ChangedLineType::None;
                        } else {
                            current_scroll = std::cmp::max(0, current_scroll as i32 - 2) as usize;
                            changed_line = ChangedLineType::All;
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
                        for_real_clear()?;
                        move_to(0, 0)?;
                        break;
                    }
                    KeyCode::F(2) => {}
                    KeyCode::Char('j') => {
                        if key_event.modifiers == KeyModifiers::ALT {
                            match current_mode {
                                Mode::WriteMode => current_mode = Mode::EditMode,
                                Mode::EditMode => {
                                    move_left(&mut current_char, &current_line, &mut lines, true)?;
                                    ()
                                }
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Char('s') => {
                        if key_event.modifiers == KeyModifiers::CONTROL {
                            info_text = save_file_as(&lines, &file_name)?;
                            block_event = true;
                            changed_line = ChangedLineType::All;
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
        // purge()?;

        if matches!(changed_line, ChangedLineType::None) && !initial {
            continue;
        }

        if initial {
            changed_line = ChangedLineType::All;
            initial = false;
        }

        macro_rules! draw_skeleton {
            () => {
                skeleton::draw_skeleton(
                    &(term_size.0 as usize),
                    &(term_size.1 as usize),
                    &current_mode,
                    &current_line,
                    &current_char,
                    &current_scroll,
                )?;
            };
        }

        macro_rules! draw_menu {
            () => {
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
            };
        }

        match changed_line {
            ChangedLineType::All => {
                move_to(0, 0)?;
                draw_skeleton!();
                move_to(0, 0)?;
                draw_editor(
                    &lines,
                    &mut cached_lines,
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
                draw_menu!();
            }
            ChangedLineType::Skeleton => {
                move_to(0, 0)?;
                draw_skeleton!();
            }
            ChangedLineType::Line(line) => {
                move_to(0, current_line as u16 + 3)?;
                let editor_height = &(term_size.1 as usize);
                let editor_width = &(term_size.0 as usize);
                jump_to_editor_point(&mut current_line, &mut current_scroll, editor_height);
                draw_single_line(
                    &current_line,
                    &current_char,
                    &lines,
                    &mut cached_lines,
                    generate_select_char(&current_char, &current_line, &lines, &current_mode),
                    line as usize,
                    editor_width,
                );
            }
            ChangedLineType::Lines(i, j) => {
                for line in i..j + 1 {
                    move_to(0, line as u16 + 3)?;
                    let editor_height = &(term_size.1 as usize);
                    let editor_width = &(term_size.0 as usize);
                    jump_to_editor_point(&mut current_line, &mut current_scroll, editor_height);
                    draw_single_line(
                        &current_line,
                        &current_char,
                        &lines,
                        &cached_lines,
                        generate_select_char(&current_char, &current_line, &lines, &current_mode),
                        line,
                        editor_width,
                    )
                }
            }
            ChangedLineType::AllLines => {
                let editor_height = &(term_size.1 as usize);
                jump_to_editor_point(&mut current_line, &mut current_scroll, editor_height);
                draw_editor(
                    &lines,
                    &mut cached_lines,
                    &current_mode,
                    editor_height,
                    &(term_size.0 as usize),
                    &current_line,
                    &current_char,
                    &current_scroll,
                    &info_text,
                    &file_name,
                )
            }
            _ => {}
        }

        io::stdout().flush()?;
    }

    disable_raw_mode()?;
    execute!(io::stdout(), Show)?;
    Ok(())
}

fn draw_single_line(
    current_line: &usize,
    current_char: &usize,
    lines: &Vec<String>,
    cached_lines: &Vec<Vec<StyledContent<String>>>,
    char: StyledContent<char>,
    i: usize,
    width: &usize,
) {
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

    let mut line_indicator = String::new();
    let mut divider = " │ ";

    if written_line {
        line_indicator.push_str(&str::repeat(" ", 4 - (i + 1).to_string().len()));
        line_indicator.push_str(&(i + 1).to_string());
    } else {
        line_indicator.push_str("    ");
        divider = "   "
    }

    if *current_line == i {
        print!("{}", on_secondary(&line_indicator));
    } else {
        print!("{}", on_secondary(&line_indicator).dark_grey());
    }
    print!("{}", on_secondary(&divider).dark_grey());

    if written_line {
        // for value in format(&start) {
        //     print!("{}", styled_on_secondary(value));
        // }

        print!("{}", on_secondary(&start));

        if *current_line == i {
            print!("{}", char);
        }

        print!("{}", on_secondary(&end));
        // for value in format(&end) {
        //     print!("{}", styled_on_secondary(value));
        // }
    }

    print!(
        "{}",
        on_secondary(&str::repeat(" ", width - 8 - start.len() - end.len()))
    );

    if *current_line != i {
        print!("{}", on_secondary(" "))
    }
}

fn generate_select_char(
    current_char: &usize,
    current_line: &usize,
    lines: &Vec<String>,
    mode: &Mode,
) -> StyledContent<char> {
    let mut select_char = match *current_char == lines[*current_line].len() {
        true => ' '.on_white().slow_blink(),
        false => lines[*current_line]
            .chars()
            .nth(*current_char)
            .unwrap()
            .on_white()
            .slow_blink(),
    };

    if matches!(*mode, Mode::EditMode) {
        select_char = select_char.white().on_dark_green();
    }

    return select_char;
}

fn format_all_lines(lines: &Vec<String>, cached_lines: &mut Vec<Vec<StyledContent<String>>>) {
    *cached_lines = Vec::new();
    for line in lines {
        cached_lines.push(format(line));
    }
}

fn draw_editor(
    lines: &Vec<String>,
    cached_lines: &mut Vec<Vec<StyledContent<String>>>,
    mode: &Mode,
    height: &usize,
    width: &usize,
    current_line: &usize,
    current_char: &usize,
    current_scroll: &usize,
    info_text: &String,
    file_name: &String,
) {
    let select_char = generate_select_char(current_char, current_line, lines, mode);

    // print!("\n");

    print!("\n");
    print!("{}", on_secondary(" "));
    print!("{}", on_secondary(file_name));
    print!("{}", on_secondary("  "));
    print!("\n");
    print!("{}\n", on_secondary(&str::repeat(" ", *width)));

    let editor_height = calculate_editor_height(height);

    for i in *current_scroll..editor_height + current_scroll {
        // print!("        ");

        draw_single_line(
            current_line,
            current_char,
            lines,
            cached_lines,
            select_char,
            i,
            width,
        );

        println!("");
    }
}

fn draw_help_window(width: &usize, height: &usize) {
    print!("{}", on_secondary("Welcome to Pico-Term!"));
    print!("{}", "Open a new File with STR + N");
    print!("{}", "See all shortcuts up at Settings");
}
