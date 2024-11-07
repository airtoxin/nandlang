#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VariableDef {
    name: String,
    module_name: String,
}

impl VariableDef {
    pub fn new(name: String, module_name: String) -> VariableDef {
        VariableDef { name, module_name }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WirePoint {
    var_name: String,
    port_name: String,
}

impl WirePoint {
    pub fn new(var_name: String, port_name: String) -> WirePoint {
        WirePoint {
            var_name,
            port_name,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Wire(WirePoint, WirePoint),
    Variable(VariableDef),
    GateStart(String),
    GateEnd,
    Comment(String),
}
