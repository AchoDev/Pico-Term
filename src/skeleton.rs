use std::io;

use crate::{functions::move_to, on_main_background};

pub fn draw_skeleton(width: usize, height: usize) -> io::Result<()> {
    for y in 0..height {
        let string = str::repeat(" ", width);
        print!("{}", on_main_background(&string));
    }

    Ok(())
}
