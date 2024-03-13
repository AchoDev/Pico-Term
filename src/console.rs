use crossterm::style::Stylize;

pub struct Console {
    prompt: String,
    input: String,
    current_char: usize,
    submitted: bool,
}

impl Console {
    pub fn new() -> Self {
        Self {
            prompt: String::new(),
            input: String::new(),
            current_char: 0,
            submitted: false,
        }
    }

    pub fn open(&mut self, prompt: &str) -> &str {
        self.current_char = 0;
        self.input.clear();
        self.prompt = String::from(prompt);
    }

    pub fn draw(&mut self) {
        println!("{}", self.prompt.clone().red());
        print!("{}", "CONSOLE />".red());

        print!("{}", &self.input[0..self.current_char]);
        print!(
            "{}",
            &self.input[self.current_char..self.current_char + 1].on_red()
        );
        print!(
            "{}",
            &self.input[self.current_char + 1..self.input.len() - 1]
        );
    }

    pub fn submit(&mut self) {
        (self.on_submit)(self.input.clone());
    }
}
