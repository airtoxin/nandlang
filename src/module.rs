use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Wire {
    pub src: Variable,
    pub src_port: String,
    pub dest: Variable,
    pub dest_port: String,
}

#[derive(Clone)]
pub struct Variable {
    pub name: String,
    pub module: Rc<RefCell<Module>>,
}

#[derive(Clone)]
pub struct Module {
    pub name: String,
    pub modules: Vec<Rc<RefCell<Module>>>,
    pub vars: Vec<Variable>,
    pub wires: Vec<Wire>,
}

impl Module {
    pub fn new(name: impl Into<String>, modules: Vec<Rc<RefCell<Module>>>) -> Module {
        Module {
            name: name.into(),
            modules,
            vars: vec![],
            wires: vec![],
        }
    }
}
