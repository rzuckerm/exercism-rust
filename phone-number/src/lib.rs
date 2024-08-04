pub fn number(user_number: &str) -> Option<String> {
    let mut user_number: Vec<char> = user_number.chars().filter(char::is_ascii_digit).collect();
    if user_number[0] == '1' {
        user_number.remove(0);
    }

    (user_number.len() == 10 && user_number[0] >= '2' && user_number[3] >= '2')
        .then_some(user_number.iter().collect())
}
