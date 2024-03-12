pub struct Console<'a> {
    prompt: &'a str,
    input: String,
    current_char: usize,
    target: &'a mut String,
}

impl<'a> Console<'a> {
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt,
            input: String::new(),
            current_char: 0,
            target: &mut String::new(),
        }
    }

    pub fn open(&mut self, target_ref: &mut String) {
        self.target = target_ref;
        self.current_char = 0;
        self.input = String::new();
    }

    pub fn draw(&mut self) {
        println!("{}", &self.prompt);
    }
}
