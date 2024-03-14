use crate::functions::move_to;

fn skeleton(size: (usize, usize)) {
    move_to(0, 0);

    println!("{}", "----|".dark_grey());
    println!("\nLine: {} Char: {}", current_line + 1, current_char);
    print!("{}", info_text.clone().on_white());
    if matches!(current_mode, Mode::EditMode) {
        println!("\n{}", "EDIT MODE".on_dark_green());
        print!("{}", "Switch to write mode: Q".dark_green());
        // print!("{}", "Move cursor: I J K L".dark_green());
        // print!("{}", "Move to next word: ALT + J / K".dark_green());
        // print!("{}", "Move line up/down: ALT + I / K".dark_green());
        // print!("{}", "Move to start/end of line: U / O".dark_green());
    } else {
        println!("\n{}", "WRITE MODE".on_blue());
        print!("{}", "Switch to edit mode: ALT + J ".blue());
        print!("{}", "|".dark_blue());
        // println!("{}", "Press F1 to enter Menu Mode".blue());
        print!("{}", " Exit Pico: ESC".blue());
    }
}
