#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VariableDef {
    pub var_name: String,
    pub module_name: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WirePoint {
    pub var_name: String,
    pub port_name: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Wire(WirePoint, WirePoint),
    Variable(VariableDef),
    ModuleStart(String),
    ModuleEnd,
}
