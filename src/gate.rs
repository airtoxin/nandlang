use crate::primitive::nand;

pub fn not(a: bool) -> bool {
    nand(a, a)
}

pub fn and(a: bool, b: bool) -> bool {
    not(nand(a, b))
}

pub fn and3(a: bool, b: bool, c: bool) -> bool {
    and(and(a, b), c)
}

pub fn nor(a: bool, b: bool) -> bool {
    not(nand(not(a), not(b)))
}

pub fn or(a: bool, b: bool) -> bool {
    nand(not(a), not(b))
}

pub fn or3(a: bool, b: bool, c: bool) -> bool {
    or(or(a, b), c)
}

pub fn on() -> bool {
    nand(false, false)
}

pub fn off() -> bool {
    not(on())
}

pub fn xor(a: bool, b: bool) -> bool {
    let x = nand(a, b);
    nand(nand(a, x), nand(b, x))
}

pub fn xnor(a: bool, b: bool) -> bool {
    not(xor(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not() {
        assert_eq!(not(true), false);
        assert_eq!(not(false), true);
    }

    #[test]
    fn test_and() {
        assert_eq!(and(false, false), false);
        assert_eq!(and(false, true), false);
        assert_eq!(and(true, false), false);
        assert_eq!(and(true, true), true);
    }

    #[test]
    fn test_and3() {
        assert_eq!(and3(false, false, false), false);
        assert_eq!(and3(false, false, true), false);
        assert_eq!(and3(false, true, false), false);
        assert_eq!(and3(false, true, true), false);
        assert_eq!(and3(true, false, false), false);
        assert_eq!(and3(true, false, true), false);
        assert_eq!(and3(true, true, false), false);
        assert_eq!(and3(true, true, true), true);
    }

    #[test]
    fn test_nor() {
        assert_eq!(nor(false, false), true);
        assert_eq!(nor(false, true), false);
        assert_eq!(nor(true, false), false);
        assert_eq!(nor(true, true), false);
    }

    #[test]
    fn test_or() {
        assert_eq!(or(false, false), false);
        assert_eq!(or(false, true), true);
        assert_eq!(or(true, false), true);
        assert_eq!(or(true, true), true);
    }

    #[test]
    fn test_or3() {
        assert_eq!(or3(false, false, false), false);
        assert_eq!(or3(false, false, true), true);
        assert_eq!(or3(false, true, false), true);
        assert_eq!(or3(false, true, true), true);
        assert_eq!(or3(true, false, false), true);
        assert_eq!(or3(true, false, true), true);
        assert_eq!(or3(true, true, false), true);
        assert_eq!(or3(true, true, true), true);
    }

    #[test]
    fn test_on() {
        assert_eq!(on(), true);
    }

    #[test]
    fn test_off() {
        assert_eq!(off(), false);
    }

    #[test]
    fn test_xor() {
        assert_eq!(xor(false, false), false);
        assert_eq!(xor(false, true), true);
        assert_eq!(xor(true, false), true);
        assert_eq!(xor(true, true), false);
    }

    #[test]
    fn test_xnor() {
        assert_eq!(xnor(false, false), true);
        assert_eq!(xnor(false, true), false);
        assert_eq!(xnor(true, false), false);
        assert_eq!(xnor(true, true), true);
    }
}
