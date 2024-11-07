use crate::token::{Token, VariableDef, WirePoint};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer { tokens: vec![] }
    }

    pub fn parse(&mut self, program: String) -> Vec<Token> {
        for line in program.lines() {
            self.parse_line(line.split_whitespace().collect());
        }
        self.tokens.clone()
    }

    fn parse_line(&mut self, line_words: Vec<&str>) -> () {
        match line_words.as_slice() {
            &["VAR", var_name, module_name] => {
                let value = Token::Variable(VariableDef {
                    var_name: var_name.to_string(),
                    module_name: module_name.to_string(),
                });
                self.tokens.push(value);
            }
            &["FROM", src_var, src_port, "TO", dest_var, dest_port] => {
                let value = Token::Wire(
                    WirePoint {
                        var_name: src_var.to_string(),
                        port_name: src_port.to_string(),
                    },
                    WirePoint {
                        var_name: dest_var.to_string(),
                        port_name: dest_port.to_string(),
                    },
                );
                self.tokens.push(value);
            }
            &["MOD", "START", module_name] => {
                let value = Token::ModuleStart(module_name.to_string());
                self.tokens.push(value);
            }
            &["MOD", "END"] => {
                self.tokens.push(Token::ModuleEnd);
            }
            &["#", ..] => { /* ignore comment line */ }
            _ => panic!("Unknown tokens: {line_words:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::Token::{ModuleEnd, ModuleStart, Variable, Wire};
    use crate::token::{VariableDef, WirePoint};

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
                WirePoint {
                    var_name: "a".to_string(),
                    port_name: "out".to_string()
                },
                WirePoint {
                    var_name: "x".to_string(),
                    port_name: "in".to_string()
                }
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
            [Variable(VariableDef {
                var_name: "x".to_string(),
                module_name: "NAND".to_string()
            })]
        );
    }

    #[test]
    fn test_module_def() {
        let program = r#"
            MOD START TRASH
                VAR in BITIN
            MOD END
        "#
        .trim()
        .to_string();

        let mut vm = Lexer::new();
        vm.parse(program);

        assert_eq!(
            vm.tokens,
            [
                ModuleStart("TRASH".to_string()),
                Variable(VariableDef {
                    var_name: "in".to_string(),
                    module_name: "BITIN".to_string()
                }),
                ModuleEnd
            ]
        );
    }

    #[test]
    fn test_comment() {
        let program = r#"
            # MOD START TRASH
                # IN in BIT
            # MOD END
        "#
        .trim()
        .to_string();

        let mut vm = Lexer::new();
        vm.parse(program);

        assert_eq!(vm.tokens, []);
    }
}
