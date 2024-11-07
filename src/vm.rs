use crate::lexer::Lexer;
use std::collections::HashMap;

struct VarIo {
    inputs: Vec<String>,
    outputs: Vec<String>,
}

struct Scope {}

pub struct Vm {
    lexer: Lexer,
    vars: HashMap<String, VarIo>,
    scope_stack: Vec<Scope>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            lexer: Lexer::new(),
            vars: HashMap::from([(
                "NAND".to_string(),
                VarIo {
                    inputs: vec!["i1".to_string(), "i2".to_string()],
                    outputs: vec!["o1".to_string()],
                },
            )]),
        }
    }

    pub fn run(&mut self, program: String) {
        let tokens = self.lexer.parse(program);
    }
}
