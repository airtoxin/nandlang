use crate::token::{BitIo, SymbolDef, Value};

#[derive(Debug)]
pub struct Vm {
    tokens: Vec<Value>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm { tokens: vec![] }
    }

    pub fn parse(&mut self, program: String) {
        for line in program.lines() {
            self.parse_line(line.split_whitespace().collect());
        }
    }

    fn parse_line(&mut self, line_words: Vec<&str>) -> () {
        match line_words.as_slice() {
            &["IN", "BIT", name, ..] => {
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
            &["OUT", "BIT", name] => {
                let value = Value::Output(BitIo::new(name.to_string(), vec![]));
                self.tokens.push(value);
            }
            &["FROM", src, "TO", dest] => {
                let value = Value::Wire(src.to_string(), dest.to_string());
                self.tokens.push(value);
            }
            &["VAR", name, ..] => {
                let mut inputs: Vec<String> = vec![];
                let mut outputs: Vec<String> = vec![];

                let mut io_defs = line_words[2..].iter().as_slice();
                while !io_defs.is_empty() {
                    match io_defs {
                        &["IN", name, ..] => {
                            inputs.push(name.to_string());
                            io_defs = &io_defs[2..];
                        }
                        &["OUT", name, ..] => {
                            outputs.push(name.to_string());
                            io_defs = &io_defs[2..];
                        }
                        _ => panic!("Unknown VAR IO {io_defs:?}"),
                    }
                }

                let value = Value::Symbol(SymbolDef::new(name.to_string(), inputs, outputs));
                self.tokens.push(value);
            }
            &[] => {}
            _ => panic!("Unknown tokens: {line_words:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Value::{Input, Output, Symbol, Wire};

    #[test]
    fn test_in_out() {
        let program = r#"
            IN BIT A 1 0 1 0
            IN BIT B 1 1 0 0
            
            OUT BIT X
        "#
        .trim()
        .to_string();

        let mut vm = Vm::new();
        vm.parse(program);

        assert_eq!(
            vm.tokens,
            [
                Input(BitIo::new("A".to_string(), vec![true, false, true, false])),
                Input(BitIo::new("B".to_string(), vec![true, true, false, false])),
                Output(BitIo::new("X".to_string(), vec![])),
            ]
        );
    }

    #[test]
    fn test_wiring() {
        let program = r#"
            FROM A TO X
        "#
        .trim()
        .to_string();

        let mut vm = Vm::new();
        vm.parse(program);

        assert_eq!(vm.tokens, [Wire("A".to_string(), "X".to_string())]);
    }

    #[test]
    fn test_var() {
        let program = r#"
            VAR S IN a1 IN a2 OUT b1 OUT b2
            VAR P OUT x
        "#
        .trim()
        .to_string();

        let mut vm = Vm::new();
        vm.parse(program);

        assert_eq!(
            vm.tokens,
            [
                Symbol(SymbolDef::new(
                    "S".to_string(),
                    vec!["a1".to_string(), "a2".to_string()],
                    vec!["b1".to_string(), "b2".to_string()]
                )),
                Symbol(SymbolDef::new(
                    "P".to_string(),
                    vec![],
                    vec!["x".to_string()]
                ))
            ]
        );
    }
}
