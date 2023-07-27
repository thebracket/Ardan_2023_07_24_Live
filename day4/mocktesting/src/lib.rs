use mockall::*;

#[automock]
pub trait MyTrait {
    fn calculate(&self, x: u32) -> u32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_my_mock() {
        // Mockall has added a constructor to build a mocked implementation
        let mut mock = MockMyTrait::new();
        // Tell the mocked trait that `calculate` is expected to return 42
        mock.expect_calculate().return_const(42u32);
        assert_eq!(mock.calculate(12), 42);
    }

    #[test]
    fn test_my_mock_fn() {
        // Mockall has added a constructor to build a mocked implementation
        let mut mock = MockMyTrait::new();
        // Tell the mocked trait that `calculate` is expected to return 42
        mock.expect_calculate().returning(|x| x + 30);
        assert_eq!(mock.calculate(12), 42);
    }

    use mockall::predicate::*;

    #[test]
    fn test_my_mock_predicate() {
        // Mockall has added a constructor to build a mocked implementation
        let mut mock = MockMyTrait::new();
        
        mock.expect_calculate().with(eq(3)).returning(|x| x + 30);
        mock.expect_calculate().with(eq(4)).returning(|x| x + 31);
        assert_eq!(mock.calculate(3), 33);
        assert_eq!(mock.calculate(4), 35);
    }
}
