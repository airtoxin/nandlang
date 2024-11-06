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
pub enum Value {
    Input(BitIo),
    Output(BitIo),
    Bit(bool),
}