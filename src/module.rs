use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Wire {
    src: Variable,
    dest: Variable,
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
