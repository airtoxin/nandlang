use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BitIo {
    name: String,
    bits: Vec<bool>,
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
enum SignalType {
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
pub enum Value {
    Input(BitIo),
    Output(BitIo),
    Bit(bool),
    Wire(WirePort, WirePort),
    Variable(VariableDef),
    GateStart(String),
    GateEnd,
}
