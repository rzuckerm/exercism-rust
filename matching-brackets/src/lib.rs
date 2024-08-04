pub fn brackets_are_balanced(string: &str) -> bool {
    let mut expected_brackets: Vec<char> = vec![];
    for ch in string.chars() {
        match ch {
            '{' => expected_brackets.push('}'),
            '[' => expected_brackets.push(']'),
            '(' => expected_brackets.push(')'),
            '}' | ']' | ')' => {
                if expected_brackets.pop() != Some(ch) {
                    return false;
                }
            }
            _ => (),
        }
    }

    expected_brackets.is_empty()
}
