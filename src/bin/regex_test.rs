use regex::Regex;

fn main() {
    let keywords =
        Regex::new(r"\b(func|var|struct|if|elseif|else|static|return|true|false|null)\b").unwrap();
    let types = Regex::new(r"\bstruct\s+(\w+)").unwrap();
    let code = "
        func main() {
            return 'hello'
        }

        struct Person {
            var name = 'acho'
        }
    ";

    for caps in keywords.captures_iter(&code) {
        let mat = caps.get(0).unwrap();
        print!("Keyword: {} ", mat.as_str());
        print!("Start: {} ", mat.start());
        print!("End: {}\n", mat.end());
    }
    for caps in types.captures_iter(&code) {
        let mat = caps.get(1).unwrap();
        print!("Type: {} ", mat.as_str());
        print!("Start: {} ", mat.start());
        print!("End: {}\n", mat.end());
    }
}
