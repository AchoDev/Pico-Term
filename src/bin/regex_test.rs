use regex::Regex;

fn main() {
    let re =
        Regex::new(r"\b(func|var|struct|if|elseif|else|static|return|true|false|null)\b").unwrap();
    let code = "
        func main() {
            return 'hello'
        }

        struct Person {
            var name = 'acho'
        }
    ";

    for caps in re.captures_iter(&code) {
        let mat = caps.get(0).unwrap();
        print!("One match found: {} ", mat.as_str());
        print!("Start: {} ", mat.start());
        print!("End: {}\n", mat.end());
    }
}
