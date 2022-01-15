pub fn enlarge(input: &str) -> String {
    let mut out = String::new();

    for i in 0..=4 {
        for line in input.lines() {
            // out.push_str(&increase_line(line, i));
            increase_line(line, i, &mut out);
            out.push('\n');
        }
    }

    out
}

fn increase_line(line: &str, by: u8, into: &mut String) {
    for i in 0..=4 {
        for ch in line.chars() {
            let new = increase_char(ch, by + i);
            into.push(new);
        }
    }
}

fn increase_char(input: char, by: u8) -> char {
    let num = input as u8 - b'0' + by;
    
    if num <= 9 {
        digit_to_char(num)
    } else {
        digit_to_char(num % 9)
    }
}

// fn char_to_digit(ch: char) -> u8 {
//     ch as u8 - b'0'
// }

fn digit_to_char(digit: u8) -> char {
    (digit + b'0') as char
}


