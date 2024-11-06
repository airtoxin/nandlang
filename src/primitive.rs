pub fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nand() {
        assert_eq!(nand(false, false), true);
        assert_eq!(nand(true, false), true);
        assert_eq!(nand(false, true), true);
        assert_eq!(nand(true, true), false);
    }
}
