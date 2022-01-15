use std::str::Chars;

pub enum Number {
    Number(i32),
    Expression(Box<Number>, Box<Number>)
}

impl Number {
    pub fn from_str(input: &str) -> Self {
        let mut iter = input.chars();

        while let Some(ch) = iter.next() {
            
        }
    }

    fn parse_num(input: &mut Chars) -> Option<i32> {
        let before = input.as_str();

        let mut last = 0;
        while let Some(ch) = input.next() {
            if ch.is_numeric() {
                last += 1;
            } else {
                break
            }
        }


        let num = &before[..last+1];
        num.parse().ok()
    }
}