pub fn count_and_trim_start(line: &str) -> (String, usize) {
    let mut count = 0;
    let replaced = line.replace("\t", "    ");
    for (idx, char) in replaced.char_indices() {
        if char.is_whitespace() {
            count = idx + 1;
        } else {
            break;
        }
    }

    return (replaced.to_owned(), count);
}