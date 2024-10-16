#![cfg_attr(not(test), no_std)]

pub fn test() -> Option<u32> {
    Some(42)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = test().unwrap();
        assert_eq!(result, 42);
    }
}
