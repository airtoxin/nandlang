use crate::lexer::Lexer;
use crate::token::{SignalType, Token};
use std::collections::HashMap;

struct Gate {
    name: String,
    input_ports: HashMap<String, SignalType>,
    output_ports: HashMap<String, SignalType>,
}

struct Scope {
    variables: HashMap<String, Token>,
    gates: Vec<Gate>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            variables: HashMap::new(),
            gates: vec![],
        }
    }

    pub fn add_gate(&mut self, gate: Gate) -> () {
        self.gates.push(gate);
    }
}

pub struct Vm {
    scope_stack: Vec<Scope>,
}

impl Vm {
    pub fn new() -> Vm {
        let mut global_scope = Scope::new();
        global_scope.add_gate(Gate {
            name: "NAND".to_string(),
            input_ports: HashMap::from([
                ("0".to_string(), SignalType::Bit),
                ("1".to_string(), SignalType::Bit),
            ]),
            output_ports: HashMap::from([("0".to_string(), SignalType::Bit)]),
        });

        Vm {
            scope_stack: vec![global_scope],
        }
    }

    pub fn run(&mut self, program: String) {
        let tokens = Lexer::new().parse(program);
    }
}
