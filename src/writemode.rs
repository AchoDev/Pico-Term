use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    functions::clear, jump_to_editor_point, move_down, move_left, move_right, move_up,
    ChangedLineType,
};

// handle key event for write mode
pub fn handle_key_event(
    key_event: KeyEvent,
    info_text: &mut String,
    current_line: &mut usize,
    current_char: &mut usize,
    current_scroll: &mut usize,
    editor_height: &usize,
    lines: &mut Vec<String>,
    initial: bool,
) -> io::Result<ChangedLineType> {
    let mut changed_line = ChangedLineType::None;

    match key_event.code {
        KeyCode::Down => {
            *info_text = String::new();
            if move_down(
                current_line,
                current_char,
                current_scroll,
                editor_height,
                lines,
            )? {
                // changed_line = ChangedLineType::Lines(*current_line - 1, *current_line)
                changed_line = ChangedLineType::All;
            }
        }
        KeyCode::Up => {
            *info_text = String::new();
            if move_up(
                current_line,
                current_char,
                current_scroll,
                editor_height,
                lines,
            )? {
                // changed_line = ChangedLineType::Lines(*current_line, *current_line + 1)
                changed_line = ChangedLineType::All;
            }
        }
        KeyCode::Right => {
            *info_text = String::new();
            if move_right(current_char, current_line, lines, false)? {
                // changed_line = ChangedLineType::Line(*current_line);
                changed_line = ChangedLineType::All;
            }
        }
        KeyCode::Left => {
            *info_text = String::new();

            if move_left(current_char, current_line, lines, false)? {
                // changed_line = ChangedLineType::Line(*current_line);
                changed_line = ChangedLineType::All;
            }
        }
        KeyCode::Enter => {
            *current_line += 1;
            lines.insert(*current_line, String::new());

            if *current_char < lines[*current_line - 1].len() {
                lines[*current_line] = lines[*current_line - 1]
                    [*current_char..lines[*current_line - 1].len()]
                    .to_string();

                lines[*current_line - 1] = lines[*current_line - 1][0..*current_char].to_string();
            }

            jump_to_editor_point(current_line, current_scroll, editor_height);

            *current_char = 0;

            clear()?;
            if initial {
                lines.remove(0);
                *current_line -= 1;
            } else {
                // changed_line = ChangedLineType::Lines(*current_line - 1, lines.len() - 1);
                changed_line = ChangedLineType::AllLines;
            }
        }

        KeyCode::Tab => {
            *current_char += 1;
            // changed_line = ChangedLineType::Line(*current_line);
            changed_line = ChangedLineType::All;
            if *current_char >= lines[*current_line].len() {
                lines[*current_line].push_str("    ");
            } else {
                lines[*current_line] = lines[*current_line][0..*current_char - 1].to_string()
                    + "    "
                    + &lines[*current_line][*current_char - 1..lines[*current_line].len()]
            }

            *current_char += 3;

            clear()?;
        }

        KeyCode::Char(c) => {
            *info_text = String::new();
            clear()?;
            *current_char += 1;
            // changed_line = ChangedLineType::Line(*current_line);
            changed_line = ChangedLineType::All;
            jump_to_editor_point(current_line, current_scroll, editor_height);
            if *current_char >= lines[*current_line].len() {
                lines[*current_line].push(c);
            } else {
                lines[*current_line] = lines[*current_line][0..*current_char - 1].to_string()
                    + &c.to_string()
                    + &lines[*current_line][*current_char - 1..lines[*current_line].len()]
            }
        }
        KeyCode::Backspace => {
            if *current_char == 0 {
                *info_text = String::new();
                if *current_line == 0 {
                    return Ok(ChangedLineType::None);
                }

                let copied_line = lines[*current_line].clone();
                lines[*current_line - 1].push_str(&copied_line);
                lines.remove(*current_line);

                *current_line -= 1;
                *current_char = lines[*current_line].len() - copied_line.len();
                *current_char += 1;
            } else if key_event.modifiers == KeyModifiers::CONTROL {
                // lines[*current_line] += "test";
                // cannot add this as long as crossterm doesn't fix backspace + keymod issue
            } else if *current_char >= lines[*current_line].len() {
                lines[*current_line].pop();
                // clear_all()?;
            } else {
                lines[*current_line] = lines[*current_line][0..*current_char - 1].to_string()
                    + &lines[*current_line][*current_char..lines[*current_line].len()]
            }
            *current_char -= 1;
            // changed_line = ChangedLineType::Line(*current_line);
            changed_line = ChangedLineType::AllLines;
            jump_to_editor_point(current_line, current_scroll, editor_height);
            clear()?;
            // execute!(
            //     io::stdout(),
            //     MoveTo(current_char as u16, current_line as u16)
            // )?;
        }

        _ => {}
    }

    return Ok(changed_line);
}
