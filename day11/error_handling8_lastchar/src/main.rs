fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    let s="Hello!!";
    last_char_of_first_line(&s);
}
