pub fn is_uppercase(inp: &str) -> bool {
    for ch in inp.chars() {
        if !ch.is_uppercase() {
            return false;
        }
    }
    true
}

pub fn is_lowercase(inp: &str) -> bool {
    for ch in inp.chars() {
        if !ch.is_lowercase() {
            return false;
        }
    }
    true
}
