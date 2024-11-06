#[derive(Debug, PartialEq)]
pub struct BitIo {
    name: String,
    bits: Vec<bool>,
}

impl BitIo {
    pub fn new(name: String, bits: Vec<bool>) -> BitIo {
        BitIo { name, bits }
    }
}

#[derive(Debug, PartialEq)]
pub struct SymbolDef {
    name: String,
    inputs: Vec<SymbolDef>,
    outputs: Vec<SymbolDef>,
}

impl SymbolDef {
    pub fn new(name: String, inputs: Vec<SymbolDef>, outputs: Vec<SymbolDef>) -> SymbolDef {
        SymbolDef { name, inputs, outputs }
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Input(BitIo),
    Output(BitIo),
    Bit(bool),
    Wire(String, String),
}