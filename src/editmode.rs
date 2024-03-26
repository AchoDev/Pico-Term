match c {
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

        move_right(&mut *current_char, &*current_line, &lines, whole_word)?;
    }

    'j' => move_left(&mut *current_char, &*current_line, &mut lines, false)?,

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
    _ => {}
}