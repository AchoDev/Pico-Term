use crossterm::style::{Color, StyledContent, Stylize};
use regex::{Match, Regex};

pub fn format(line: &str) -> Vec<StyledContent<&str>> {
    let atom_one = ColorScheme {
        keywords: (180, 105, 184),
    };
    let keywords =
        Regex::new(r"\b(func|var|struct|if|elseif|else|static|return|true|false|null)\b").unwrap();

    let mut captures: Vec<Match> = Vec::new();

    for caps in keywords.captures_iter(&line) {
        captures.push(caps.get(0).unwrap());
    }

    let mut current_index = 0;
    let mut result: Vec<StyledContent<&str>> = Vec::new();
    for i in 0..captures.len() {
        let start = captures[i].start();
        let end = captures[i].end();

        if current_index != start {
            result.push(line[current_index..start].white());
        }

        result.push(line[start..end].with(Color::Rgb {
            r: atom_one.keywords.0,
            g: atom_one.keywords.1,
            b: atom_one.keywords.2,
        }));

        current_index = end;
    }

    result.push(line[current_index..line.len()].white());

    return result;
}

struct ColorScheme {
    keywords: (u8, u8, u8),
}
