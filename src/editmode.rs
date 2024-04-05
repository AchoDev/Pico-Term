use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{functions::clear, move_down, move_left, move_right, move_up, Mode};

pub fn handle_key_event(
    key_event: KeyEvent,
    current_line: &mut usize,
    current_char: &mut usize,
    current_mode: &mut Mode,
    lines: &mut Vec<String>,
) -> io::Result<bool> {
    let mut changed_line = true;
    match key_event.code {
        KeyCode::Char(c) => match c {
            'i' => match key_event.modifiers {
                KeyModifiers::ALT => {
                    if *current_line > 0 {
                        let cursor_line = lines[*current_line].clone();
                        let next_line = lines[*current_line - 1].clone();

                        lines[*current_line] = next_line;
                        lines[*current_line - 1] = cursor_line;
                        *current_line -= 1;

                        clear()?;
                    }
                }
                _ => move_up(&mut *current_line, &mut *current_char, &lines)?,
            },
            'k' => match key_event.modifiers {
                KeyModifiers::ALT => {
                    if *current_line < lines.len() - 1 {
                        let cursor_line = lines[*current_line].clone();
                        let next_line = lines[*current_line + 1].clone();

                        lines[*current_line] = next_line;
                        lines[*current_line + 1] = cursor_line;
                        *current_line += 1;

                        clear()?;
                    }
                }

                _ => move_down(&mut *current_line, &mut *current_char, &lines)?,
            },
            'l' => {
                let whole_word: bool = match key_event.modifiers {
                    KeyModifiers::ALT => true,
                    _ => false,
                };

                changed_line = true;

                move_right(&mut *current_char, &*current_line, &lines, whole_word)?;
            }

            'j' => move_left(&mut *current_char, &*current_line, lines, false)?,

            'u' => {
                *current_char = 0;
                clear()?;
            }
            'o' => {
                *current_char = lines[*current_line].len();
                clear()?;
            }

            'q' => {
                *current_mode = Mode::WriteMode;
                clear()?;
            }
            _ => changed_line = false,
        },
        _ => {}
    }

    Ok(changed_line)
}
