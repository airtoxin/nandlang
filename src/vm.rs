use crate::lexer::Lexer;
use std::collections::HashMap;

struct VarIo {
    inputs: Vec<String>,
    outputs: Vec<String>,
}

pub struct Vm {
    lexer: Lexer,
    vars: HashMap<String, VarIo>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            lexer: Lexer::new(),
            vars: HashMap::new(),
        }
    }
    
    pub fn run(&mut self, program: String) {
        self.lexer.parse(program);
    }
}
