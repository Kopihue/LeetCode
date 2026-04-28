fn main() {
    let my_string = String::from("dvdf");
    let length = length_longest(my_string);
    println!("{length}");
}

fn length_longest(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let mut start = 0;
    let mut longest = &s[..1];

    for (index, letter) in s.chars().skip(1).enumerate() {
        let index = index + 1;
        let mut found = false;

        for new_letter in longest.chars() {
            if new_letter == letter {
                found = true;
                break;
            }
        }

        if !found {
            longest = &s[start..=index];
        } else {
            start = index;
        }
    }

    let length: Vec<char> = longest
        .chars()
        .collect();

    length.len() as i32
}
