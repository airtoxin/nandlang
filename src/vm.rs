use crate::lexer::Lexer;
use crate::module::{Module, Variable};
use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Vm {
    modules: Vec<Rc<RefCell<Module>>>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            modules: vec![Rc::new(RefCell::new(Module::new(
                "Global",
                vec![
                    Rc::new(RefCell::new(Module::new("NAND", vec![]))),
                    Rc::new(RefCell::new(Module::new("BITIN", vec![]))),
                    Rc::new(RefCell::new(Module::new("BITOUT", vec![]))),
                ],
            )))],
        }
    }

    pub fn parse(&mut self, program: impl Into<String>) {
        let tokens = Lexer::new().parse(program);
        for token in tokens {
            let current_module = self.modules.last().unwrap(); // 常にトップレベルのGlobalモジュールがあるのでunwrapして良い
            match token {
                Token::Wire(_, _) => {}
                Token::Variable(def) => current_module.borrow_mut().vars.push(Variable {
                    name: def.var_name,
                    module: self.get_module(def.module_name).unwrap(),
                }),
                Token::ModuleStart(_) => {}
                Token::ModuleEnd => {}
            }
        }
    }

    pub fn run(&self, input_signals: HashMap<impl Into<String>, bool>) -> HashMap<String, bool> {
        todo!()
    }

    fn get_module(&self, module_name: String) -> Option<Rc<RefCell<Module>>> {
        self.modules
            .iter()
            .rfind(|m| m.borrow().name == module_name)
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_run() {
        let program = r#"
            VAR in0 BITIN
            VAR in1 BITIN
            VAR out BITOUT

            VAR nand NAND

            FROM in0 o0 TO nand i0
            FROM in1 o0 TO nand i1
            FROM nand o0 TO out i0
        "#
        .trim();

        let mut vm = Vm::new();
        vm.parse(program);

        assert_eq!(
            vm.run(HashMap::from([("in0", false), ("in1", false),])),
            HashMap::from([("out".to_string(), true)])
        );

        assert_eq!(
            vm.run(HashMap::from([("in0", true), ("in1", false),])),
            HashMap::from([("out".to_string(), true)])
        );

        assert_eq!(
            vm.run(HashMap::from([("in0", false), ("in1", true),])),
            HashMap::from([("out".to_string(), true)])
        );

        assert_eq!(
            vm.run(HashMap::from([("in0", true), ("in1", true),])),
            HashMap::from([("out".to_string(), false)])
        );
    }
}
