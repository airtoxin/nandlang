#[derive(PartialEq, Debug, Clone)]
pub struct Lexer {
    tokens: Vec<String>,
}

impl Lexer {
    pub fn new(tokens: Vec<String>) -> Lexer {
        Lexer { tokens }
    }

    pub fn lex(self: &mut Self, program: String) -> () {
        for line in program.lines() {
            for t in line.split_whitespace() {
                self.tokens.push(t.to_string());
            }
        }
    }

    pub fn next(self: &mut Self) -> Option<String> {
        self.tokens.drain(0..1).next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let mut lexer = Lexer::new(vec![]);
        lexer.lex("IN BIT 0 1 0 1".to_string());
        assert_eq!(lexer.tokens, vec!["IN", "BIT", "0", "1", "0", "1"]);

        assert_eq!(lexer.next(), Some(String::from("IN")));
        assert_eq!(lexer.tokens, vec!["BIT", "0", "1", "0", "1"]);

        assert_eq!(lexer.next(), Some(String::from("BIT")));
        assert_eq!(lexer.tokens, vec!["0", "1", "0", "1"]);

        assert_eq!(lexer.next(), Some(String::from("0")));
        assert_eq!(lexer.tokens, vec!["1", "0", "1"]);

        assert_eq!(lexer.next(), Some(String::from("1")));
        assert_eq!(lexer.tokens, vec!["0", "1"]);

        assert_eq!(lexer.next(), Some(String::from("0")));
        assert_eq!(lexer.tokens, vec!["1"]);

        assert_eq!(lexer.next(), Some(String::from("1")));
        assert_eq!(lexer.tokens, [] as [String; 0]);
    }
}
