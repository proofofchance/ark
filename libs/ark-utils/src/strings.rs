pub fn truncate_string(input: &str, num_characters: usize) -> String {
    let mut truncated_string = String::from(input);

    if num_characters < truncated_string.len() {
        truncated_string.truncate(truncated_string.len() - num_characters);
    } else {
        truncated_string.clear();
    }

    truncated_string
}
