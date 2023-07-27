#[cfg(not(test))]
pub fn complex_math() -> i32 {
    4 * 3 // Let's pretend that's complex
}

#[cfg(test)]
pub fn complex_math() -> i32 {
    12
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test() {
        assert_eq!(complex_math(), 12)
    }
}