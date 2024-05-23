use std::io;

use crossterm::style::{StyledContent, Stylize};

use crate::{functions::move_to, on_main, Mode};

pub fn draw_skeleton(
    width: &usize,
    height: &usize,
    info_text: &str,
    current_mode: &Mode,
    current_line: &usize,
    current_char: &usize,
    current_scroll: &usize,
) -> io::Result<()> {
    for y in 0..height - 1 {
        let string = str::repeat(" ", *width);
        print!("{}", on_main(&string));
    }

    let mode_status: StyledContent<&str>;
    let help_text: &str;
    let mut spacer_len = width.clone();
    let line_info = generate_line_info(current_line, current_char, current_scroll);

    match current_mode {
        Mode::WriteMode => {
            let text = "WRITE MODE";
            mode_status = text.white().on_blue();
            spacer_len -= text.len();
            help_text = "ALT+J - Edit Mode"
        }
        Mode::EditMode => {
            let text = "EDIT MODE";
            mode_status = text.on_green().white();
            spacer_len -= text.len();
            help_text = "Q - Write Mode"
        }
        _ => {
            mode_status = "something went wrong".on_red();
            help_text = "??!!";
        }
    }

    // print!("{}", on_main(" "));
    print!("{}", mode_status);
    print!("{}", on_main(" "));
    print!("{}", on_main(help_text).dark_grey());
    print!("{}", on_main(" "));
    print!("{}", info_text.on_grey());

    print!(
        "{}",
        on_main(&str::repeat(
            " ",
            spacer_len - help_text.len() - line_info.len() - 1 + 4 - info_text.len() - 1
        ))
    );

    print!("{}", on_main(&line_info).dark_grey());

    Ok(())
}

fn generate_line_info(
    current_line: &usize,
    current_char: &usize,
    current_scroll: &usize,
) -> String {
    let mut line_info = String::new();

    line_info.push_str("Sc: ");
    line_info.push_str(&current_scroll.to_string());

    line_info.push_str(" Ln: ");
    line_info.push_str(&current_line.to_string());
    line_info.push_str(" Ch: ");
    line_info.push_str(&current_char.to_string());
    line_info.push_str(" │ Spaces: 4 │ Plain Text ");
    return line_info;
}
