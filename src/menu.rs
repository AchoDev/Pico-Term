use std::io;

use crossterm::{
    cursor::MoveTo,
    event::{KeyCode, KeyEvent},
    execute,
    style::{Color, Stylize},
};

pub struct Menu<'a> {
    menu_item: usize,
    menu_option: usize,
    titles: [&'a str; 3],
    items: [Vec<&'a str>; 3],
}

impl<'a> Menu<'a> {
    pub fn new() -> Self {
        return Self {
            menu_item: 0,
            menu_option: 99,
            titles: ["File", "Color", "Settings"],
            items: [
                vec!["New file", "Open file", "Save", "Save as"],
                vec!["Theme1", "Theme2", "Theme3"],
                vec!["Save on unfocus", "Something"],
            ],
        };
    }

    fn adjust_item_pos(&mut self) {
        self.menu_item = 0;
    }

    pub fn move_right(&mut self) {
        self.menu_option = (self.menu_option + 1) % self.titles.len();
        self.adjust_item_pos();
    }

    pub fn move_left(&mut self) {
        if self.menu_option > 0 {
            self.menu_option -= 1;
        } else {
            self.menu_option = self.titles.len() - 1;
        }

        self.adjust_item_pos();
    }

    pub fn move_up(&mut self) {
        if self.menu_item > 0 {
            self.menu_item -= 1;
        } else {
            self.menu_item = self.items[self.menu_option].len() - 1;
        }
    }

    pub fn move_down(&mut self) {
        self.menu_item = (self.menu_item + 1) % self.items[self.menu_option].len();
    }

    pub fn reset(&mut self) {
        self.menu_option = 0;
        self.menu_item = 0;
    }

    pub fn hide(&mut self) {
        self.menu_option = 99;
    }

    pub fn draw_header(&mut self) -> io::Result<usize> {
        let mut start_pos = 0;

        print!(
            "{}",
            "Pico-Term  │  File  Edit  Settings"
                .on(Color::Rgb {
                    r: 30,
                    g: 30,
                    b: 40
                })
                .dark_grey()
        );

        start_pos += 11;

        return Ok(1);

        for i in 0..self.titles.len() {
            if i == self.menu_option {
                print!("{}", self.titles[i].on_white());
            } else {
                print!("{}", self.titles[i]);
            }

            if self.menu_option > i {
                start_pos += self.titles[i].len() + 1;
            }

            if i != self.titles.len() - 1 {
                print!("{}", " ");
            }
        }

        return Ok(start_pos);
    }

    pub fn draw_sidebar(&mut self) {
        let icons = ["E", "S", "B"];

        for i in 0..icons.len() {
            println!(
                "{}",
                "       "
                    .on(Color::Rgb {
                        r: 30,
                        g: 30,
                        b: 40
                    })
                    .dark_grey()
            );
            println!(
                "{}",
                (String::from("   ") + icons[i] + "   ")
                    .on(Color::Rgb {
                        r: 30,
                        g: 30,
                        b: 40
                    })
                    .dark_grey()
            );
            println!(
                "{}",
                "       "
                    .on(Color::Rgb {
                        r: 30,
                        g: 30,
                        b: 40
                    })
                    .dark_grey()
            );
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> io::Result<bool> {
        let mut changed_line = true;
        match key_event.code {
            KeyCode::Up => self.move_up(),
            KeyCode::Down => self.move_down(),
            KeyCode::Right => self.move_right(),
            KeyCode::Left => self.move_left(),
            _ => changed_line = false,
        }

        Ok(changed_line)
    }

    pub fn draw(&mut self) -> io::Result<()> {
        if self.menu_option > 50 {
            self.reset();
        }

        let start_pos = self.draw_header()?;

        execute!(io::stdout(), MoveTo(start_pos as u16, 1))?;

        let mut i = 0;
        for text in &self.items[self.menu_option] {
            let parsed_text;
            let spacer;

            if i == self.menu_item {
                parsed_text = text.on_blue();
                spacer = " ".on_blue();
            } else {
                parsed_text = text.on_white();
                spacer = " ".on_white();
            }

            print!("{}", parsed_text);
            for _ in 0..20 - text.len() {
                print!("{}", spacer);
            }

            i += 1;
            execute!(io::stdout(), MoveTo(start_pos as u16, i as u16 + 1))?;
        }

        Ok(())
    }

    pub fn select(&mut self) -> &str {
        let result = self.items[self.menu_option][self.menu_item];
        self.hide();
        return result;
    }
}
