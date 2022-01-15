pub fn bits_to_num(bits: &[bool]) -> u64 {
    let mut out = 0;

    for bit in bits {
        out = out << 1;
        if *bit {
            out |= 1;
        }
    }

    out
}

pub fn string_to_bits(s: &str) -> Vec<bool> {
    let mut res = Vec::with_capacity(s.len() * 4);

    for ch in s.chars() {
        for bit in char_to_bits(ch) {
            res.push(bit);
        }
    }

    res
}

pub fn char_to_bits(ch: char) -> [bool; 4] {
    let o = false;
    let i = true;

    match ch.to_ascii_lowercase() {
        '0' => [o, o, o, o],
        '1' => [o, o, o, i],
        '2' => [o, o, i, o],
        '3' => [o, o, i, i],
        '4' => [o, i, o, o],
        '5' => [o, i, o, i],
        '6' => [o, i, i, o],
        '7' => [o, i, i, i],
        '8' => [i, o, o, o],
        '9' => [i, o, o, i],
        'a' => [i, o, i, o],
        'b' => [i, o, i, i],
        'c' => [i, i, o, o],
        'd' => [i, i, o, i],
        'e' => [i, i, i, o],
        'f' => [i, i, i, i],
        _ => panic!("AAAAAAAA"),
    }
}


pub fn parse_literal(bits: &[bool]) -> (u64, usize) {
    let mut i = 0;
    let mut result = Vec::new();
    loop {
        let keep_reading = bits[i];
        i += 1;

        for pos in i..i+4 {
            result.push(bits[pos]);
        }

        i += 4;

        if !keep_reading {
            break;
        }
    }
    
    (bits_to_num(&result), i)
}