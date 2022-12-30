use rand::seq::SliceRandom;

const TO_REPLACE: [&str; 7] = ["w", "l", "r", "n", "f", ".", "ove"];
const REPLACE_TO: [&str; 7] = ["w-w", "w", "w", "ny", "th", "!!!", "uv"];
const ENDINGS: [&str; 15] = ["OwO", "UwU", "TwT", "QwQ", "Nya!", "O_O", "O_o", "oWo", "OvO", "*~*", ":3", "=3", "<(^V^<)", "UmU", "^~^"];

fn uwuify(text: &String) -> String {
    let mut result: String = text.trim().to_string();
    for (i, v) in TO_REPLACE.iter().enumerate() {
        result = result.replace(v, REPLACE_TO[i]);
        result = result.replace(&v.to_uppercase(), &REPLACE_TO[i].to_uppercase());
    }

    let ending: &&str = ENDINGS.choose(&mut rand::thread_rng()).unwrap();
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
