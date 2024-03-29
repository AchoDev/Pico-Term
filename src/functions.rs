use std::io::{self, Write};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};

pub fn move_to(x: u16, y: u16) -> io::Result<()> {
    execute!(io::stdout(), MoveTo(x, y))
}

pub fn purge() -> io::Result<()> {
    execute!(io::stdout(), Clear(ClearType::Purge))?;

    io::stdout().flush()?;

    Ok(())
}

pub fn clear() -> io::Result<()> {
    execute!(io::stdout(), Clear(ClearType::CurrentLine))?;
    execute!(io::stdout(), Clear(ClearType::FromCursorUp))?;

    io::stdout().flush()?;
    Ok(())
}

pub fn move_down(
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

pub fn move_up(
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

pub fn move_right(
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

pub fn move_left(
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
