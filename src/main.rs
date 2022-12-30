use std::collections::HashMap;

fn uwuify(text: &String) -> String {
    let replacements = HashMap::from([
        ("w", "w-w"),
        ("l", "w"),
        ("r", "w"),
        ("n", "ny"),
        ("f", "th"),
        (".", "!!!"),
        ("ove", "uv")
    ]);
    let mut result: String = text.clone();

    for (key, value) in replacements {
        result = result.replace(key, value);
        result = result.replace(&key.to_uppercase(), &value.to_uppercase());
    }

    return result;
}

fn main() {
    let stdin = std::io::stdin();
    let input = &mut String::new();

    #[allow(unused_must_use)]
    loop {
        input.clear();
        stdin.read_line(input);
        println!("{}", uwuify(input));
    }
}
