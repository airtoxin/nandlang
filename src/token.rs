#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BitIo {
    pub name: String,
    pub bits: Vec<bool>,
}

impl BitIo {
    pub fn new(name: String, bits: Vec<bool>) -> BitIo {
        BitIo { name, bits }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VariableDef {
    name: String,
    gate: String,
}

impl VariableDef {
    pub fn new(name: String, gate: String) -> VariableDef {
        VariableDef { name, gate }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SignalType {
    Bit,
    Byte8,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WirePort {
    variable_name: String,
    port_name: String,
}

impl WirePort {
    pub fn new(variable_name: String, port_name: String) -> WirePort {
        WirePort {
            variable_name,
            port_name,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Input(BitIo),
    Output(BitIo),
    Wire(WirePort, WirePort),
    Variable(VariableDef),
    GateStart(String),
    GateEnd,
}
