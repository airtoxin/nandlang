use crate::token::{BitIo, Value, VariableDef, WirePort};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Lexer {
    tokens: Vec<Value>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer { tokens: vec![] }
    }

    pub fn parse(&mut self, program: String) -> Vec<Value> {
        for line in program.lines() {
            self.parse_line(line.split_whitespace().collect());
        }
        self.tokens.clone()
    }

    fn parse_line(&mut self, line_words: Vec<&str>) -> () {
        match line_words.as_slice() {
            &["IN", name, "BIT", ..] => {
                let bits = line_words[3..]
                    .iter()
                    .map(|s| {
                        if s.to_string() == "0" {
                            false
                        } else if s.to_string() == "1" {
                            true
                        } else {
                            panic!("Unknown bit {s}")
                        }
                    })
                    .collect();
                let value = Value::Input(BitIo::new(name.to_string(), bits));
                self.tokens.push(value);
            }
            &["OUT", name, "BIT"] => {
                let value = Value::Output(BitIo::new(name.to_string(), vec![]));
                self.tokens.push(value);
            }
            &["VAR", name, gate] => {
                let value = Value::Variable(VariableDef::new(name.to_string(), gate.to_string()));
                self.tokens.push(value);
            }
            &["FROM", input_variable_name, input_port_name, "TO", output_variable_name, output_port_name] =>
            {
                let value = Value::Wire(
                    WirePort::new(input_variable_name.to_string(), input_port_name.to_string()),
                    WirePort::new(
                        output_variable_name.to_string(),
                        output_port_name.to_string(),
                    ),
                );
                self.tokens.push(value);
            }
            &["GATE", "START", gate_name] => {
                let value = Value::GateStart(gate_name.to_string());
                self.tokens.push(value);
            }
            &["GATE", "END"] => {
                self.tokens.push(Value::GateEnd);
            }
            &["#", ..] => { /* ignore comment line */ }
            &[] => {}
            _ => panic!("Unknown tokens: {line_words:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::Value::{GateEnd, GateStart, Input, Output, Variable, Wire};
    use crate::token::{BitIo, VariableDef, WirePort};

    #[test]
    fn test_in_out() {
        let program = r#"
            IN a BIT 1 0 1 0
            IN b BIT 1 1 0 0

            OUT x BIT
        "#
        .trim()
        .to_string();

        let mut vm = Lexer::new();
        vm.parse(program);

        assert_eq!(
            vm.tokens,
            [
                Input(BitIo::new("a".to_string(), vec![true, false, true, false])),
                Input(BitIo::new("b".to_string(), vec![true, true, false, false])),
                Output(BitIo::new("x".to_string(), vec![])),
            ]
        );
    }

    #[test]
    fn test_wiring() {
        let program = r#"
            FROM a out TO x in
        "#
        .trim()
        .to_string();

        let mut vm = Lexer::new();
        vm.parse(program);

        assert_eq!(
            vm.tokens,
            [Wire(
                WirePort::new("a".to_string(), "out".to_string()),
                WirePort::new("x".to_string(), "in".to_string())
            )]
        );
    }

    #[test]
    fn test_var() {
        let program = r#"
            VAR x NAND
        "#
        .trim()
        .to_string();

        let mut vm = Lexer::new();
        vm.parse(program);

        assert_eq!(
            vm.tokens,
            [Variable(VariableDef::new(
                "x".to_string(),
                "NAND".to_string()
            ))]
        );
    }

    #[test]
    fn test_gate_def() {
        let program = r#"
            GATE START TRASH
                IN in BIT
            GATE END
        "#
        .trim()
        .to_string();

        let mut vm = Lexer::new();
        vm.parse(program);

        assert_eq!(
            vm.tokens,
            [
                GateStart("TRASH".to_string()),
                Input(BitIo::new("in".to_string(), vec![])),
                GateEnd
            ]
        );
    }

    #[test]
    fn test_comment() {
        let program = r#"
            # GATE START TRASH
                # IN in BIT
            # GATE END
        "#
        .trim()
        .to_string();

        let mut vm = Lexer::new();
        vm.parse(program);

        assert_eq!(vm.tokens, []);
    }
}
