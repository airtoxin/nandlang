mod gate;
mod lexer;
mod primitive;
mod token;
mod vm;

#[cfg(test)]
mod tests {
    use crate::vm::Vm;

    #[test]
    fn it_works() {
        let mut vm = Vm::new();
        let program = r#"
            IN a BIT 1 0 1 0
            IN b BIT 1 1 0 0
            OUT x BIT

            VAR nand NAND

            FROM
        "#
        .trim()
        .to_string();

        vm.run(program);
    }
}
