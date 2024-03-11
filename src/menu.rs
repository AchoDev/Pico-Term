use std::io;

use crossterm::{cursor::MoveTo, execute, style::Stylize};

pub struct Menu<'a> {
    menu_item: u16,
    menu_option: u16,
    titles: [&'a str, 4],
}

impl<'a> Menu<'a> {
    pub fn new() -> Self {
        Self {
            menu_item: 0,
            menu_option: 0,
            titles: ["File", "Color", "Settings"],
            [
        vec!["New file", "Open file", "Save", "Save as"],
        vec!["Theme1", "Theme2", "Theme3"],
        vec!["Save on unfocus", "Something"],
    ];
        };
    }

    pub fn move_right() {

    }
}

pub fn draw_menu(current_selected: &u16, current_item: &u16) -> io::Result<DrawResult> {
    let titles = ["File", "Color", "Settings"];

    let menu: [Vec<&str>; 3] = [
        vec!["New file", "Open file", "Save", "Save as"],
        vec!["Theme1", "Theme2", "Theme3"],
        vec!["Save on unfocus", "Something"],
    ];

    let mut width = 0;

    for i in 0..menu.len() {
        let current_list = &menu[i];
        for j in 0..current_list.len() {
            let current_len = current_list[j].len();
            if current_len > width {
                width = current_len;
            }
        }
    }

    let mut start_pos = 0;

    for i in 0..titles.len() {
        if i == *current_selected as usize {
            print!("{}", titles[i].on_white());
        } else {
            print!("{}", titles[i]);
        }

        if *current_selected > i as u16 {
            start_pos += titles[i].len() + 1;
        }

        if i != titles.len() - 1 {
            print!("{}", " ");
        }
    }

    execute!(io::stdout(), MoveTo(start_pos as u16, 1))?;

    let mut result = 0;
    let mut selected_action = "";

    let mut i = 0;
    for text in &menu[*current_selected as usize] {
        result += 1;
        let parsed_text;
        let spacer;

        if i == *current_item {
            parsed_text = text.on_blue();
            spacer = " ".on_blue();
            selected_action = text;
        } else {
            parsed_text = text.on_white();
            spacer = " ".on_white();
        }

        print!("{}", parsed_text);
        for _ in 0..20 - text.len() {
            print!("{}", spacer);
        }

        i += 1;
        execute!(io::stdout(), MoveTo(start_pos as u16, i + 1))?;
    }

    // println!("CURRENT MENU OPTION: {}", *current_item);

    return Ok(DrawResult {
        selected_index: result - 1,
        selected_action: selected_action.to_string(),
    });
}
