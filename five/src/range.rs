pub struct MyRange {
    pub start: isize,
    pub end: isize,

    at: isize,
    inclusive: bool,
}

impl MyRange {
    pub fn new(s: isize, e: isize) -> Self {
        MyRange {
            start: s,
            end: e,
            at: s,
            inclusive: false,
        }
    }

    pub fn inclusive(s: isize, e: isize) -> Self {
        MyRange {
            start: s,
            end: e,
            at: s,
            inclusive: true,
        }
    }
    

    fn forwards(&self) -> bool {
        self.end > self.start
    }

    fn stop_at(&self) -> isize {
        if self.inclusive {
            if self.forwards() {
                self.end + 1
            } else {
                self.end - 1
            }
        } else {
            self.end
        }
    }
}

impl Iterator for MyRange {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.at == self.stop_at() {
            None
        } else if self.forwards() {
            let res = Some(self.at);
            self.at += 1;
            res
        } else {
            let res = Some(self.at);
            self.at -= 1;
            res
        }
    }
}