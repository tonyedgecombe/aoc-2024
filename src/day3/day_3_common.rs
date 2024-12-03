pub struct Parser<'a> {
    data: &'a[u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a[u8]) -> Parser {
        Parser { data, pos: 0 }
    }

    pub fn sum_expressions_doos(&mut self) -> i32 {
        let mut doo = true;
        let mut total = 0;

        while self.pos < self.data.len() {
            if let Some(result) = self.match_function() {
                if doo {
                    total += result;
                }
            } else if self.match_do(){
                doo = true;
            } else if self.match_dont() {
                doo = false;
            } else {
                self.pos += 1;
            }
        }

        total
    }
    pub fn sum_expressions_all(&mut self) -> i32 {
        let mut total = 0;

        while self.pos < self.data.len() {
            match self.match_function() {
                Some(result) => {
                    total += result;
                }
                None => {
                    self.pos += 1;
                }
            }
        }

        total
    }

    fn match_function(&mut self) -> Option<i32> {
        const START: &[u8] = b"mul(";

        if !self.match_bytes(START) {
            return None;
        }

        let left = self.match_number()?;

        if !self.match_byte(b',') {
            return None;
        }

        let right = self.match_number()?;

        if !self.match_byte(b')') {
            return None;
        }

        Some(left * right)
    }

    fn match_bytes(&mut self, data: &[u8]) -> bool {
        if self.data[self.pos..].starts_with(data) {
            self.pos += data.len();
            true
        } else {
            false
        }
    }

    fn match_byte(&mut self, c: u8) -> bool {
        if self.data[self.pos] == c {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn match_number(&mut self) -> Option<i32> {
        let start = self.pos;

        while self.pos < self.data.len() && self.data[self.pos].is_ascii_digit() {
            self.pos += 1;
        }

        if self.pos == start {
            return None;
        }

        let s = std::str::from_utf8(&self.data[start..self.pos]).unwrap();
        let result = s.parse::<i32>().unwrap();
        Some(result)
    }

    fn match_do(&mut self) -> bool {
        const DO: &[u8] = b"do()";
        self.match_bytes(DO)
    }

    fn match_dont(&mut self) -> bool {
        const DONT: &[u8] = b"don't()";
        self.match_bytes(DONT)
    }
}

mod test {
    use super::*;

    #[test]
    fn test_sum_expressions_all() {
        let data = b"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let mut parser = Parser::new(data);
        assert_eq!(parser.sum_expressions_all(), 161);
    }
    #[test]
    fn test_sum_expressions_doos() {
        let data = b"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let mut parser = Parser::new(data);
        assert_eq!(parser.sum_expressions_doos(), 48);
    }
}