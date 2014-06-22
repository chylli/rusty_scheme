use std::fmt;
use std::str;
use std::from_str;

fn main() {
    run("(+ 2 3)");
    run("(+ 21 325)");
}

fn run(s: &str) {
    println!("str: \"{}\"", s);
    let tokens = Lexer::tokenize(s);
    println!("tokens: {}", tokens);
}

enum Token {
    OpenParen,
    CloseParen,
    Identifier(String),
    Integer(int),
}

impl fmt::Show for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OpenParen => write!(f, "OpenParen"),
            CloseParen => write!(f, "CloseParen"),
            Identifier(ref v) => write!(f, "Identifier({})", v),
            Integer(ref v) => write!(f, "Integer({})", v),
        }
    }
}

struct Lexer<'a> {
  chars: str::Chars<'a>,
  current: Option<char>,
  tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    fn tokenize(s: &str) -> Vec<Token> {
        let mut lexer = Lexer { chars: s.chars(), current: None, tokens: Vec::new() };
        lexer.run();
        lexer.tokens
    }

    fn current(&self) -> Option<char> {
        self.current
    }

    fn advance(&mut self) {
        self.current = self.chars.next()
    }

    fn run(&mut self) {
        self.advance();
        loop {
            match self.current() {
                Some(c) => {
                    match c {
                        '(' => {
                            self.tokens.push(OpenParen);
                            self.advance();
                        },
                        ')' => {
                            self.tokens.push(CloseParen);
                            self.advance();
                        },
                        '+' => {
                            self.tokens.push(Identifier(str::from_char(c)));
                            self.advance();
                        },
                        '0'..'9' => {
                            let val = self.parse_number();
                            self.tokens.push(Integer(val))
                        },
                        ' ' => self.advance(),
                        _   => fail!("unexpected character: {}", c),
                    }
                },
                None => return (),
            }
        };
    }

    fn parse_number(&mut self) -> int {
        let mut s = String::new();
        loop {
            match self.current() {
                Some(c) => {
                    match c {
                        '0'..'9' => {
                            s.push_char(c);
                            self.advance();
                        },
                        _ => break
                    }
                },
                None => break
            }
        }
        from_str::from_str(s.as_slice()).unwrap()
    }
}
