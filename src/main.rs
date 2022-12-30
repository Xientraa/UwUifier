use std::collections::HashMap;
use rand::seq::SliceRandom;

fn uwuify(text: &String) -> String {
    let endings = ["OwO", "UwU", "TwT", "QwQ", "Nya!", "O_O", "O_o", "oWo", "OvO", "*~*", ":3", "=3", "<(^V^<)", "UmU", "^~^"];
    let replacements = HashMap::from([
        ("w", "w-w"),
        ("l", "w"),
        ("r", "w"),
        ("n", "ny"),
        ("f", "th"),
        (".", "!!!"),
        ("ove", "uv")
    ]);
    let mut result: String = text.trim().to_string();

    for (key, value) in replacements {
        result = result.replace(key, value);
        result = result.replace(&key.to_uppercase(), &value.to_uppercase());
    }

    let ending: &&str = endings.choose(&mut rand::thread_rng()).unwrap();
    return format!("{result} {ending}");
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
